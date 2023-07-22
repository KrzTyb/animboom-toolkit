use crate::*;

#[repr(transparent)]
/// Project handle
pub struct ABTProjectHandle(pub *mut libc::c_void);

impl From<*mut types::AnimBoomProject> for ABTProjectHandle {
    fn from(ptr: *mut types::AnimBoomProject) -> Self {
        ABTProjectHandle(ptr as *mut libc::c_void)
    }
}
