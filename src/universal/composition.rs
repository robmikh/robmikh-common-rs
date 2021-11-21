use windows::{
    core::{IUnknown, Interface, Result},
    Win32::{
        Foundation::{POINT, RECT, SIZE},
        System::WinRT::Composition::{ICompositionDrawingSurfaceInterop, ICompositorInterop},
    },
    UI::Composition::{
        CompositionDrawingSurface, CompositionGraphicsDevice, Compositor, ICompositionSurface,
    },
};

pub trait CompositorInterop {
    fn create_graphics_device(&self, device: &IUnknown) -> Result<CompositionGraphicsDevice>;
    fn create_composition_surface_for_swap_chain(
        &self,
        swap_chain: &IUnknown,
    ) -> Result<ICompositionSurface>;
}

impl CompositorInterop for Compositor {
    fn create_graphics_device(&self, device: &IUnknown) -> Result<CompositionGraphicsDevice> {
        let interop: ICompositorInterop = self.cast()?;
        unsafe { interop.CreateGraphicsDevice(device) }
    }

    fn create_composition_surface_for_swap_chain(
        &self,
        swap_chain: &IUnknown,
    ) -> Result<ICompositionSurface> {
        let interop: ICompositorInterop = self.cast()?;
        unsafe { interop.CreateCompositionSurfaceForSwapChain(swap_chain) }
    }
}

pub trait CompositionDrawingSurfaceInterop {
    fn resize(&self, size: &SIZE) -> Result<()>;
    fn begin_draw<T: Interface>(&self, update_rect: Option<&RECT>) -> Result<(T, POINT)>;
    fn end_draw(&self) -> Result<()>;
}

impl CompositionDrawingSurfaceInterop for CompositionDrawingSurface {
    fn resize(&self, size: &SIZE) -> Result<()> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        unsafe { interop.Resize(size) }
    }

    fn begin_draw<UpdateObject: Interface>(
        &self,
        update_rect: Option<&RECT>,
    ) -> Result<(UpdateObject, POINT)> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        let update_rect = if let Some(update_rect) = update_rect {
            update_rect as *const _
        } else {
            std::ptr::null()
        };
        unsafe {
            let mut update_object = None;
            let mut update_offset = POINT::default();
            interop.BeginDraw(
                update_rect,
                &UpdateObject::IID,
                &mut update_object as *mut _ as *mut *mut _,
                &mut update_offset,
            )?;
            Ok((update_object.unwrap(), update_offset))
        }
    }

    fn end_draw(&self) -> Result<()> {
        let interop: ICompositionDrawingSurfaceInterop = self.cast()?;
        unsafe { interop.EndDraw() }
    }
}

pub struct CompositionSurfaceDrawingSession<UpdateObject: Interface> {
    surface: CompositionDrawingSurface,
    update_object: UpdateObject,
    update_offset: POINT,
}

impl<UpdateObject: Interface> CompositionSurfaceDrawingSession<UpdateObject> {
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

impl<UpdateObject: Interface> Drop for CompositionSurfaceDrawingSession<UpdateObject> {
    fn drop(&mut self) {
        self.surface.end_draw().unwrap()
    }
}