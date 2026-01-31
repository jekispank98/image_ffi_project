#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    is_horizontal: bool,
) {
    if rgba_data.is_null() {
        return;
    }

    let pixels = std::slice::from_raw_parts_mut(rgba_data as *mut u32, (width * height) as usize);

    if is_horizontal {
        for y in 0..height {
            for x in 0..(width / 2) {
                let idx1 = (y * width + x) as usize;
                let idx2 = (y * width + (width - 1 - x)) as usize;
                pixels.swap(idx1, idx2);
            }
        }
    } else {
        for y in 0..(height / 2) {
            let row_top = (y * width) as usize;
            let row_bottom = ((height - 1 - y) * width) as usize;
            for x in 0..width as usize {
                pixels.swap(row_top + x, row_bottom + x);
            }
        }
    }
}

fn main() {}
