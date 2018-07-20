extern crate libc;

use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::fs;

#[repr(C)]
pub struct FileList {
    list: *mut *mut c_char,
    len: size_t,
}
#[no_mangle]
pub extern "C" fn scan_dir(working_dir: *const c_char, files: *mut *mut c_char) -> FileList {
    let c_str = unsafe {CStr::from_ptr(working_dir)};
    let foldername = c_str.to_str().unwrap();
    let r_files = fs::read_dir(foldername).unwrap();
    let mut paths: Vec<*mut c_char> = r_files.map(|file| 
        CString::new(
            file.unwrap().path().to_str().unwrap())
        .unwrap().into_raw())
    .collect();
    paths.shrink_to_fit();
    let ret = paths.len();

    let vecptr = paths.as_mut_ptr();
    std::mem::forget(paths);
    FileList {
        list: vecptr,
        len: ret,
    }
}

#[no_mangle]
pub extern "C" fn free(files: *mut *mut c_char, len: size_t) {
    unsafe {
        if files.is_null() {return;}
        let f: Vec<*mut c_char> = Vec::from_raw_parts(files, len, len);
        for i in f {
            free_string(i);
        }
    }
}

fn free_string(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() {return;}
        CString::from_raw(s)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
