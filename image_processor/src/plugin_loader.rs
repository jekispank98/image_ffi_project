use libloading::{Library, Symbol};
use std::ffi::c_char;
use std::path::Path;

type ProcessFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

pub struct Plugin {
    _lib: Library,
    func: Symbol<'static, ProcessFn>,
}

impl Plugin {
    pub fn load(path: &Path) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(path)?;
            let func: Symbol<ProcessFn> = lib.get(b"process_image")?;
            let func = std::mem::transmute::<Symbol<ProcessFn>, Symbol<'static, ProcessFn>>(func);

            Ok(Self { _lib: lib, func })
        }
    }

    pub fn execute(&self, width: u32, height: u32, data: &mut [u8], is_horizontal: *const c_char) {
        unsafe {
            (self.func)(width, height, data.as_mut_ptr(), is_horizontal);
        }
    }
}