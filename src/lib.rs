//! Easy way for loading Windows DLLs
//!
//! Provides a light wrapper to the Windows API calls for loading DLL libraries,
//! as well as an attribute macro that allows easy definition of the interfaces

#![no_std]

extern crate alloc;
extern crate core;

pub mod error;
pub use win_lib_loader_proc::load_from_dll;

use core::mem::transmute_copy;

use alloc::{ffi::CString, string::ToString};
use error::LoaderError;
use winapi::{
    shared::minwindef::HINSTANCE__,
    um::{
        errhandlingapi::GetLastError,
        libloaderapi::{GetProcAddress, LoadLibraryA},
    },
};

/// Provides a wrapper for loading a DLL library
///
/// # Example
/// ```
/// use win_lib_loader::WinLibrary;
/// let test_dll = WinLibrary::load("testdll.dll").unwrap();
/// type AddFn = fn(i32,i32)->i32;
/// let add_fn = test_dll.get_function::<AddFn>("add").unwrap();
/// assert_eq!(8, add_fn(5,3));
/// ```
pub struct WinLibrary {
    handle: &'static mut HINSTANCE__,
}
impl WinLibrary {
    pub fn load(filename: &str) -> Result<Self, LoaderError> {
        let name = CString::new(filename)
            .map_err(|_| LoaderError::InterfaceError("Could not make CString".to_string()))?;

        let handle = unsafe { LoadLibraryA(name.as_ptr()).as_mut() }.ok_or_else(|| {
            let err = unsafe { GetLastError() };
            LoaderError::LibraryLoadError(err)
        })?;

        Ok(WinLibrary { handle })
    }
    pub fn get_function<F>(&self, function: &str) -> Result<F, LoaderError> {
        let name = CString::new(function)
            .map_err(|_| LoaderError::InterfaceError("Could not make CString".to_string()))?;
        let function_ptr = unsafe {
            GetProcAddress(
                self.handle as *const HINSTANCE__ as *mut HINSTANCE__,
                name.as_ptr(),
            )
        };
        if function_ptr.is_null() {
            return Err(LoaderError::FunctionLoadError);
        }
        let function_ptr = unsafe { transmute_copy(&function_ptr) };
        Ok(function_ptr)
    }
}
