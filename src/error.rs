use core::fmt::Display;

use alloc::string::String;

#[derive(Debug)]
pub enum LoaderError {
    LibraryLoadError,
    FunctionLoadError,
    InterfaceError(String),
}

impl Display for LoaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoaderError::LibraryLoadError => write!(f, "could not load library"),
            LoaderError::FunctionLoadError => write!(f, "could not load function"),
            LoaderError::InterfaceError(s) => write!(f, "{}", s),
        }
    }
}
