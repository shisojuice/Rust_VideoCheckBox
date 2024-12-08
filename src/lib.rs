use js_sys::Array;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ascii_filter(buffer: Vec<u8>, canvas_width: u32, canvas_height: u32, dot_size: u32) -> Array {
    let width = canvas_width as usize;
    let height = canvas_height as usize;
    let dot_size = dot_size as usize;

    // グレースケール化
    let mut new_buffer = vec![0; buffer.len()];
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let index = (y * width + x) * 4;
            let r = buffer[index] as f32;
            let g = buffer[index + 1] as f32;
            let b = buffer[index + 2] as f32;
            let gray = (r * 0.299 + g * 0.587 + b * 0.114) as u8; // 相対輝度の計算式
            new_buffer[index] = gray;
            new_buffer[index + 1] = gray;
            new_buffer[index + 2] = gray;
            new_buffer[index + 3] = buffer[index + 3];
        }
    }

    let result_array = Array::new();
    for y in (0..height).step_by(dot_size) {
        for x in (0..width).step_by(dot_size) {
            let cell = extract_cell(&new_buffer, x, y, width, height, dot_size);
            let recognized_bool = analyze_cell(&cell);
            result_array.push(&JsValue::from_bool(recognized_bool));
        }
    }
    result_array
}

fn extract_cell(buffer: &[u8], x: usize, y: usize, width: usize, height: usize, dot_size: usize) -> Vec<u8> {
    let mut cell = Vec::new();
    for dy in 0..dot_size {
        for dx in 0..dot_size {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                let index = (py * width + px) * 4;
                if index + 4 <= buffer.len() {
                    cell.extend_from_slice(&buffer[index..index + 4]);
                } else {
                    // バッファの範囲外の場合は、透明ピクセルを追加
                    cell.extend_from_slice(&[0, 0, 0, 0]);
                }
            }
        }
    }
    cell
}

fn analyze_cell(cell: &[u8]) -> bool {
    let black_pixels = cell.chunks(4)
        .filter(|p| p[0] < 128 && p[1] < 128 && p[2] < 128 && p[3] > 0)
        .count();

    let total_pixels = cell.len() / 4;
    let black_ratio = black_pixels as f32 / total_pixels as f32;

    if black_ratio > 0.4 {
        true
    } else {
        false
    }
}