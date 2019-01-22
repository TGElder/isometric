use std::ffi::CString;

pub fn create_whitespace_cstring_with_len(length: usize) -> CString {
    let mut buffer: Vec<u8> = vec![b' '; length + 1];
    unsafe { CString::from_vec_unchecked(buffer) }
}