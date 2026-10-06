//! Operation-wide capacity exclusion installed in single-thread O before its first child.
//!
//! The policy covers native and kernel compat fcntl aliases, independent of the descriptor number
//! or a later dup/procfd reopen. It changes only F_SETPIPE_SZ to EPERM in admitted ABIs. Unknown
//! syscall architectures are killed rather than becoming an unfiltered route. This does not add
//! a backend platform: unsupported host policy targets refuse before writer exposure.

use std::io;

/// Finite CG-authored policy bound, separate from the kernel's much larger instruction ceiling.
pub(super) const INSTRUCTION_LIMIT: usize = 32;

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
))]
mod supported {
    use std::collections::BTreeMap;

    use seccompiler::{sock_filter, BpfProgram};

    use super::{io, INSTRUCTION_LIMIT};

    /// Kernel audit identity, distinct from a syscall number or a wire process identity.
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct AuditArchitecture(u32);

    struct ArchitecturePolicy {
        architecture: AuditArchitecture,
        fcntl_aliases: &'static [u32],
    }

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    enum Label {
        Syscalls(AuditArchitecture),
        Command,
        RefuseResize,
    }

    enum Node {
        Label(Label),
        LoadArchitecture,
        LoadSyscall,
        LoadCommandLow,
        IfEqual { value: u32, target: Label },
        Allow,
        RefuseResize,
        KillUnknownArchitecture,
    }

    fn architectures() -> &'static [ArchitecturePolicy] {
        // Linux UAPI audit.h and per-ABI syscall tables. Each row owns its audit identity and
        // every fcntl alias together. x32 is a number-bit variant under the x86_64 audit identity.
        #[cfg(target_arch = "x86_64")]
        return &[
            ArchitecturePolicy {
                architecture: AuditArchitecture(0xc000_003e),
                fcntl_aliases: &[72, 0x4000_0048],
            },
            ArchitecturePolicy {
                architecture: AuditArchitecture(0x4000_0003),
                fcntl_aliases: &[55, 221],
            },
        ];
        #[cfg(target_arch = "aarch64")]
        return &[
            ArchitecturePolicy {
                architecture: AuditArchitecture(0xc000_00b7),
                fcntl_aliases: &[25],
            },
            ArchitecturePolicy {
                architecture: AuditArchitecture(0x4000_0028),
                fcntl_aliases: &[55, 221],
            },
        ];
        #[cfg(target_arch = "riscv64")]
        return &[
            ArchitecturePolicy {
                architecture: AuditArchitecture(0xc000_00f3),
                fcntl_aliases: &[25],
            },
            ArchitecturePolicy {
                architecture: AuditArchitecture(0x4000_00f3),
                fcntl_aliases: &[25],
            },
        ];
    }

    fn invalid() -> io::Error {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "bounded pipe-capacity policy is invalid",
        )
    }

    fn instruction(code: u16, k: u32) -> sock_filter {
        sock_filter {
            code,
            jt: 0,
            jf: 0,
            k,
        }
    }

    pub(super) fn program() -> io::Result<BpfProgram> {
        if !cfg!(target_endian = "little") {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "pipe-capacity policy requires an audited little-endian syscall ABI",
            ));
        }
        let mut nodes = Vec::with_capacity(INSTRUCTION_LIMIT);
        nodes.push(Node::LoadArchitecture);
        for architecture in architectures() {
            nodes.push(Node::IfEqual {
                value: architecture.architecture.0,
                target: Label::Syscalls(architecture.architecture),
            });
        }
        nodes.push(Node::KillUnknownArchitecture);
        for architecture in architectures() {
            nodes.push(Node::Label(Label::Syscalls(architecture.architecture)));
            nodes.push(Node::LoadSyscall);
            for number in architecture.fcntl_aliases {
                nodes.push(Node::IfEqual {
                    value: *number,
                    target: Label::Command,
                });
            }
            nodes.push(Node::Allow);
        }
        nodes.push(Node::Label(Label::Command));
        nodes.push(Node::LoadCommandLow);
        nodes.push(Node::IfEqual {
            value: u32::try_from(nix::libc::F_SETPIPE_SZ).map_err(|_| invalid())?,
            target: Label::RefuseResize,
        });
        nodes.push(Node::Allow);
        nodes.push(Node::Label(Label::RefuseResize));
        nodes.push(Node::RefuseResize);
        if nodes.len() > INSTRUCTION_LIMIT {
            return Err(invalid());
        }
        lower(&nodes)
    }

    fn lower(nodes: &[Node]) -> io::Result<BpfProgram> {
        let mut labels = BTreeMap::new();
        let mut position = 0_usize;
        for node in nodes {
            match node {
                Node::Label(label) => {
                    if labels.insert(*label, position).is_some() {
                        return Err(invalid());
                    }
                }
                _ => position = position.checked_add(1).ok_or_else(invalid)?,
            }
        }
        if position > INSTRUCTION_LIMIT {
            return Err(invalid());
        }
        let mut program = Vec::with_capacity(INSTRUCTION_LIMIT);
        for node in nodes {
            // Linux classic BPF: LD|W|ABS=0x20; JMP|JEQ|K=0x15; RET|K=0x06.
            let instruction = match node {
                Node::Label(_) => continue,
                Node::LoadArchitecture => instruction(0x20, 4),
                Node::LoadSyscall => instruction(0x20, 0),
                // fcntl cmd is unsigned int. Compare the effective low32, including high-word
                // variations; supported ABIs share the kernel's little-endian seccomp_data.
                Node::LoadCommandLow => instruction(0x20, 24),
                Node::IfEqual { value, target } => {
                    let destination = labels.get(target).ok_or_else(invalid)?;
                    let offset = destination
                        .checked_sub(program.len().checked_add(1).ok_or_else(invalid)?)
                        .and_then(|offset| u8::try_from(offset).ok())
                        .ok_or_else(invalid)?;
                    sock_filter {
                        code: 0x15,
                        jt: offset,
                        jf: 0,
                        k: *value,
                    }
                }
                Node::Allow => instruction(0x06, nix::libc::SECCOMP_RET_ALLOW),
                Node::RefuseResize => instruction(
                    0x06,
                    nix::libc::SECCOMP_RET_ERRNO
                        | u32::try_from(nix::libc::EPERM).map_err(|_| invalid())?,
                ),
                Node::KillUnknownArchitecture => {
                    instruction(0x06, nix::libc::SECCOMP_RET_KILL_PROCESS)
                }
            };
            program.push(instruction);
        }
        Ok(program)
    }

    pub(super) fn install() -> io::Result<()> {
        let program = program()?;
        seccompiler::apply_filter(&program)
            .map_err(|error| io::Error::new(io::ErrorKind::Unsupported, error))
    }
}

/// Called by actual single-thread O only, after sizing and BEFORE any writer-bearing child.
pub(super) fn install() -> io::Result<()> {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    return supported::install();
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )))]
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "no audited pipe-capacity policy for this Linux target",
    ))
}
