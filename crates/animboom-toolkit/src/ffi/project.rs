use super::*;
use std::ffi::CStr;
extern crate libc;

#[no_mangle]
/// Create new project
///
/// # Arguments
/// * `path` - A string that holds path to new project
///
/// # Returns
/// Handle to created project (null if failed)
///
/// # Safety
/// `path` argument shouldn't be a null pointer
///
pub unsafe extern "C" fn ABT_new_project(path: *const libc::c_char) -> types::ABTProjectHandle {
    let project = Box::new(crate::new_project(into_correct_path(path).as_str()));
    Box::into_raw(project).into()
}

#[no_mangle]
/// Open project
///
/// # Arguments
/// * `path` - A string that holds path to existing project
///
/// # Returns
/// Handle to opened project (null if failed)
///
/// # Safety
/// `path` argument shouldn't be a null pointer
///
pub unsafe extern "C" fn ABT_open_project(path: *const libc::c_char) -> types::ABTProjectHandle {
    let project = Box::new(crate::open_project(into_correct_path(path).as_str()));
    Box::into_raw(project).into()
}

#[no_mangle]
/// Close project
///
/// # Arguments
/// * `project_handle` - Handle to project
///
/// # Safety
/// `project_handle` argument shouldn't be a null pointer
///  It should also point to the correct handle returned by ABT_new_project or ABT_open_project
///
pub unsafe extern "C" fn ABT_close_project(project_handle: types::ABTProjectHandle) {
    assert!(!project_handle.0.is_null(), "Null pointer!");
    let project = Box::from_raw(project_handle.0 as *mut crate::types::AnimBoomProject);
    crate::close_project(*project);
}

fn into_correct_path(path: *const libc::c_char) -> String {
    let c_str = unsafe {
        assert!(!path.is_null(), "Null pointer!");
        CStr::from_ptr(path)
    };
    String::from_utf8_lossy(c_str.to_bytes()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn open_close_project() {
        let path = CString::new("/home/user/myproject.abp").expect("Conversion failed");
        let mut project;

        // Create new project
        unsafe {
            project = ABT_new_project(path.as_ptr());
        }
        assert2::assert!(!project.0.is_null());

        // Close project
        unsafe {
            ABT_close_project(project);
        }

        // Open project
        unsafe {
            project = ABT_open_project(path.as_ptr());
        }
        assert2::assert!(!project.0.is_null());
    }
}
