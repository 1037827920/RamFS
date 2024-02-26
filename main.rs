extern crate libc;

use libc::{c_char, c_int, size_t, ssize_t, O_RDONLY, O_WRONLY, O_CREAT, open, read, write, close};
use std::ffi::CStr;

fn main() {
    let filename = "file.txt";
    let fd = unsafe { open(filename.as_ptr() as *const c_char, O_RDONLY) };
    if fd == -1 {
        panic!("Failed to open file");
    }

    // Write to file
    let mut output_buf = b"Hello, DragonOS!\0";
    let bytes_written = unsafe { write(fd, output_buf.as_ptr() as *const libc::c_void, output_buf.len() as size_t) };
    if bytes_written == -1 {
        panic!("Failed to write to file");
    }


    // Read from file
    let mut buf = [0u8; 1024];
    let bytes_read = unsafe { read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len() as size_t) };
    if bytes_read == -1 {
        panic!("Failed to read from file");
    }

    

    // Convert read bytes to string
    let content = unsafe { std::str::from_utf8_unchecked(&buf[0..bytes_read as usize]) };

    // Print read content
    println!("Read content: {}", content);


    // Close file
    let status = unsafe { close(fd) };
    if status == -1 {
        panic!("Failed to close file");
    }
}
