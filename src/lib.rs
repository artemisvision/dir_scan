extern crate libc;

use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;

#[repr(C)]
pub struct FileList {
    list: *mut *mut c_char,
    len: size_t,
}

#[no_mangle]
pub extern "C" fn scan_dir(working_dir: *const c_char) -> *mut FileList {
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
    Box::into_raw(Box::new(FileList {
        list: vecptr,
        len: ret,
    }))
}

#[no_mangle]
pub extern "C" fn strip_file_ext(file_path: *const c_char) -> *mut c_char {
    if file_path.is_null() 
    {
        CString::new("").unwrap().into_raw()
    }
    else {
        let r_str = unsafe{CStr::from_ptr(file_path)}.to_str().unwrap();
        let path = Path::new(r_str);
        let filename_no_ext = path.file_stem().unwrap().to_str().unwrap();
        CString::new(filename_no_ext).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn create_all_dirs(path: *const c_char) {
    if path.is_null() {
        // TODO return error
        return;
    }
    else {
        let r_str = unsafe{ CStr::from_ptr(path)}.to_str().unwrap();
        let path = Path::new(r_str).parent().unwrap();
        match std::fs::create_dir_all(path) {
            Ok(_) => (),
            Err(e) => println!("Error creating directories: {:?}", e)
        }

    }
}

#[no_mangle]
pub extern "C" fn free_filelist(filelist: *mut FileList) {
    unsafe {
        if filelist.is_null() {return;}
        let f = Box::from_raw(filelist);
        let list = Vec::from_raw_parts(f.list, f.len, f.len);
        for i in 0..f.len {
            free_string(list[i]);
        }
    }
}


#[no_mangle]
pub extern "C" fn free_string(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() {return;}
        CString::from_raw(s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strip_file_ext_test() {
        let file = String::from("some.file");
        let f = CString::new(file).unwrap().into_raw();
        let stripped_file = strip_file_ext(f);
        let assert_file = unsafe{CStr::from_ptr(stripped_file)}.to_str().unwrap();
        assert_eq!("some", assert_file);
        unsafe {
            CString::from_raw(stripped_file);
            CString::from_raw(f);
        }
    }
}
