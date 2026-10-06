//! IPC and privilege restriction for the single-thread, same-PID backend installer only.
//!
//! Install before positive Dispatch, then exec the exact recipe in this process. INIT, outer
//! owner and monitor must never call this entry. Policy compilation supports the audited native
//! syscall ABIs only; compat/x32 execution cannot bypass the filter. Runtime ABI killing does
//! not establish typed pre-Dispatch admission for the selected recipe. This does not establish private network/root,
//! INIT non-dumpability, descriptor exclusion or authenticated startup: those remain owner duties.
//! In particular, inherited socket/listener/io_uring and trusted transport descriptors must be
//! excluded before arbitrary backend exec. Installing this policy alone cannot establish that.

use std::io;

/// Private installation errors retain concrete causes for typed unavailable startup admission.
#[derive(Debug)]
pub(super) enum BackendPolicyError {
    UnsupportedArchitecture,
    InvalidProgram,
    NotBackend,
    ProtectionUnverified,
    Preparation(io::Error),
    Privilege(io::Error),
    #[cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )
    ))]
    Filter(seccompiler::Error),
}

impl std::fmt::Display for BackendPolicyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedArchitecture => {
                formatter.write_str("backend policy has no audited native syscall ABI")
            }
            Self::InvalidProgram => formatter.write_str("bounded backend policy is invalid"),
            Self::NotBackend => {
                formatter.write_str("backend policy cannot restrict namespace INIT")
            }
            Self::ProtectionUnverified => {
                formatter.write_str("backend privilege exclusion was not positively established")
            }
            Self::Preparation(error) => write!(formatter, "backend policy preparation: {error}"),
            Self::Privilege(error) => write!(formatter, "backend privilege restriction: {error}"),
            #[cfg(all(
                target_os = "linux",
                any(
                    target_arch = "x86_64",
                    target_arch = "aarch64",
                    target_arch = "riscv64"
                )
            ))]
            Self::Filter(error) => write!(formatter, "backend seccomp installation: {error}"),
        }
    }
}

impl std::error::Error for BackendPolicyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Preparation(error) | Self::Privilege(error) => Some(error),
            #[cfg(all(
                target_os = "linux",
                any(
                    target_arch = "x86_64",
                    target_arch = "aarch64",
                    target_arch = "riscv64"
                )
            ))]
            Self::Filter(error) => Some(error),
            Self::UnsupportedArchitecture
            | Self::InvalidProgram
            | Self::NotBackend
            | Self::ProtectionUnverified => None,
        }
    }
}

#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    )
))]
mod supported {
    use rustix::thread::{
        capabilities, capability_is_in_ambient_set, capability_is_in_bounding_set,
        clear_ambient_capability_set, no_new_privs, remove_capability_from_bounding_set,
        set_capabilities, set_no_new_privs, CapabilitySet, CapabilitySets,
    };
    use seccompiler::{sock_filter, BpfProgram};

    use super::BackendPolicyError;

    // Finite CG-authored working bound, below the kernel instruction limit. Both vectors are
    // allocated fallibly before irreversible privilege/filter changes and charged by owned RSS.
    const INSTRUCTION_LIMIT: usize = 64;

    #[derive(Clone, Copy)]
    enum Label {
        Native,
        SocketFamily,
        PairFamily,
        PairType,
        NewUserFlags,
        NamespaceType,
        Allow,
        Refuse,
        KillAbi,
        #[cfg(target_arch = "x86_64")]
        LegacyX32,
    }

    impl Label {
        fn slot(self) -> usize {
            match self {
                Self::Native => 0,
                Self::SocketFamily => 1,
                Self::PairFamily => 2,
                Self::PairType => 3,
                Self::NewUserFlags => 4,
                Self::NamespaceType => 5,
                Self::Allow => 6,
                Self::Refuse => 7,
                Self::KillAbi => 8,
                #[cfg(target_arch = "x86_64")]
                Self::LegacyX32 => 9,
            }
        }
    }

    enum Node {
        Label(Label),
        Load(u32),
        And(u32),
        Equal(u32, Label),
        AnyBit(u32, Label),
        #[cfg(target_arch = "x86_64")]
        GreaterOrEqual(u32, Label),
        Allow,
        Refuse,
        KillAbi,
    }

    fn native_architecture() -> u32 {
        // Linux UAPI audit.h: native little-endian audit identities, not syscall numbers.
        #[cfg(target_arch = "x86_64")]
        return 0xc000_003e;
        #[cfg(target_arch = "aarch64")]
        return 0xc000_00b7;
        #[cfg(target_arch = "riscv64")]
        return 0xc000_00f3;
    }

    fn number(value: nix::libc::c_long) -> Result<u32, BackendPolicyError> {
        u32::try_from(value).map_err(|_| BackendPolicyError::InvalidProgram)
    }

    fn argument(value: i32) -> Result<u32, BackendPolicyError> {
        u32::try_from(value).map_err(|_| BackendPolicyError::InvalidProgram)
    }

    fn program() -> Result<BpfProgram, BackendPolicyError> {
        if !cfg!(target_endian = "little") {
            return Err(BackendPolicyError::UnsupportedArchitecture);
        }
        let mut nodes = Vec::new();
        nodes
            .try_reserve_exact(INSTRUCTION_LIMIT)
            .map_err(|error| BackendPolicyError::Preparation(std::io::Error::other(error)))?;
        // seccomp_data: nr at 0, arch at 4, args[0] at 16, args[1] at 24. Native socket
        // family/type and namespace flags use their effective low32 on these Linux ABIs.
        nodes.extend([
            Node::Load(4),
            Node::Equal(native_architecture(), Label::Native),
            Node::KillAbi,
            Node::Label(Label::Native),
            Node::Load(0),
        ]);
        // x32 uses the x86_64 audit identity but a different syscall-number bit and pointer ABI.
        // Reject it before native-number dispatch; i386/ARM/rv32 socketcall/aliases already fail
        // the audit-architecture gate. Runtime ABI rejection is not proof of startup admission.
        #[cfg(target_arch = "x86_64")]
        nodes.extend([
            Node::AnyBit(0x4000_0000, Label::KillAbi),
            // The reserved x32-only 512..547 numbers also cannot become a bitless alias on
            // kernels with historical x32 dispatch. Later native numbers remain unaffected.
            Node::GreaterOrEqual(512, Label::LegacyX32),
        ]);
        // io_uring can create sockets and perform IPC outside socket syscall checks. Its pointer
        // payload cannot be inspected by cBPF. clone3 likewise hides CLONE_NEWUSER in memory.
        // ptrace/pidfd_getfd must not acquire trusted INIT endpoints (FR-034 owner protection).
        for syscall in [
            nix::libc::SYS_io_uring_setup,
            nix::libc::SYS_io_uring_enter,
            nix::libc::SYS_io_uring_register,
            nix::libc::SYS_clone3,
            nix::libc::SYS_ptrace,
            nix::libc::SYS_pidfd_getfd,
        ] {
            nodes.push(Node::Equal(number(syscall)?, Label::Refuse));
        }
        for (syscall, label) in [
            (nix::libc::SYS_socket, Label::SocketFamily),
            (nix::libc::SYS_socketpair, Label::PairFamily),
            (nix::libc::SYS_clone, Label::NewUserFlags),
            (nix::libc::SYS_unshare, Label::NewUserFlags),
            (nix::libc::SYS_setns, Label::NamespaceType),
        ] {
            nodes.push(Node::Equal(number(syscall)?, label));
        }
        let unix = argument(nix::libc::AF_UNIX)?;
        let new_user = argument(nix::libc::CLONE_NEWUSER)?;
        let pair_flags = argument(nix::libc::SOCK_CLOEXEC | nix::libc::SOCK_NONBLOCK)?;
        nodes.extend([
            Node::Allow,
            Node::Label(Label::SocketFamily),
            Node::Load(16),
            Node::Equal(unix, Label::Refuse),
            Node::Allow,
            Node::Label(Label::PairFamily),
            Node::Load(16),
            Node::Equal(unix, Label::PairType),
            Node::Allow,
            Node::Label(Label::PairType),
            Node::Load(24),
            Node::And(!pair_flags),
            Node::Equal(argument(nix::libc::SOCK_STREAM)?, Label::Allow),
            Node::Refuse,
            Node::Label(Label::NewUserFlags),
            Node::Load(16),
            Node::AnyBit(new_user, Label::Refuse),
            Node::Allow,
            Node::Label(Label::NamespaceType),
            Node::Load(24),
            // A zero nstype follows the FD's namespace type; cBPF cannot inspect that FD.
            // Explicit non-user namespace types are not blanket-denied by this policy.
            Node::Equal(0, Label::Refuse),
            Node::AnyBit(new_user, Label::Refuse),
            Node::Allow,
        ]);
        #[cfg(target_arch = "x86_64")]
        nodes.extend([
            Node::Label(Label::LegacyX32),
            Node::GreaterOrEqual(548, Label::Allow),
            Node::KillAbi,
        ]);
        nodes.extend([
            Node::Label(Label::Allow),
            Node::Allow,
            Node::Label(Label::Refuse),
            Node::Refuse,
            Node::Label(Label::KillAbi),
            Node::KillAbi,
        ]);
        if nodes.len() > INSTRUCTION_LIMIT {
            return Err(BackendPolicyError::InvalidProgram);
        }
        lower(&nodes)
    }

    fn instruction(code: u16, k: u32) -> sock_filter {
        sock_filter {
            code,
            jt: 0,
            jf: 0,
            k,
        }
    }

    fn lower(nodes: &[Node]) -> Result<BpfProgram, BackendPolicyError> {
        let mut labels = [None; 10];
        let mut position = 0_usize;
        for node in nodes {
            match node {
                Node::Label(label) => {
                    let slot = labels
                        .get_mut(label.slot())
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                    if slot.replace(position).is_some() {
                        return Err(BackendPolicyError::InvalidProgram);
                    }
                }
                Node::Load(_)
                | Node::And(_)
                | Node::Equal(_, _)
                | Node::AnyBit(_, _)
                | Node::Allow
                | Node::Refuse
                | Node::KillAbi => {
                    position = position
                        .checked_add(1)
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                }
                #[cfg(target_arch = "x86_64")]
                Node::GreaterOrEqual(_, _) => {
                    position = position
                        .checked_add(1)
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                }
            }
        }
        if position > INSTRUCTION_LIMIT {
            return Err(BackendPolicyError::InvalidProgram);
        }
        let mut program = Vec::new();
        program
            .try_reserve_exact(INSTRUCTION_LIMIT)
            .map_err(|error| BackendPolicyError::Preparation(std::io::Error::other(error)))?;
        for node in nodes {
            let next = match node {
                Node::Label(_) => continue,
                Node::Load(offset) => instruction(0x20, *offset), // LD|W|ABS
                Node::And(mask) => instruction(0x54, *mask),      // ALU|AND|K
                Node::Equal(value, target) | Node::AnyBit(value, target) => {
                    let destination = labels
                        .get(target.slot())
                        .copied()
                        .flatten()
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                    let offset = destination
                        .checked_sub(
                            program
                                .len()
                                .checked_add(1)
                                .ok_or(BackendPolicyError::InvalidProgram)?,
                        )
                        .and_then(|offset| u8::try_from(offset).ok())
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                    sock_filter {
                        code: match node {
                            Node::Equal(_, _) => 0x15,  // JMP|JEQ|K
                            Node::AnyBit(_, _) => 0x45, // JMP|JSET|K
                            #[cfg(target_arch = "x86_64")]
                            Node::GreaterOrEqual(_, _) => {
                                return Err(BackendPolicyError::InvalidProgram)
                            }
                            Node::Label(_)
                            | Node::Load(_)
                            | Node::And(_)
                            | Node::Allow
                            | Node::Refuse
                            | Node::KillAbi => return Err(BackendPolicyError::InvalidProgram),
                        },
                        jt: offset,
                        jf: 0,
                        k: *value,
                    }
                }
                #[cfg(target_arch = "x86_64")]
                Node::GreaterOrEqual(value, target) => {
                    let destination = labels
                        .get(target.slot())
                        .copied()
                        .flatten()
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                    let offset = destination
                        .checked_sub(
                            program
                                .len()
                                .checked_add(1)
                                .ok_or(BackendPolicyError::InvalidProgram)?,
                        )
                        .and_then(|offset| u8::try_from(offset).ok())
                        .ok_or(BackendPolicyError::InvalidProgram)?;
                    sock_filter {
                        code: 0x35, // JMP|JGE|K
                        jt: offset,
                        jf: 0,
                        k: *value,
                    }
                }
                Node::Allow => instruction(0x06, nix::libc::SECCOMP_RET_ALLOW),
                Node::Refuse => instruction(
                    0x06,
                    nix::libc::SECCOMP_RET_ERRNO | argument(nix::libc::EPERM)?,
                ),
                Node::KillAbi => instruction(0x06, nix::libc::SECCOMP_RET_KILL_PROCESS),
            };
            program.push(next);
        }
        Ok(program)
    }

    fn privilege<T>(result: rustix::io::Result<T>) -> Result<T, BackendPolicyError> {
        result.map_err(|error| BackendPolicyError::Privilege(error.into()))
    }

    pub(super) fn install() -> Result<(), BackendPolicyError> {
        let program = program()?;
        super::super::outer_setup::require_single_thread()
            .map_err(|error| BackendPolicyError::Preparation(std::io::Error::other(error)))?;
        if rustix::process::getpid().as_raw_nonzero().get() == 1 {
            return Err(BackendPolicyError::NotBackend);
        }
        privilege(set_no_new_privs(true))?;
        if privilege(capability_is_in_bounding_set(CapabilitySet::SYS_PTRACE))? {
            privilege(remove_capability_from_bounding_set(
                CapabilitySet::SYS_PTRACE,
            ))?;
        }
        privilege(clear_ambient_capability_set())?;
        privilege(set_capabilities(
            None,
            CapabilitySets {
                effective: CapabilitySet::empty(),
                permitted: CapabilitySet::empty(),
                inheritable: CapabilitySet::empty(),
            },
        ))?;
        let actual = privilege(capabilities(None))?;
        if !actual.effective.is_empty()
            || !actual.permitted.is_empty()
            || !actual.inheritable.is_empty()
            || privilege(capability_is_in_bounding_set(CapabilitySet::SYS_PTRACE))?
            || privilege(capability_is_in_ambient_set(CapabilitySet::SYS_PTRACE))?
            || !privilege(no_new_privs())?
        {
            return Err(BackendPolicyError::ProtectionUnverified);
        }
        // Safe dependency API installs on this sole thread. The filter/NNP persist into exact
        // recipe exec and descendants. No TSYNC of any supervisor and no exec callback occurs.
        seccompiler::apply_filter(&program).map_err(BackendPolicyError::Filter)
    }
}

/// Install in the authenticated same-PID backend boundary before positive Dispatch only.
/// A caller must report any error as unavailable admission and settle; partially established
/// protection cannot authorize Dispatch or an unfiltered retry. Descriptor/ABI admission and
/// INIT's separate protection must already have been established by their actual owners.
pub(super) fn install() -> Result<(), BackendPolicyError> {
    #[cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )
    ))]
    return supported::install();
    #[cfg(not(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64"
        )
    )))]
    Err(BackendPolicyError::UnsupportedArchitecture)
}
