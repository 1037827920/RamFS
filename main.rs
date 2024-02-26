extern crate libc;
use libc::mount;
use libc::c_char;


fn main() {
    let mut src:[c_char;10]=[0;10];
    src[0]='/' as c_char;

    let mut target:[c_char;10]=[0;10];
    target[0]='/' as c_char;
    target[1]='m' as c_char;
    target[2]='n' as c_char;
    target[3]='t' as c_char;

    let mut fstype:[c_char;10]=[0;10];
    fstype[0]='r' as c_char;
    fstype[1]='a' as c_char;
    fstype[2]='m' as c_char;
    fstype[3]='f' as c_char;
    fstype[4]='s' as c_char;
    unsafe{
        let c_str=std::ffi::CStr::from_ptr(src.as_ptr());
        println!("{:?}",c_str);
    }
    unsafe{
        let result=mount(src.as_ptr(), target.as_ptr(), fstype.as_ptr(), 0, std::ptr::null());
        if result!=0{
            println!("mount fs failed!");
        }
    }
}