extern crate libc;

use std::mem;
use std::ptr;
use std::ffi::CStr;

fn get_unix_username(uid: u32) -> Option<String> {

    unsafe {
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();

        match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(),
                              buf.capacity() as libc::size_t,
                              &mut result) {
           0 if !result.is_null() => {
               let ptr = passwd.pw_name as *const _;
               let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
               Some(username)
           },
           _ => None
        }
    }

}
	
fn main() {
    unsafe{ let uid=libc::geteuid();         
	let uname = get_unix_username(uid).unwrap();

    println!("{}", uname);
     }
}
