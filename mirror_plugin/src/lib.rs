use std::ffi::{c_char, CStr};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    if rgba_data.is_null() || params.is_null() {
        return;
    }

    let total_pixels = match (width as usize).checked_mul(height as usize) {
        Some(val) => val,
        None => return,
    };

    let total_bytes = match total_pixels.checked_mul(4) {
        Some(val) => val,
        None => return,
    };

    let c_str = unsafe { CStr::from_ptr(params) };
    let params_str = c_str.to_string_lossy();
    let is_horizontal = params_str.contains("horizontal");

    let pixels = unsafe { std::slice::from_raw_parts_mut(rgba_data, total_bytes) };

    let w = width as usize;
    let h = height as usize;

    if is_horizontal {
        for y in 0..h {
            for x in 0..(w / 2) {
                let idx1 = (y * w + x) * 4;
                let idx2 = (y * w + (w - 1 - x)) * 4;
                for b in 0..4 {
                    pixels.swap(idx1 + b, idx2 + b);
                }
            }
        }
    } else {
        for y in 0..(h / 2) {
            let row_top_start = (y * w) * 4;
            let row_bottom_start = ((h - 1 - y) * w) * 4;
            for x in 0..w {
                let idx1 = row_top_start + x * 4;
                let idx2 = row_bottom_start + x * 4;
                for b in 0..4 {
                    pixels.swap(idx1 + b, idx2 + b);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_vertical_mirror_minimal() {
        let mut data = vec![1, 1, 1, 255, 2, 2, 2, 255, 3, 3, 3, 255];
        let params = CString::new("vertical").unwrap();

        unsafe {
            process_image(1, 3, data.as_mut_ptr(), params.as_ptr());
        }

        let expected = vec![3, 3, 3, 255, 2, 2, 2, 255, 1, 1, 1, 255];
        assert_eq!(data, expected);
    }

    #[test]
    fn test_horizontal_mirror_minimal() {
        let mut data = vec![10, 10, 10, 255, 20, 20, 20, 255];
        let params = CString::new("horizontal").unwrap();

        unsafe {
            process_image(2, 1, data.as_mut_ptr(), params.as_ptr());
        }

        let expected = vec![20, 20, 20, 255, 10, 10, 10, 255];
        assert_eq!(data, expected);
    }

    #[test]
    fn test_overflow_protection() {
        let mut data = vec![0u8; 4];
        let params = CString::new("").unwrap();

        unsafe {
            process_image(u32::MAX, u32::MAX, data.as_mut_ptr(), params.as_ptr());
        }
    }

    #[test]
    fn test_large_dimension_iteration() {
        let mut data = vec![1, 2, 3, 4];
        let params = CString::new("vertical").unwrap();
        unsafe {
            process_image(1, 1, data.as_mut_ptr(), params.as_ptr());
        }
        assert_eq!(data, vec![1, 2, 3, 4]);
    }
}
