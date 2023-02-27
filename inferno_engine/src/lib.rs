//! # Inferno Engine
//!
//! Easy to use, rapid prototype engine written in Rust!
pub mod line;
pub mod reload;
pub mod shaders;
pub mod window;

/// Draw an object using it's handle.
/// # Example
///
/// ```
/// engine_draw(0);
/// ```
pub fn engine_draw(handle: u32) {
    println!("Drawing resource: {}", handle);
}
