use windows::core::implement;
use windows::core::{Interface, Result};
use windows::Graphics::IGeometrySource2D_Impl;
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::Graphics::Direct2D::{
    ID2D1Factory, ID2D1Geometry, D2D1_DEBUG_LEVEL_INFORMATION,
};
use windows::Win32::Graphics::{
    Direct2D::{
        D2D1CreateFactory, ID2D1Device, ID2D1Factory1, D2D1_FACTORY_OPTIONS,
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
    },
    Direct3D11::ID3D11Device,
    Dxgi::IDXGIDevice,
};
use windows::Win32::System::WinRT::Graphics::Direct2D::IGeometrySource2DInterop_Impl;

pub fn create_d2d_factory() -> Result<ID2D1Factory1> {
    let options = {
        let mut options = D2D1_FACTORY_OPTIONS::default();
        if cfg!(feature = "d2d-debug") {
            options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        }
        options
    };
    let mut result = None;
    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            &mut result as *mut _ as *mut *mut _,
        )?;
    }
    Ok(result.unwrap())
}

pub fn create_d2d_device(factory: &ID2D1Factory1, device: &ID3D11Device) -> Result<ID2D1Device> {
    let dxgi_device: IDXGIDevice = device.cast()?;
    unsafe { factory.CreateDevice(&dxgi_device) }
}

#[implement(
    windows::Graphics::IGeometrySource2D,
    windows::Win32::System::WinRT::Graphics::Direct2D::IGeometrySource2DInterop
)]
pub struct GeometrySource {
    geometry: ID2D1Geometry,
}

#[allow(non_snake_case)]
impl GeometrySource {
    pub fn new(geometry: ID2D1Geometry) -> Self {
        Self { geometry }
    }
}

#[allow(non_snake_case)]
impl IGeometrySource2D_Impl for GeometrySource {}

#[allow(non_snake_case)]
impl IGeometrySource2DInterop_Impl for GeometrySource {
    fn GetGeometry(&self) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    fn TryGetGeometryUsingFactory(&self, _: &Option<ID2D1Factory>) -> Result<ID2D1Geometry> {
        E_NOTIMPL.ok()?;
        unreachable!()
    }
}
