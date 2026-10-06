//! This package's actual namespace-PID1 guardian. Control is its mapped stdin socket.

#![forbid(unsafe_code)]

fn main() -> std::process::ExitCode {
    quire_contract_codegen::run_kani_guardian()
}
