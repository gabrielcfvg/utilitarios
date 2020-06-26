use image::{open, ImageBuffer, Rgba, Pixel};
use std::time::Duration;
use std::time::SystemTime;


fn paste(mut img: ImageBuffer<Rgba<u8>, Vec<u8>>, img2: &ImageBuffer<Rgba<u8>, Vec<u8>>, pos: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
 
    for (x, y, pixel) in img2.enumerate_pixels() {

        if pixel[3] == 255 {
            img.put_pixel(x+pos.0, y+pos.1, *pixel)
        }
        else if pixel[3] > 0 {
            let mut or = *img.get_pixel(x+pos.0, y+pos.1);
            or.blend(pixel);
            img.put_pixel(x+pos.0, y+pos.1, or)
        }
    }

    img
}