use std::io::Cursor;

use glium::texture::SrgbTexture2d;

pub fn get_texture(display: &glium::Display) -> SrgbTexture2d{ 
    let image = image::load(Cursor::new(&include_bytes!("../assets/dirt.png")), image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    return glium::texture::SrgbTexture2d::new(display, image).unwrap();
}