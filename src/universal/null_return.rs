use windows::{core::Result, Win32::Foundation::S_OK};

pub trait NullReturn<T> {
    fn as_option(self) -> Result<Option<T>>;
}

impl<T> NullReturn<T> for Result<T> {
    fn as_option(self) -> Result<Option<T>> {
        match self {
            Ok(element) => Ok(Some(element)),
            Err(error) => {
                if error.code() == S_OK {
                    Ok(None)
                } else {
                    Err(error.into())
                }
            }
        }
    }
}
