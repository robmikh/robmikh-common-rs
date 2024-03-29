use windows::{
    core::{ComInterface, IUnknown, Result},
    Win32::{
        Foundation::{POINT, RECT, SIZE},
        Graphics::{Direct2D::ID2D1Device, Direct3D11::ID3D11Device, Dxgi::IDXGISwapChain1},
        System::WinRT::Composition::{ICompositionDrawingSurfaceInterop, ICompositorInterop},
    },
    UI::Composition::{
        CompositionDrawingSurface, CompositionGraphicsDevice, Compositor, ICompositionSurface,
    },
};

pub trait CompositorInterop {
    fn create_graphics_device_from_d3d_device(
        &self,
        device: &ID3D11Device,
    ) -> Result<CompositionGraphicsDevice>;
    fn create_graphics_device_from_d2d_device(
        &self,
        device: &ID2D1Device,
    ) -> Result<CompositionGraphicsDevice>;
    fn create_composition_surface_for_swap_chain(
        &self,
        swap_chain: &IDXGISwapChain1,
    ) -> Result<ICompositionSurface>;
}

impl CompositorInterop for Compositor {
    fn create_graphics_device_from_d3d_device(
        &self,
        device: &ID3D11Device,
    ) -> Result<CompositionGraphicsDevice> {
        let interop: ICompositorInterop = self.cast()?;
        let unknown: IUnknown = device.cast()?;
        unsafe { interop.CreateGraphicsDevice(&unknown) }
    }

    fn create_graphics_device_from_d2d_device(
        &self,
        device: &ID2D1Device,
    ) -> Result<CompositionGraphicsDevice> {
        let interop: ICompositorInterop = self.cast()?;
        let unknown: IUnknown = device.cast()?;
        unsafe { interop.CreateGraphicsDevice(&unknown) }
    }

    fn create_composition_surface_for_swap_chain(
        &self,
        swap_chain: &IDXGISwapChain1,
    ) -> Result<ICompositionSurface> {
        let interop: ICompositorInterop = self.cast()?;
        let unknown: IUnknown = swap_chain.cast()?;
        unsafe { interop.CreateCompositionSurfaceForSwapChain(&unknown) }
    }
}

pub trait CompositionDrawingSurfaceInterop {
    fn resize(&self, size: &SIZE) -> Result<()>;
    fn begin_draw<T: ComInterface>(&self, update_rect: Option<&RECT>) -> Result<(T, POINT)>;
    fn end_draw(&self) -> Result<()>;
}

impl CompositionDrawingSurfaceInterop for CompositionDrawingSurface {
    fn resize(&self, size: &SIZE) -> Result<()> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        unsafe { interop.Resize(*size) }
    }

    fn begin_draw<UpdateObject: ComInterface>(
        &self,
        update_rect: Option<&RECT>,
    ) -> Result<(UpdateObject, POINT)> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        let update_rect = if let Some(update_rect) = update_rect {
            Some(update_rect as *const _)
        } else {
            None
        };
        unsafe {
            let mut update_offset = POINT::default();
            let update_object =
                interop.BeginDraw::<UpdateObject>(update_rect, &mut update_offset)?;
            Ok((update_object, update_offset))
        }
    }

    fn end_draw(&self) -> Result<()> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        unsafe { interop.EndDraw() }
    }
}

pub struct CompositionSurfaceDrawingSession<UpdateObject: ComInterface> {
    surface: CompositionDrawingSurface,
    update_object: UpdateObject,
    update_offset: POINT,
}

impl<UpdateObject: ComInterface> CompositionSurfaceDrawingSession<UpdateObject> {
    pub fn new(surface: CompositionDrawingSurface) -> Result<Self> {
        let (update_object, update_offset) = surface.begin_draw(None)?;
        Ok(Self {
            surface,
            update_object,
            update_offset,
        })
    }
    pub fn new_with_update_rect(
        surface: CompositionDrawingSurface,
        update_rect: &RECT,
    ) -> Result<Self> {
        let (update_object, update_offset) = surface.begin_draw(Some(update_rect))?;
        Ok(Self {
            surface,
            update_object,
            update_offset,
        })
    }

    pub fn update_object(&self) -> &UpdateObject {
        &self.update_object
    }

    pub fn update_offset(&self) -> &POINT {
        &self.update_offset
    }
}

impl<UpdateObject: ComInterface> Drop for CompositionSurfaceDrawingSession<UpdateObject> {
    fn drop(&mut self) {
        self.surface.end_draw().unwrap()
    }
}
