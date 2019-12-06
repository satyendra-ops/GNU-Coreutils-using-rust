extern crate libc;
use std::ffi::{CStr, CString, OsStr};
use std::mem;
use std::ptr;
use std::env;
//use libc::{c_char, uid_t, gid_t, c_int, getlogin};
use std::os::unix::ffi::OsStrExt;

//function to get username from userid
//uses libc function getpwuid_r which is thread-safe
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

//function to get groupname from gid
//uses libc function getgrgid_r which is thread-safe
fn get_unix_groupname(gid: u32) -> Option<String> {

    unsafe {
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETGR_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut group: libc::group = mem::zeroed();

        match libc::getgrgid_r(gid, &mut group, buf.as_mut_ptr(),
                              buf.capacity() as libc::size_t,
                              &mut result) {
           0 if !result.is_null() => {
               let ptr = group.gr_name as *const _;
               let groupname = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
               Some(groupname)
           },
           _ => None
        }
    }

}

//function to get uid from username in case of user entered username
//uses libc function getpwnam_r which is thread-safe
fn get_unix_uid<S: AsRef<OsStr> + ?Sized>(username: &S) -> u32 
{
    
    let username = match CString::new(username.as_ref().as_bytes()) 
    {
        Ok(u)  => u,
        Err(_) => {
            // The username that was passed in contained a null character,
            // which will match no usernames.
            return 999;
        }
    };
    
    unsafe 
	{
	
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) 
		{
            		n if n < 0 => 512 as usize,
            		n => n as usize,
        	};
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();

        libc::getpwnam_r(username.as_ptr(), &mut passwd, buf.as_mut_ptr(), buf.capacity() as libc::size_t, &mut result);
    
	if result.is_null() {
        // There is no such user, or an error has occurred.
        // errno gets set if there’s an error.
        return 999;
    }

        if result != &mut passwd {
        // The result of getpwnam_r should be its input passwd.
        return 999;
    }
	
	let userid=(*result).pw_uid;
	println!("{}",userid);
return userid;

}}

//function to get uid from username in case of user entered username
//uses libc function getpwnam_r which is thread-safe
fn get_unix_gid<S: AsRef<OsStr> + ?Sized>(username: &S) -> u32 
{
    
    let username = match CString::new(username.as_ref().as_bytes()) 
    {
        Ok(u)  => u,
        Err(_) => {
            // The username that was passed in contained a null character,
            // which will match no usernames.
            return 999;
        }
    };
    
    unsafe 
	{
	
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) 
		{
            		n if n < 0 => 512 as usize,
            		n => n as usize,
        	};
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();

        libc::getpwnam_r(username.as_ptr(), &mut passwd, buf.as_mut_ptr(), buf.capacity() as libc::size_t, &mut result);
    
	if result.is_null() {
        // There is no such user, or an error has occurred.
        // errno gets set if there’s an error.
        return 999;
    }

        if result != &mut passwd {
        // The result of getpwnam_r should be its input passwd.
        return 999;
    }
	
	let userid=(*result).pw_gid;
	//println!("{}",userid);
return userid;

}}
	
fn main() {
	let args: Vec<String> = env::args().collect();
    	println!("{:?}", args);
	let n = args.len();
	//parse options and set flags
    	let mut uflag = 0;
    	let mut gflag = 0;
	let name;
	let mut j=9999;
	if n>1
	{	
		for i in 1..n
		{
			if args[i]=="g" {gflag=1;}
			else if args[i]=="u" {uflag=1;}
			else {j=i;}
		}
	}
	

    	unsafe{ let id;let gid;
		if j!=9999
		{
			name=&args[j];
			//println!("{}",name);
			id = get_unix_uid(name);
			gid = get_unix_gid(name);
		}
		else
		{
			id=libc::geteuid();
			gid=libc::getegid();
		}
		let uname = get_unix_username(id).unwrap();
		let gname = get_unix_groupname(gid).unwrap();
		if n==1
		{
			println!("uid={}({}) gid={}({})",id,uname,gid,gname);
		}
		if n==2 && gflag==0 && uflag==0
		{
			println!("uid={}({}) gid={}({})",id,uname,gid,gname);
		}
		if uflag==1
		{
			print!("uid={}({}) ",id,uname);
		}
		if gflag==1
		{
			println!("gid={}({})",gid,gname);
		}
    		
     }
}
