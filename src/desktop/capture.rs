use windows::{
    core::Result,
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{
        Foundation::HWND, Graphics::Gdi::HMONITOR,
        System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
    },
};

pub fn create_capture_item_for_window(window_handle: HWND) -> Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    unsafe { interop.CreateForWindow(window_handle) }
}

pub fn create_capture_item_for_monitor(monitor_handle: HMONITOR) -> Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    unsafe { interop.CreateForMonitor(monitor_handle) }
}
