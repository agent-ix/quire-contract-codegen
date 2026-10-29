//! Host controls for the Kani lane and QSpec fixture preflight.

use std::{
    env, fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    time::Instant,
};

fn main() {
    let code = match dispatch(env::args().skip(1).collect()) {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    };
    std::process::exit(code);
}

fn dispatch(mut args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        return Err(
            "usage: cg_gates kani-scope | kani-gate LOCK -- CARGO ... | qspec-selftest".into(),
        );
    }
    match args.remove(0).as_str() {
        "kani-scope" => scope(&args),
        "kani-gate" => gate(&args),
        "qspec-selftest" if args.is_empty() => qspec_selftest(),
        _ => Err("invalid cg_gates command".into()),
    }
}

fn command_output(command: &mut Command) -> Result<Output, String> {
    command
        .output()
        .map_err(|error| format!("{}: {error}", command.get_program().to_string_lossy()))
}

fn git(args: &[&str]) -> Result<String, String> {
    let output = command_output(Command::new("git").args(args))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| format!("git output is not UTF-8: {error}"))
}

fn touches_kani(path: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "src/kani/",
        "src/oracle/",
        "src/routed/",
        "src/replay/",
        "src/core/",
        "src/publication/",
        "tests/state_frame_support/",
    ];
    const EXACT: &[&str] = &[
        "tests/it/skeleton_spine.rs",
        "tests/it/bounded_kani_corpus.rs",
        "tests/it/scratch_crate.rs",
        "tests/exact_scalar_support/package.rs",
        "tests/checked_package_support/base.rs",
        "tests/checked_package_support/rekey.rs",
        "Cargo.toml",
        "Cargo.lock",
    ];
    PREFIXES.iter().any(|prefix| path.starts_with(prefix))
        || EXACT.contains(&path)
        || (path.starts_with("tests/it/kani_") && path.ends_with(".rs"))
        || (path.starts_with("schemas/kani-") && path.ends_with(".schema.json"))
        || (path.starts_with("schemas/generated-rust-kani-") && path.ends_with(".schema.json"))
}

fn merge_base() -> Result<String, String> {
    Ok(git(&["merge-base", "HEAD", "origin/main"])?
        .trim()
        .to_owned())
}

fn changed_paths(base: &str) -> Result<Vec<String>, String> {
    let mut args = vec!["diff", "--no-renames", "--name-only", base];
    args.push("--");
    Ok(git(&args)?
        .lines()
        .filter(|path| touches_kani(path))
        .map(str::to_owned)
        .collect())
}

fn scope(args: &[String]) -> Result<(), String> {
    match args {
        [] => {
            let paths = changed_paths(&merge_base()?)?;
            if paths.is_empty() {
                println!("not required");
            } else {
                println!("required");
                for path in paths {
                    println!("{path}");
                }
            }
            Ok(())
        }
        _ => Err("usage: cg_gates kani-scope".into()),
    }
}

fn trusted_home() -> Result<PathBuf, String> {
    let uid = command_output(Command::new("/usr/bin/id").arg("-u"))?;
    if !uid.status.success() {
        return Err("id -u failed".into());
    }
    let uid = String::from_utf8_lossy(&uid.stdout);
    let record = command_output(Command::new("/usr/bin/getent").args(["passwd", uid.trim()]))?;
    if !record.status.success() {
        return Err("passwd home lookup failed".into());
    }
    let record = String::from_utf8_lossy(&record.stdout);
    let home = record
        .split(':')
        .nth(5)
        .ok_or("passwd home field missing")?;
    Ok(PathBuf::from(home))
}

fn gate(args: &[String]) -> Result<(), String> {
    let (held, args) = if args.first().map(String::as_str) == Some("--held") {
        (true, &args[1..])
    } else {
        (false, args)
    };
    if args.len() < 4 || args[1] != "--" {
        return Err("usage: cg_gates kani-gate LOCK -- CARGO +TOOLCHAIN test ...".into());
    }
    let lock = &args[0];
    let test_command = &args[2..];
    if !held {
        let exe = env::current_exe().map_err(|error| format!("gate executable: {error}"))?;
        let status = Command::new("flock")
            .args(["-x", lock])
            .arg(exe)
            .arg("kani-gate")
            .arg("--held")
            .args(args)
            .status()
            .map_err(|error| format!("flock: {error}"))?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("kani-gate lock/run exited {status}"));
    }
    gate_held(test_command)
}

fn gate_result(
    status: &str,
    ran: usize,
    expected: usize,
    start: Instant,
    version: &str,
    tree: &str,
) {
    println!("kani-gate: result={status} ran={ran} expected={expected} elapsed={}s kani={version} tree={tree}", start.elapsed().as_secs());
}

fn gate_command(executable: &str, args: &[String], real_cargo: bool, home: &Path) -> Command {
    let mut command = Command::new(executable);
    command.args(args);
    if real_cargo {
        command.env(
            "PATH",
            format!("{}/.cargo/bin:/usr/bin:/bin", home.display()),
        );
        command.env("HOME", home);
        command.env_remove("CARGO_HOME");
        command.env_remove("RUSTUP_HOME");
        command.env_remove("KANI_HOME");
    }
    command
}

fn tree_clean() -> Result<bool, String> {
    Ok(git(&["status", "--porcelain", "--untracked-files=all"])?.is_empty())
}

fn passed_count(output: &str) -> Option<usize> {
    let lines: Vec<_> = output
        .lines()
        .filter(|line| line.starts_with("test result: "))
        .collect();
    if lines.len() != 1 {
        return None;
    }
    let words: Vec<_> = lines[0].split_whitespace().collect();
    words.windows(2).find_map(|pair| {
        (pair[1] == "passed;")
            .then(|| pair[0].parse().ok())
            .flatten()
    })
}

fn gate_held(test_command: &[String]) -> Result<(), String> {
    let start = Instant::now();
    let home = trusted_home()?;
    let cargo = home.join(".cargo/bin/cargo");
    let real_cargo = test_command[0] == cargo.to_string_lossy();
    let launcher = if real_cargo {
        home.join(".cargo/bin/cargo-kani")
            .to_string_lossy()
            .into_owned()
    } else {
        "cargo-kani".to_owned()
    };
    if real_cargo
        && (fs::symlink_metadata(&launcher).is_err()
            || fs::symlink_metadata(&launcher)
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
            || fs::canonicalize(&cargo).ok().as_deref()
                != Some(home.join(".cargo/bin/rustup").as_path()))
    {
        println!("kani-gate: not run: launcher absent");
        return Err("launcher absent".into());
    }
    let version_output = gate_command(&launcher, &["--version".into()], real_cargo, &home).output();
    let version_output = match version_output {
        Ok(output) => output,
        Err(_) => {
            println!("kani-gate: not run: launcher absent");
            return Err("launcher absent".into());
        }
    };
    let version = String::from_utf8_lossy(&version_output.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_owned();
    if !version_output.status.success() || version.is_empty() {
        gate_result("failed", 0, 0, start, "unavailable", "clean");
        return Err("Kani version unavailable".into());
    }
    if !tree_clean()? {
        gate_result("failed", 0, 0, start, &version, "dirty");
        return Err("dirty tree".into());
    }
    let delimiter = test_command.iter().position(|arg| arg == "--");
    let Some(delimiter) = delimiter else {
        gate_result("failed", 0, 0, start, &version, "clean");
        return Err("missing libtest separator".into());
    };
    if test_command.get(delimiter + 1).map(String::as_str) != Some("--ignored")
        || test_command.get(delimiter + 2).map(String::as_str) != Some("--test-threads=1")
    {
        gate_result("failed", 0, 0, start, &version, "clean");
        return Err("real Kani lane must be ignored and serial".into());
    }
    let mut list = test_command[1..=delimiter].to_vec();
    list.extend(["--list".into(), "--ignored".into()]);
    list.extend_from_slice(&test_command[delimiter + 3..]);
    let list_output = command_output(&mut gate_command(
        &test_command[0],
        &list,
        real_cargo,
        &home,
    ));
    let list_output = match list_output {
        Ok(output) => output,
        Err(error) => {
            gate_result("failed", 0, 0, start, &version, "clean");
            return Err(error);
        }
    };
    if !list_output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&list_output.stdout));
        eprint!("{}", String::from_utf8_lossy(&list_output.stderr));
        gate_result("failed", 0, 0, start, &version, "clean");
        return Err("test list failed".into());
    }
    let listed = String::from_utf8_lossy(&list_output.stdout);
    let expected = listed
        .lines()
        .filter(|line| line.ends_with(": test"))
        .count();
    if expected == 0 {
        gate_result("failed", 0, 0, start, &version, "clean");
        return Err("test list empty".into());
    }
    let run = gate_command(&test_command[0], &test_command[1..], real_cargo, &home)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn();
    let mut run = match run {
        Ok(run) => run,
        Err(error) => {
            gate_result("failed", 0, expected, start, &version, "clean");
            return Err(format!("start real Kani lane: {error}"));
        }
    };
    let mut reader = BufReader::new(run.stdout.take().ok_or("Kani stdout unavailable")?);
    let mut line = Vec::new();
    let mut result_lines = 0;
    let mut ran = None;
    loop {
        line.clear();
        if reader
            .read_until(b'\n', &mut line)
            .map_err(|error| format!("read Kani output: {error}"))?
            == 0
        {
            break;
        }
        io::stdout()
            .write_all(&line)
            .and_then(|()| io::stdout().flush())
            .map_err(|error| format!("stream Kani output: {error}"))?;
        let text = String::from_utf8_lossy(&line);
        let text = text.trim_end_matches(['\r', '\n']);
        if text.starts_with("test result: ") {
            result_lines += 1;
            ran = passed_count(text);
        }
    }
    let run_status = run
        .wait()
        .map_err(|error| format!("wait for Kani lane: {error}"))?;
    let ran = if result_lines == 1 {
        ran.unwrap_or(0)
    } else {
        0
    };
    let tree = if tree_clean()? { "clean" } else { "dirty" };
    if run_status.success() && ran == expected && real_cargo && tree == "clean" {
        gate_result("passed", ran, expected, start, &version, tree);
        Ok(())
    } else {
        gate_result("failed", ran, expected, start, &version, tree);
        Err("real Kani evidence incomplete".into())
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn qspec_selftest() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let temp = command_output(Command::new("/usr/bin/mktemp").arg("-d"))?;
    if !temp.status.success() {
        return Err("qspec preflight selftest: mktemp failed".into());
    }
    let scratch = Scratch(PathBuf::from(String::from_utf8_lossy(&temp.stdout).trim()));
    let run_make = |args: &[&str]| -> Result<Output, String> {
        command_output(
            Command::new("make")
                .args(["--no-print-directory", "-s", "-C"])
                .arg(root)
                .args(args)
                .env_remove("MAKEFLAGS")
                .env_remove("QSPEC_DIR"),
        )
    };
    let reject = |label: &str, args: &[&str]| -> Result<(), String> {
        let output = run_make(args)?;
        let message = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if output.status.success() || !message.contains("QSPEC_DIR") || message.contains("cargo") {
            return Err(format!("qspec preflight selftest: {label} did not reject with a QSPEC_DIR diagnostic before Cargo (exit {}):\n{message}", output.status));
        }
        Ok(())
    };
    let absent = format!("QSPEC_DIR={}/absent", scratch.0.display());
    reject("unset root", &["qspec-preflight"])?;
    reject("empty root", &["qspec-preflight", "QSPEC_DIR="])?;
    reject("nonexistent root", &["qspec-preflight", &absent])?;
    reject("test before Cargo", &["test", "QSPEC_DIR="])?;
    reject("ci before Cargo", &["ci", "QSPEC_DIR="])?;
    reject("ci -k before Cargo", &["-k", "ci", "QSPEC_DIR="])?;
    let fixtures = scratch.0.join("proposals/checked-package-v2/fixtures");
    fs::create_dir_all(&fixtures).map_err(|error| format!("qspec fixture setup: {error}"))?;
    let names = [
        "positive-union-nodes",
        "positive-clause-operations",
        "positive-all-families",
    ];
    for name in names {
        fs::write(fixtures.join(format!("{name}.json")), "").map_err(|error| error.to_string())?;
    }
    let fixture_arg = format!("QSPEC_DIR={}", scratch.0.display());
    let accepted = run_make(&["qspec-preflight", &fixture_arg])?;
    if !accepted.status.success() {
        return Err("qspec preflight selftest: complete fixture set refused".into());
    }
    for name in names {
        fs::remove_file(fixtures.join(format!("{name}.json")))
            .map_err(|error| error.to_string())?;
        reject(
            &format!("missing {name}"),
            &["qspec-preflight", &fixture_arg],
        )?;
        fs::write(fixtures.join(format!("{name}.json")), "").map_err(|error| error.to_string())?;
    }
    println!("qspec preflight selftest passed");
    Ok(())
}
