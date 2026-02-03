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

    let size = match (width as usize)
        .checked_mul(height as usize)
        .and_then(|total| total.checked_mul(4))
    {
        Some(s) => s,
        None => return,
    };

    let c_str = unsafe { CStr::from_ptr(params) };
    let params_str = c_str.to_string_lossy();

    let radius = params_str
        .split(',')
        .find(|s| s.contains("radius"))
        .and_then(|s| s.split('=').last())
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(1);

    if radius <= 0 { return; }

    let src = unsafe { std::slice::from_raw_parts(rgba_data, size) };
    let mut dst = src.to_vec();

    for y in 0..height as i64 {
        for x in 0..width as i64 {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            let mut a: u32 = 0;
            let mut count: u32 = 0;

            let r_long = radius as i64;
            for dy in -r_long..=r_long {
                for dx in -r_long..=r_long {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                        let idx = ((ny * width as i64 + nx) * 4) as usize;
                        r += src[idx] as u32;
                        g += src[idx + 1] as u32;
                        b += src[idx + 2] as u32;
                        a += src[idx + 3] as u32;
                        count += 1;
                    }
                }
            }

            let out_idx = ((y * width as i64 + x) * 4) as usize;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_minimal_buffer_blur() {
        let mut data = vec![0u8; 3 * 3 * 4];
        let center_idx = (1 * 3 + 1) * 4;
        data[center_idx] = 255;
        data[center_idx + 1] = 255;
        data[center_idx + 2] = 255;
        data[center_idx + 3] = 255;

        let params = CString::new("radius=1").unwrap();

        unsafe {
            process_image(3, 3, data.as_mut_ptr(), params.as_ptr());
        }
        assert!(data[center_idx] < 255);
        assert!(data[center_idx] > 0);
    }

    #[test]
    fn test_overflow_protection() {
        let mut data = vec![1, 2, 3, 4];
        let params = CString::new("radius=1").unwrap();
        unsafe {
            process_image(u32::MAX, u32::MAX, data.as_mut_ptr(), params.as_ptr());
        }
        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_invalid_params() {
        let mut data = vec![255; 4];
        let params = CString::new("radius=-5").unwrap();

        unsafe {
            process_image(1, 1, data.as_mut_ptr(), params.as_ptr());
        }
        assert_eq!(data[0], 255);
    }
}