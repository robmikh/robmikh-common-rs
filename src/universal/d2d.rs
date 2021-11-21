use windows as Windows;
use windows::core::{Interface, Result};
use windows::Win32::Graphics::{
    Direct2D::{
        D2D1CreateFactory, ID2D1Device, ID2D1Factory1, D2D1_FACTORY_OPTIONS,
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
    },
    Direct3D11::ID3D11Device,
    Dxgi::IDXGIDevice,
};
use Windows::core::implement;
use Windows::Win32::Foundation::E_NOTIMPL;
use Windows::Win32::Graphics::Direct2D::{ID2D1Factory, ID2D1Geometry};

pub fn create_d2d_factory() -> Result<ID2D1Factory1> {
    let options = D2D1_FACTORY_OPTIONS::default();
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
    Windows::Graphics::IGeometrySource2D,
    Windows::Win32::System::WinRT::Graphics::Direct2D::IGeometrySource2DInterop
)]
pub struct GeometrySource {
    geometry: ID2D1Geometry,
}

#[allow(non_snake_case)]
impl GeometrySource {
    pub fn new(geometry: ID2D1Geometry) -> Self {
        Self { geometry }
    }

    pub fn GetGeometry(&self) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    pub fn TryGetGeometryUsingFactory(&self, _: &Option<ID2D1Factory>) -> Result<ID2D1Geometry> {
        E_NOTIMPL.ok()?;
        unreachable!()
    }
}