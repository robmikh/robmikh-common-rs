#[cfg(feature = "desktop")]
pub mod desktop;
#[cfg(feature = "universal")]
pub mod universal;

#[cfg(test)]
mod tests {
    use windows::{
        core::Result,
        Foundation::Numerics::Vector2,
        Graphics::{
            DirectX::{DirectXAlphaMode, DirectXPixelFormat},
            SizeInt32,
        },
        System::DispatcherQueueController,
        Win32::{
            Foundation::SIZE,
            Graphics::{Direct3D11::ID3D11Texture2D, Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM},
            System::WinRT::{RoInitialize, RO_INIT_MULTITHREADED},
        },
        UI::Composition::Core::CompositorController,
    };

    use crate::{
        desktop::dispatcher_queue::DispatcherQueueControllerExtensions,
        universal::{
            composition::{
                CompositionDrawingSurfaceInterop, CompositionSurfaceDrawingSession,
                CompositorInterop,
            },
            d3d::{create_d3d_device, create_dxgi_swap_chain_default},
        },
    };

    // Make sure you run test with --all-features
    //#[cfg(all(feature = "dispatcher-queue-desktop", feature = "composition", feature = "d3d", feature = "d3d-debug", ))]
    #[test]
    fn composition_smoke_test() -> Result<()> {
        unsafe { RoInitialize(RO_INIT_MULTITHREADED)? };
        let _controller =
            DispatcherQueueController::create_dispatcher_queue_controller_for_current_thread()?;
        // We're not going to pump messages, so we'll be calling commit ourselves.
        // Note that because of this, we don't receive callbacks.
        let compositor_controller = CompositorController::new()?;
        let compositor = compositor_controller.Compositor()?;
        let d3d_device = create_d3d_device()?;
        let d3d_context = {
            let mut d3d_context = None;
            unsafe { d3d_device.GetImmediateContext(&mut d3d_context) };
            d3d_context.unwrap()
        };

        // Create and clear a surface
        let comp_graphics = compositor.create_graphics_device_from_d3d_device(&d3d_device)?;
        let surface = comp_graphics.CreateDrawingSurface2(
            SizeInt32 {
                Width: 1,
                Height: 1,
            },
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            DirectXAlphaMode::Premultiplied,
        )?;
        surface.resize(&SIZE { cx: 250, cy: 250 })?;
        {
            let session = CompositionSurfaceDrawingSession::<ID3D11Texture2D>::new(surface)?;
            let update_object = session.update_object();
            let render_target_view =
                unsafe { d3d_device.CreateRenderTargetView(update_object, std::ptr::null())? };
            unsafe {
                d3d_context
                    .ClearRenderTargetView(render_target_view, [1.0, 0.0, 0.0, 1.0].as_ptr());
            }
        }
        compositor_controller.Commit()?;

        // Create a swap chain and get a composition surface for it
        let swap_chain =
            create_dxgi_swap_chain_default(&d3d_device, 800, 600, DXGI_FORMAT_B8G8R8A8_UNORM, 2)?;
        let swap_chain_surface =
            compositor.create_composition_surface_for_swap_chain(&swap_chain)?;
        unsafe {
            let back_buffer: ID3D11Texture2D = swap_chain.GetBuffer(0)?;
            let render_target_view =
                d3d_device.CreateRenderTargetView(&back_buffer, std::ptr::null())?;
            d3d_context.ClearRenderTargetView(render_target_view, [0.0, 1.0, 0.0, 1.0].as_ptr());
        }
        let swap_chain_visual = compositor.CreateSpriteVisual()?;
        swap_chain_visual.SetSize(Vector2::new(800.0, 600.0))?;
        let swap_chain_brush = compositor.CreateSurfaceBrushWithSurface(swap_chain_surface)?;
        swap_chain_visual.SetBrush(swap_chain_brush)?;
        compositor_controller.Commit()?;
        Ok(())
    }
}
