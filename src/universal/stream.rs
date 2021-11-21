use windows::{
    core::Result,
    Storage::Streams::{IRandomAccessStream, IRandomAccessStreamWithContentType},
    Win32::System::{Com::IStream, WinRT::CreateStreamOverRandomAccessStream},
};

pub trait AsIStream {
    fn as_istream(&self) -> Result<IStream>;
}

impl AsIStream for IRandomAccessStream {
    fn as_istream(&self) -> Result<IStream> {
        unsafe { CreateStreamOverRandomAccessStream(self) }
    }
}

impl AsIStream for IRandomAccessStreamWithContentType {
    fn as_istream(&self) -> Result<IStream> {
        unsafe { CreateStreamOverRandomAccessStream(self) }
    }
}
