use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn draw_pixels(
    width: u32,
    height: u32,
    pixels: &mut [u8],
    param1: f32,
    param2: f32,
    time: f32,
) {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    for y in 0..height {
        for x in 0..width {
            let i = ((x + y * width) * 4) as usize;
            let x = x as f32;
            let y = y as f32;

            let dx = x - cx;
            let dy = y - cy;

            let d = (dx * dx + dy * dy).sqrt() * (param2 * 0.1 + 1.0) * 0.002;

            let ripple = (d - time * 0.0001 * param1).rem_euclid(1.0);

            let color2 = [0.49, 0.98, 0.64];
            let color1 = [0.05, 0.04, 0.06];

            let c = lerp_color(color1, color2, ripple * ripple);

            pixels[i] = (c[0] * 256.0) as u8;
            pixels[i + 1] = (c[1] * 256.0) as u8;
            pixels[i + 2] = (c[2] * 256.0) as u8;
            pixels[i + 3] = 255;
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn lerp_color(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
        lerp(a[2], b[2], t),
    ]
}
