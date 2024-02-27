extern crate libc;

use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read};
use std::ptr;

fn main() -> io::Result<()> {
    // 文件路径
    let filename = "/path/to/your/file.txt";
    
    // 打开文件，返回文件描述符
    let fd: libc::c_int;
    let c_filename = CString::new(filename)?;
    unsafe {
        fd = libc::open(c_filename.as_ptr(), libc::O_RDONLY);
    }
    if fd == -1 {
        return Err(io::Error::last_os_error());
    }
    
    // 读取文件内容
    let mut buf = [0u8; 4096];
    loop {
        let bytes_read: isize;
        unsafe {
            bytes_read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
        }
        if bytes_read == -1 {
            return Err(io::Error::last_os_error());
        } else if bytes_read == 0 {
            break; // 文件读取完毕
        } else {
            // 输出文件内容
            let slice = &buf[0..bytes_read as usize];
            io::stdout().write_all(slice)?;
        }
    }
    
    // 关闭文件
    unsafe {
        libc::close(fd);
    }
    
    Ok(())
}
