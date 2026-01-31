use libloading::{Library, Symbol};
use std::path::Path;
use std::ffi::CString;

type ProcessFn = unsafe extern "C" fn(u32, u32, *mut u8, bool);

pub struct Plugin {
    _lib: Library,
    func: Symbol<'static, ProcessFn>,
}

impl Plugin {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path)?;
            // Загружаем символ и "забываем" о лайфтайме библиотеки внутри структуры
            let func: Symbol<ProcessFn> = lib.get(b"process_image")?;
            let func = std::mem::transmute::<Symbol<ProcessFn>, Symbol<'static, ProcessFn>>(func);

            Ok(Self { _lib: lib, func })
        }
    }

    pub fn execute(&self, width: u32, height: u32, data: &mut [u8], is_horizontal: bool) {
        unsafe {
            (self.func)(width, height, data.as_mut_ptr(), is_horizontal);
        }
    }
}