use windows::System::DispatcherQueueController;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DispatcherQueueOptions, DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
    DISPATCHERQUEUE_THREAD_TYPE, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT,
};

use windows::core::Result;

pub trait DispatcherQueueControllerExtensions {
    fn create_dispatcher_queue_controller(
        thread_type: DISPATCHERQUEUE_THREAD_TYPE,
        apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
    ) -> Result<DispatcherQueueController>;
    fn create_dispatcher_queue_controller_for_current_thread() -> Result<DispatcherQueueController>;
}

impl DispatcherQueueControllerExtensions for DispatcherQueueController {
    fn create_dispatcher_queue_controller(
        thread_type: DISPATCHERQUEUE_THREAD_TYPE,
        apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
    ) -> Result<DispatcherQueueController> {
        let options = DispatcherQueueOptions {
            dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
            threadType: thread_type,
            apartmentType: apartment_type,
        };
        let controller = unsafe { CreateDispatcherQueueController(options)? };
        Ok(controller)
    }

    fn create_dispatcher_queue_controller_for_current_thread() -> Result<DispatcherQueueController>
    {
        Self::create_dispatcher_queue_controller(DQTYPE_THREAD_CURRENT, DQTAT_COM_NONE)
    }
}
