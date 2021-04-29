use tokio::io::Error;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::winnt::TokenElevation;
use std::ptr::null_mut;

use std::ffi::OsStr;
use std::os::windows::prelude::*;
use std::iter::once;

use std::process::Stdio;
use std::io::{self, Write};

/// Based on: https://users.rust-lang.org/t/how-do-i-determine-if-i-have-admin-rights-on-windows/35710/8
/// Returns true if the current process has admin rights, otherwise false.
pub fn is_app_elevated() -> bool {
    _is_app_elevated().unwrap_or(false)
}

/// On success returns a bool indicating if the current process has admin rights.
/// Otherwise returns an OS error.
///
/// This is unlikely to fail but if it does it's even more unlikely that you have admin permissions anyway.
/// Therefore the public function above simply eats the error and returns a bool.
pub fn _is_app_elevated() -> Result<bool, Error> {
    let token = QueryAccessToken::from_current_process()?;
    token.is_elevated()
}

/// A safe wrapper around querying Windows access tokens.
pub struct QueryAccessToken(HANDLE);

impl QueryAccessToken {
    pub fn from_current_process() -> Result<Self, Error> {
        unsafe {
            let mut handle: HANDLE = null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) != 0 {
                Ok(Self(handle))
            } else {
                Err(Error::last_os_error())
            }
        }
    }

    /// On success returns a bool indicating if the access token has elevated privilidges.
    /// Otherwise returns an OS error.
    pub fn is_elevated(&self) -> Result<bool, Error> {
        unsafe {
            let mut elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            // The weird looking repetition of `as *mut _` is casting the reference to a c_void pointer.
            if GetTokenInformation(self.0, TokenElevation, &mut elevation as *mut _ as *mut _, size, &mut ret_size) != 0 {
                Ok(elevation.TokenIsElevated != 0)
            } else {
                Err(Error::last_os_error())
            }
        }
    }
}

impl Drop for QueryAccessToken {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub fn to_wchar(str: &str) -> Vec<winapi::um::winnt::WCHAR> {
    // OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect()
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}


pub fn run_elevated(arguments: Vec<String>) {
    // Based on https://github.com/rust-lang/rustup/pull/1117/files
    let current_exe = std::env::current_exe().unwrap().display().to_string();
    let argument_string = arguments.clone().into_iter().map(|i| format!("{} ", i.to_string())).collect::<String>();
    let parameters_string = format!("{}", argument_string);
    let operation = to_wchar("runas");
    let path = to_wchar(&current_exe);
    let parameters = to_wchar(&parameters_string);
    let sw_showminnoactive = 7;
    println!("Requesting elevation of privileges for: {} {}", current_exe, parameters_string);

    let result = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew
        winapi::um::shellapi::ShellExecuteW(null_mut(),
                                            operation.as_ptr(),
                                            path.as_ptr(),
                                            parameters.as_ptr(),
                                            null_mut(),
                                            sw_showminnoactive)
    };

    match result {
        _ => { println!("Exit code: {:?}", result); }
    }

    // pub fn ShellExecuteA(
    //     hwnd: HWND,
    //     lpOperation: LPCSTR,
    //     lpFile: LPCSTR,
    //     lpParameters: LPCSTR,
    //     lpDirectory: LPCSTR,
    //     nShowCmd: c_int,
    // ) -> HINSTANCE;


}

pub fn run(command:String, arguments:Vec<String>) -> Result<bool, Error> {
    println!("Executing: {} {:?}", command, arguments);
    let mut child_process = std::process::Command::new(command)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        // let child_stdin = child_process.stdin.as_mut().unwrap();
        // child_stdin.write_all(b"cd examples/get-started/blink; idf.py fullclean; idf.py build\n")?;
        // Close stdin to finish and avoid indefinite blocking
        // drop(child_stdin);
    }
    Ok(true)
}

pub fn run_self_elevated() -> Result<bool, Error> {
    if !is_app_elevated() {
        let mut arguments: Vec<String> = std::env::args().collect();
        arguments.remove(0);
        run_elevated(arguments);
    }
    Ok(true)
}

pub fn run_self_elevated_with_extra_argument(argument:String) -> Result<bool, Error>{
    if !is_app_elevated() {
        let mut arguments: Vec<String> = std::env::args().collect();
        arguments.remove(0);
        arguments.push(argument);
        run_elevated(arguments);
    }
    Ok(true)
}