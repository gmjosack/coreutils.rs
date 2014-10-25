extern crate libc;

use libc::funcs::posix88::unistd::getlogin;
use std::c_str::CString;
use std::os;


fn main() {
    unsafe {

        let login_c = getlogin() as *const libc::c_char;
        let login = CString::new(login_c, false);

        match login.as_str() {
            Some(username) => println!("{}", username),
            None => os::set_exit_status(1),
        }

    }
}
