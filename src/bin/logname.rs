extern crate libc;

use libc::funcs::posix88::unistd::getlogin;
use std::c_str::CString;
use std::os;


fn main() {
    let login = unsafe {
        CString::new(
            getlogin() as *const libc::c_char,
            false
        )
    };

    match login.as_str() {
        Some(username) => println!("{}", username),
        None => os::set_exit_status(1),
    }
}
