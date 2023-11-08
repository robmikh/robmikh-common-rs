use windows::{
    core::{ComInterface, Result},
    Win32::{Foundation::HWND, System::WinRT::Composition::ICompositorDesktopInterop},
    UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
};

pub trait CompositorDesktopInterop {
    fn create_desktop_window_target(
        &self,
        hwnd_target: &HWND,
        is_top_most: bool,
    ) -> Result<DesktopWindowTarget>;
}

impl CompositorDesktopInterop for Compositor {
    fn create_desktop_window_target(
        &self,
        hwnd_target: &HWND,
        is_top_most: bool,
    ) -> Result<DesktopWindowTarget> {
        let interop: ICompositorDesktopInterop = self.cast()?;
        unsafe { interop.CreateDesktopWindowTarget(*hwnd_target, is_top_most) }
    }
}
