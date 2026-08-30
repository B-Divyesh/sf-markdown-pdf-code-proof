//! Linux containment for renderer subprocesses.
//!
//! Renderers are deliberately treated as untrusted: they receive a narrow
//! filesystem allowlist and cannot create or use sockets.  We use kernel
//! primitives rather than an advisory environment convention so failing to
//! establish containment fails the render.

#[cfg(target_os = "linux")]
use std::collections::BTreeMap;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
const LANDLOCK_CREATE_RULESET_VERSION: libc::c_uint = 1;
#[cfg(target_os = "linux")]
const LANDLOCK_RULE_PATH_BENEATH: libc::c_int = 1;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_EXECUTE: u64 = 1 << 0;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_WRITE_FILE: u64 = 1 << 1;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_READ_FILE: u64 = 1 << 2;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_READ_DIR: u64 = 1 << 3;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_REMOVE_DIR: u64 = 1 << 4;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_REMOVE_FILE: u64 = 1 << 5;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_CHAR: u64 = 1 << 6;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_DIR: u64 = 1 << 7;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_REG: u64 = 1 << 8;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_SOCK: u64 = 1 << 9;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_FIFO: u64 = 1 << 10;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_BLOCK: u64 = 1 << 11;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_MAKE_SYM: u64 = 1 << 12;
#[cfg(target_os = "linux")]
const LANDLOCK_ACCESS_FS_REFER: u64 = 1 << 13;
#[cfg(target_os = "linux")]
const LANDLOCK_ALL_ACCESS: u64 = LANDLOCK_ACCESS_FS_EXECUTE
    | LANDLOCK_ACCESS_FS_WRITE_FILE
    | LANDLOCK_ACCESS_FS_READ_FILE
    | LANDLOCK_ACCESS_FS_READ_DIR
    | LANDLOCK_ACCESS_FS_REMOVE_DIR
    | LANDLOCK_ACCESS_FS_REMOVE_FILE
    | LANDLOCK_ACCESS_FS_MAKE_CHAR
    | LANDLOCK_ACCESS_FS_MAKE_DIR
    | LANDLOCK_ACCESS_FS_MAKE_REG
    | LANDLOCK_ACCESS_FS_MAKE_SOCK
    | LANDLOCK_ACCESS_FS_MAKE_FIFO
    | LANDLOCK_ACCESS_FS_MAKE_BLOCK
    | LANDLOCK_ACCESS_FS_MAKE_SYM
    | LANDLOCK_ACCESS_FS_REFER;
#[cfg(target_os = "linux")]
const LANDLOCK_READ_EXECUTE: u64 =
    LANDLOCK_ACCESS_FS_EXECUTE | LANDLOCK_ACCESS_FS_READ_FILE | LANDLOCK_ACCESS_FS_READ_DIR;
#[cfg(target_os = "linux")]
const LANDLOCK_READ_ONLY: u64 = LANDLOCK_ACCESS_FS_READ_FILE | LANDLOCK_ACCESS_FS_READ_DIR;

#[cfg(target_os = "linux")]
#[repr(C)]
struct LandlockRulesetAttr {
    handled_access_fs: u64,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct LandlockPathBeneathAttr {
    allowed_access: u64,
    parent_fd: i32,
    reserved: u32,
}

/// Prepared file descriptors held open until the child has installed its rules.
#[cfg(target_os = "linux")]
pub struct Sandbox {
    directories: Vec<(File, u64)>,
}

#[cfg(target_os = "linux")]
impl Sandbox {
    pub fn prepare(source_parent: &Path, workspace: &Path, program: &Path) -> Result<Self, String> {
        if std::env::var_os("CODEPROOF_TEST_FORCE_SANDBOX_FAILURE").is_some() {
            return Err(
                "renderer sandbox setup was deliberately refused for this verification".into(),
            );
        }
        let mut permissions = BTreeMap::<PathBuf, u64>::new();
        add(&mut permissions, source_parent, LANDLOCK_READ_ONLY);
        add(&mut permissions, workspace, LANDLOCK_ALL_ACCESS);
        for path in ["/usr", "/bin", "/lib", "/lib64"] {
            let path = Path::new(path);
            if path.is_dir() {
                add(&mut permissions, path, LANDLOCK_READ_EXECUTE);
            }
        }
        if let Some(parent) = program.parent() {
            add(&mut permissions, parent, LANDLOCK_READ_EXECUTE);
        }
        let directories = permissions
            .into_iter()
            .map(|(path, access)| {
                File::open(&path)
                    .map(|file| (file, access))
                    .map_err(|error| {
                        format!("could not prepare sandbox path {}: {error}", path.display())
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { directories })
    }

    /// Runs after fork and before exec. Every failure aborts the renderer.
    ///
    /// # Safety
    ///
    /// This must run only in the renderer child, after fork and before exec.
    /// It permanently restricts the calling process and uses raw kernel APIs.
    pub unsafe fn apply(&self) -> io::Result<()> {
        let abi = libc::syscall(
            libc::SYS_landlock_create_ruleset,
            std::ptr::null::<LandlockRulesetAttr>(),
            0usize,
            LANDLOCK_CREATE_RULESET_VERSION,
        );
        if abi < 1 {
            return Err(io::Error::other(
                "Landlock filesystem sandbox is unavailable (Linux 5.13+ required)",
            ));
        }
        let ruleset = LandlockRulesetAttr {
            handled_access_fs: LANDLOCK_ALL_ACCESS,
        };
        let ruleset_fd = libc::syscall(
            libc::SYS_landlock_create_ruleset,
            &ruleset,
            std::mem::size_of::<LandlockRulesetAttr>(),
            0u32,
        );
        if ruleset_fd < 0 {
            return Err(io::Error::last_os_error());
        }
        for (directory, access) in &self.directories {
            let rule = LandlockPathBeneathAttr {
                allowed_access: *access,
                parent_fd: directory.as_raw_fd(),
                reserved: 0,
            };
            if libc::syscall(
                libc::SYS_landlock_add_rule,
                ruleset_fd,
                LANDLOCK_RULE_PATH_BENEATH,
                &rule,
                0u32,
            ) < 0
            {
                let error = io::Error::last_os_error();
                libc::close(ruleset_fd as i32);
                return Err(error);
            }
        }
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
            let error = io::Error::last_os_error();
            libc::close(ruleset_fd as i32);
            return Err(error);
        }
        if libc::syscall(libc::SYS_landlock_restrict_self, ruleset_fd, 0u32) < 0 {
            let error = io::Error::last_os_error();
            libc::close(ruleset_fd as i32);
            return Err(error);
        }
        libc::close(ruleset_fd as i32);
        install_socket_filter()
    }
}

#[cfg(target_os = "linux")]
fn add(permissions: &mut BTreeMap<PathBuf, u64>, path: &Path, access: u64) {
    permissions
        .entry(path.to_path_buf())
        .and_modify(|existing| *existing |= access)
        .or_insert(access);
}

#[cfg(target_os = "linux")]
unsafe fn install_socket_filter() -> io::Result<()> {
    const BPF_LD_W_ABS: u16 = libc::BPF_LD as u16 | libc::BPF_W as u16 | libc::BPF_ABS as u16;
    const BPF_JEQ_K: u16 = libc::BPF_JMP as u16 | libc::BPF_JEQ as u16 | libc::BPF_K as u16;
    const BPF_RET_K: u16 = libc::BPF_RET as u16 | libc::BPF_K as u16;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
    const SECCOMP_RET_ERRNO: u32 = 0x0005_0000;
    let denied = [
        libc::SYS_socket,
        libc::SYS_socketpair,
        libc::SYS_connect,
        libc::SYS_bind,
        libc::SYS_listen,
        libc::SYS_accept,
        libc::SYS_accept4,
        libc::SYS_sendto,
        libc::SYS_sendmsg,
        libc::SYS_recvfrom,
        libc::SYS_recvmsg,
    ];
    let mut filter = Vec::with_capacity(2 + denied.len() * 2);
    filter.push(libc::sock_filter {
        code: BPF_LD_W_ABS,
        jt: 0,
        jf: 0,
        k: 0,
    });
    for syscall in denied {
        filter.push(libc::sock_filter {
            code: BPF_JEQ_K,
            jt: 0,
            jf: 1,
            k: syscall as u32,
        });
        filter.push(libc::sock_filter {
            code: BPF_RET_K,
            jt: 0,
            jf: 0,
            k: SECCOMP_RET_ERRNO | libc::EPERM as u32,
        });
    }
    filter.push(libc::sock_filter {
        code: BPF_RET_K,
        jt: 0,
        jf: 0,
        k: SECCOMP_RET_ALLOW,
    });
    let program = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };
    if libc::prctl(libc::PR_SET_SECCOMP, libc::SECCOMP_MODE_FILTER, &program) != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub struct Sandbox;

#[cfg(not(target_os = "linux"))]
impl Sandbox {
    pub fn prepare(
        _source_parent: &std::path::Path,
        _workspace: &std::path::Path,
        _program: &std::path::Path,
    ) -> Result<Self, String> {
        Err("renderer sandbox unavailable: Code Proof requires Linux Landlock and seccomp; it will not run an unsandboxed renderer".into())
    }
}
