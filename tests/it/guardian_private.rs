//! Actual normal-library packaged caller/helper fixture assertions, never a libtest epoch override.

use quire_contract_codegen::{
    GuardianFixtureCleanup, GuardianFixtureDeathWitness, GuardianFixtureLeaseObservation,
    GuardianFixtureObservation, GuardianFixturePin, GuardianFixturePrefix,
};
use rustix::{
    event::{poll, PollFd, PollFlags},
    net::{
        recvmsg, socketpair, AddressFamily, RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags,
        ReturnFlags, SocketFlags, SocketType,
    },
    process::{pidfd_open, pidfd_send_signal, Pid, PidfdFlags, Signal},
    time::Timespec,
};
use std::{
    fs, io,
    os::fd::OwnedFd,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

struct Caller {
    child: Child,
    caller_pin: OwnedFd,
    receiver: OwnedFd,
    owned_init: Option<OwnedFd>,
    directory: PathBuf,
}
impl Drop for Caller {
    fn drop(&mut self) {
        // Independent emergency cleanup follows assertions; it never supplies their oracle.
        if let Some(init) = &self.owned_init {
            let _ = pidfd_send_signal(init, Signal::KILL);
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn caller(name: &str, mut scenario: serde_json::Value, backend: &str) -> Caller {
    let directory = crate::common::kani_run::discover_scratch(name);
    let (receiver, writer) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC | SocketFlags::NONBLOCK,
        None,
    )
    .unwrap();
    rustix::net::sockopt::set_socket_passcred(&receiver, true).unwrap();
    let marker = directory.join("backend-marker");
    let acknowledgement = directory.join("worker-ack");
    if let Some(marker_field) = scenario.get_mut("backend_marker") {
        *marker_field = serde_json::to_value(&marker).unwrap();
    }
    if let Some(ack_field) = scenario.get_mut("worker_acknowledgement") {
        *ack_field = serde_json::to_value(&acknowledgement).unwrap();
    }

    let configuration = directory.join("invocation.json");
    let arguments = vec![
        std::ffi::OsString::from("-c"),
        std::ffi::OsString::from(backend),
        marker.as_os_str().to_owned(),
        acknowledgement.as_os_str().to_owned(),
    ];
    fs::write(&configuration, serde_json::to_vec(&serde_json::json!({
        "operation":"private", "guardian_path":crate::common::guardian_path(),
        "run": { "program":std::ffi::OsString::from("/usr/bin/python3"), "arguments":arguments,
            "directory":directory, "environment":[], "report_path":directory.join("report.json"),
            "ceilings": { "memoryBytes":1_073_741_824u64, "wallClock":{"secs":8,"nanos":0} },
            "scenario":scenario }
    })).unwrap()).unwrap();
    // The packaged caller establishes its own session; do not make it a group leader before exec.
    let child = Command::new(env!("CARGO_BIN_EXE_quire-kani-caller-fixture"))
        .arg(&configuration)
        .stdin(Stdio::null())
        .stdout(Stdio::from(writer))
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // The still-unreaped owned Child prevents reused numeric PID authority even at immediate death.
    let caller_pin = pidfd_open(
        Pid::from_raw(i32::try_from(child.id()).unwrap()).unwrap(),
        PidfdFlags::empty(),
    )
    .unwrap();
    Caller {
        child,
        caller_pin,
        receiver,
        owned_init: None,
        directory,
    }
}

fn receive(caller: &mut Caller) -> (Vec<u8>, Vec<OwnedFd>) {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut bytes = Vec::new();
    let mut rights = Vec::new();
    let mut sender = None;
    loop {
        assert!(
            Instant::now() < deadline,
            "actual fixture report/EOF unavailable"
        );
        let mut ready = [PollFd::new(
            &caller.receiver,
            PollFlags::IN | PollFlags::RDHUP,
        )];
        poll(
            &mut ready,
            Some(&Timespec {
                tv_sec: 0,
                tv_nsec: 20_000_000,
            }),
        )
        .unwrap();
        let mut chunk = [0; 4096];
        let mut storage =
            [std::mem::MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(2), ScmCredentials(1))];
        let mut ancillary = RecvAncillaryBuffer::new(&mut storage);
        let received = match recvmsg(
            &caller.receiver,
            &mut [io::IoSliceMut::new(&mut chunk)],
            &mut ancillary,
            RecvFlags::CMSG_CLOEXEC | RecvFlags::DONTWAIT,
        ) {
            Ok(received) => received,
            Err(rustix::io::Errno::AGAIN | rustix::io::Errno::INTR) => continue,
            Err(error) => panic!("actual fixture receive failed: {error}"),
        };
        assert!(!received
            .flags
            .intersects(ReturnFlags::CTRUNC | ReturnFlags::TRUNC));
        for record in ancillary.drain() {
            match record {
                RecvAncillaryMessage::ScmCredentials(value) => {
                    let identity = (
                        value.pid.as_raw_pid(),
                        value.uid.as_raw(),
                        value.gid.as_raw(),
                    );
                    assert_eq!(
                        identity.0,
                        i32::try_from(caller.child.id()).unwrap(),
                        "witness sender is the retained actual caller Child"
                    );
                    assert_eq!(identity.1, rustix::process::getuid().as_raw());
                    assert_eq!(identity.2, rustix::process::getgid().as_raw());
                    if let Some(previous) = sender {
                        assert_eq!(previous, identity);
                    }
                    sender = Some(identity);
                }
                RecvAncillaryMessage::ScmRights(delivered) => {
                    for descriptor in delivered {
                        assert!(rustix::io::fcntl_getfd(&descriptor)
                            .unwrap()
                            .contains(rustix::io::FdFlags::CLOEXEC));
                        rights.push(descriptor);
                        assert!(rights.len() <= 1, "one exact owned process pin only");
                    }
                }
                _ => panic!("unexpected received control"),
            }
        }
        if received.bytes == 0 {
            break;
        }
        assert!(
            sender.is_some(),
            "every report chunk has an actual authenticated sender"
        );
        bytes.extend_from_slice(&chunk[..received.bytes]);
        assert!(bytes.len() <= 8192, "bounded sealed report");
    }
    assert!(sender.is_some(), "caller produced a complete actual report");
    (bytes, rights)
}

fn terminated(pin: &OwnedFd) -> bool {
    let mut ready = [PollFd::new(pin, PollFlags::IN)];
    poll(&mut ready, Some(&Timespec::default())).unwrap();
    assert!(!ready[0]
        .revents()
        .intersects(PollFlags::ERR | PollFlags::NVAL));
    ready[0].revents().contains(PollFlags::IN)
}
fn require_termination(pin: &OwnedFd) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !terminated(pin) {
        assert!(
            Instant::now() < deadline,
            "owned pinned process remains live before emergency cleanup"
        );
        std::thread::yield_now();
    }
}
fn observation(caller: &mut Caller) -> GuardianFixtureObservation {
    let (bytes, rights) = receive(caller);
    assert!(
        rights.is_empty(),
        "live raw record exports no process handle"
    );
    let observation = serde_json::from_slice(&bytes).unwrap();
    require_termination(&caller.caller_pin);
    assert!(
        caller.child.wait().unwrap().success(),
        "packaged fixture reports raw facts after cleanup"
    );
    observation
}
fn require_closed_lease(record: &GuardianFixtureObservation) {
    assert!(
        record.lease_closing_published,
        "LeaseClosing publication is present"
    );
    assert_eq!(
        record.coordination_failure, None,
        "coordination is positive, not a platform skip"
    );
    assert!(
        record.completed_close_ordinal.unwrap() < record.publication_ordinal.unwrap(),
        "actual completed close precedes sealed publication"
    );
    assert_eq!(
        record.lease,
        Some(GuardianFixtureLeaseObservation::ConfirmedTermination),
        "EOF confirms INIT before independent escalation"
    );
    assert_eq!(record.cleanup, GuardianFixtureCleanup::Confirmed);
    assert_eq!(
        record.monitor_reporter_present,
        Some(false),
        "actual monitor excludes reporter socket"
    );
    assert_eq!(
        record.init_reporter_present,
        Some(false),
        "actual INIT excludes reporter socket"
    );
}

/// Trace: FR-034-AC-1, FR-034-AC-3, FR-034-AC-24, FR-034-AC-27, FR-034-AC-28, FR-034-AC-29
#[test]
fn pending_real_dispatch_with_closed_lease_confirms_init_without_backend_before_escalation() {
    let mut caller = caller(
        "guardian-pending",
        serde_json::json!({"kind":"pending_dispatch","backend_marker":"assigned-before-spawn"}),
        "import pathlib,sys; pathlib.Path(sys.argv[1]).write_text('BACKEND')",
    );
    let record = observation(&mut caller);
    require_closed_lease(&record);
    assert_eq!(
        record.backend_marker_present,
        Some(false),
        "pending frame is never backend authority after observable EOF"
    );
    assert_eq!(record.worker_terminated, None);
}

/// Trace: FR-034-AC-3, FR-034-AC-8, FR-034-AC-24, FR-034-AC-27, FR-034-AC-28, FR-034-AC-29
#[test]
fn real_closed_lease_kills_acknowledged_escaped_worker_before_independent_escalation() {
    let worker = r#"import os,sys,json,time
if os.fork(): os._exit(0)
os.setsid()
if os.fork(): os._exit(0)
with open('/proc/self/stat','rb') as f: start=int(f.read().rsplit(b')',1)[1].split()[19])
with open(sys.argv[1],'w') as f: f.write('READY')
tmp=sys.argv[2]+'.tmp'
with open(tmp,'w') as f: json.dump({'namespace_pid':os.getpid(),'start':start},f)
os.rename(tmp,sys.argv[2])
while True: time.sleep(60)
"#;
    let mut caller = caller(
        "guardian-worker",
        serde_json::json!({"kind":"dispatched","backend_marker":"MARKER","worker_acknowledgement":"ACK"}),
        worker,
    );
    let record = observation(&mut caller);
    require_closed_lease(&record);
    assert_eq!(
        record.worker_terminated,
        Some(true),
        "acknowledged retained worker pidfd is dead before escalation"
    );
    assert!(record.worker_identity.is_some());
    assert!(record.worker_descriptor.is_some());
    assert_eq!(
        record.worker_reporter_present,
        Some(false),
        "actual escaped worker excludes reporter socket"
    );
    assert_eq!(record.backend_marker_present, Some(true));
}

/// Trace: FR-034-AC-1, FR-034-AC-3, FR-034-AC-23, FR-034-AC-26, FR-034-AC-27, FR-034-AC-28, FR-034-AC-29
#[test]
fn exact_shared_claimed_prefix_caller_and_group_death_transfer_actual_init_authority() {
    for prefix in [
        GuardianFixturePrefix::ClaimedGated,
        GuardianFixturePrefix::ClaimedBootstrap,
        GuardianFixturePrefix::InitReady,
    ] {
        for death in ["caller", "caller_group"] {
            let mut caller = caller(
                &format!("guardian-{prefix:?}-{death}"),
                serde_json::json!({"kind":"exact_death","prefix":prefix,"death":death}),
                "import pathlib,sys; pathlib.Path(sys.argv[1]).write_text('BACKEND')",
            );
            let (bytes, mut rights) = receive(&mut caller);
            let length = usize::try_from(u32::from_be_bytes(
                bytes.get(..4).unwrap().try_into().unwrap(),
            ))
            .unwrap();
            assert_eq!(
                bytes.len(),
                length + 4,
                "one complete bounded frame and no leaked report writer"
            );
            let witness: GuardianFixtureDeathWitness = serde_json::from_slice(&bytes[4..]).unwrap();
            assert_eq!(witness.prefix, prefix);
            assert_eq!(witness.caller.pid, caller.child.id());
            let caller_stat =
                fs::read_to_string(format!("/proc/{}/stat", caller.child.id())).unwrap();
            let caller_start: u64 = caller_stat
                .rsplit_once(')')
                .unwrap()
                .1
                .split_whitespace()
                .nth(19)
                .unwrap()
                .parse()
                .unwrap();
            assert_eq!(witness.caller.start, caller_start);
            assert!(
                witness.stdout_cloexec && witness.auxiliary_cloexec,
                "both actual flags precede every spawn"
            );
            assert_eq!(
                witness.caller.group,
                i32::try_from(caller.child.id()).unwrap()
            );
            assert_eq!(
                witness.caller.session,
                i32::try_from(caller.child.id()).unwrap()
            );
            assert_eq!(witness.monitor_reporter_present, Some(false));
            assert_eq!(witness.init_reporter_present, Some(false));
            assert_eq!(rights.len(), 1);
            let init = rights.pop().unwrap();
            let GuardianFixturePin::Init {
                identity,
                descriptor,
            } = witness.pin
            else {
                panic!("claimed prefix must transfer validated INIT");
            };
            assert_ne!(
                identity.namespace,
                fs::read_link("/proc/self/ns/pid")
                    .unwrap()
                    .to_string_lossy()
            );
            let stat = rustix::fs::fstat(&init).unwrap();
            assert_eq!(stat.st_dev, descriptor.device);
            assert_eq!(stat.st_ino, descriptor.inode);
            assert_eq!(
                witness.gate_retained,
                witness.prefix == GuardianFixturePrefix::ClaimedGated
            );
            caller.owned_init = Some(init);
            require_termination(&caller.caller_pin);
            require_termination(caller.owned_init.as_ref().unwrap());
            assert!(
                !caller.directory.join("backend-marker").exists(),
                "exact early death cannot authorize a backend"
            );
            let status = caller.child.wait().unwrap();
            assert!(!status.success());
            use std::os::unix::process::ExitStatusExt;
            assert_eq!(status.signal(), Some(9));
        }
    }
}

/// Trace: FR-034-AC-1, FR-034-AC-23, FR-034-AC-26, FR-034-AC-27, FR-034-AC-28
#[test]
fn before_monitor_exact_death_reports_no_init_and_no_invented_process_right() {
    for death in ["caller", "caller_group"] {
        let mut caller = caller(
            &format!("guardian-no-init-{death}"),
            serde_json::json!({"kind":"exact_death","prefix":"before_monitor","death":death}),
            "import pathlib,sys; pathlib.Path(sys.argv[1]).write_text('BACKEND')",
        );
        let (bytes, rights) = receive(&mut caller);
        assert!(
            rights.is_empty(),
            "NoInit carries no fabricated process authority"
        );
        let length = usize::try_from(u32::from_be_bytes(
            bytes.get(..4).unwrap().try_into().unwrap(),
        ))
        .unwrap();
        assert_eq!(
            bytes.len(),
            length + 4,
            "complete witness then actual reporter EOF"
        );
        let witness: GuardianFixtureDeathWitness = serde_json::from_slice(&bytes[4..]).unwrap();
        assert_eq!(witness.prefix, GuardianFixturePrefix::BeforeMonitor);
        assert_eq!(witness.pin, GuardianFixturePin::NoInit);
        assert_eq!(witness.monitor_reporter_present, None);
        assert_eq!(witness.init_reporter_present, None);
        assert_eq!(witness.caller.pid, caller.child.id());
        assert!(witness.stdout_cloexec && witness.auxiliary_cloexec);
        require_termination(&caller.caller_pin);
        assert!(!caller.directory.join("backend-marker").exists());
        use std::os::unix::process::ExitStatusExt;
        assert_eq!(caller.child.wait().unwrap().signal(), Some(9));
    }
}
