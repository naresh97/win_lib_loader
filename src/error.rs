use core::fmt::Display;

use alloc::string::String;

#[derive(Debug)]
pub enum LoaderError {
    LibraryLoadError(u32),
    FunctionLoadError,
    InterfaceError(String),
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoaderError::LibraryLoadError(err) => {
                write!(f, "could not load library: win err {}", err)
            }
            LoaderError::FunctionLoadError => write!(f, "could not load function"),
            LoaderError::InterfaceError(s) => write!(f, "{}", s),
        }
    }
}
