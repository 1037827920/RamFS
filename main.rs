extern crate libc;

use std::ffi::CString;
use std::ptr;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 创建文件
    let filename = CString::new("testfile.txt").expect("CString::new failed");
    let mode = libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC;
    let fd = unsafe { libc::open(filename.as_ptr(), mode, 0o644) };
    if fd < 0 {
        panic!("Failed to create file");
    }

    // 写入文件
    let content = "Hello, world!";
    let content_bytes = content.as_bytes();
    let ret = unsafe { libc::write(fd, content_bytes.as_ptr() as *const libc::c_void, content_bytes.len()) };
    if ret < 0 {
        panic!("Failed to write to file");
    }

    // 关闭文件
    unsafe { libc::close(fd) };

    // 重新打开文件以读取内容
    let filename = CString::new("testfile.txt").expect("CString::new failed");
    let fd = unsafe { libc::open(filename.as_ptr(), libc::O_RDONLY, 0o644) };
    if fd < 0 {
        panic!("Failed to open file for reading");
    }

    // 读取文件内容
    let mut buffer = [0u8; 128]; // 用于存储读取的内容
    let ret = unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };
    if ret < 0 {
        panic!("Failed to read from file");
    }

    // 输出到控制台
    let content_read = std::str::from_utf8(&buffer[..ret as usize]).expect("Failed to convert bytes to string");
    println!("Content read from file: {}", content_read);

    // 关闭文件
    unsafe { libc::close(fd) };
}
