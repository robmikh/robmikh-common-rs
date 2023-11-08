use windows::core::{ComInterface, Interface, Result};
use windows::Win32::Foundation::E_NOINTERFACE;

pub trait TryCast {
    fn try_cast<T: Interface + ComInterface>(&self) -> Result<Option<T>>;
}

impl<Base: Interface + ComInterface> TryCast for Base {
    fn try_cast<T: Interface + ComInterface>(&self) -> Result<Option<T>> {
        let mut result = None;
        let code = unsafe { self.query(&T::IID, &mut result as *mut _ as _) };
        if code == E_NOINTERFACE {
            Ok(None)
        } else if code.is_ok() {
            Ok(result)
        } else {
            code.ok()?;
            unreachable!()
        }
    }
}
