#[repr(transparent)]
/// Project handle
pub struct ABTProjectHandle(pub *mut libc::c_void);

impl From<*mut animboom_toolkit::types::AnimBoomProject> for ABTProjectHandle {
    fn from(ptr: *mut animboom_toolkit::types::AnimBoomProject) -> Self {
        ABTProjectHandle(ptr as *mut libc::c_void)
    }
}
