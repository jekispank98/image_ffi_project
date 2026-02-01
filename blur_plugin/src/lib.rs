use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    if rgba_data.is_null() || params.is_null() { return; }
    
    let c_str = unsafe { CStr::from_ptr(params) };
    let params_str = c_str.to_string_lossy();

    let radius = params_str
        .split(',')
        .find(|s| s.contains("radius"))
        .and_then(|s| s.split('=').last())
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(1);

    if radius <= 0 { return; }
    
    let size = (width * height * 4) as usize;
    let src = unsafe { std::slice::from_raw_parts(rgba_data, size) };
    let mut dst = src.to_vec();
    
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            let mut a: u32 = 0;
            let mut count: u32 = 0;
            
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let idx = ((ny * width as i32 + nx) * 4) as usize;
                        r += src[idx] as u32;
                        g += src[idx + 1] as u32;
                        b += src[idx + 2] as u32;
                        a += src[idx + 3] as u32;
                        count += 1;
                    }
                }
            }
            
            let out_idx = ((y * width as i32 + x) * 4) as usize;
            dst[out_idx] = (r / count) as u8;
            dst[out_idx + 1] = (g / count) as u8;
            dst[out_idx + 2] = (b / count) as u8;
            dst[out_idx + 3] = (a / count) as u8;
        }
    }
    
    unsafe {
        std::ptr::copy_nonoverlapping(dst.as_ptr(), rgba_data, size);
    }
}