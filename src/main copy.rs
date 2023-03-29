extern crate image;

use image::{GenericImageView, ImageBuffer, Rgba, Rgb};

fn main() {
    // Load the blurred image
    let img = image::open("blurred_image.jpg").unwrap();

    // Resize the image to 2000 by 2000
    let resized_img = img.resize(2000, 2000, image::imageops::FilterType::Lanczos3);

    // Create a new image buffer with the same dimensions as the resized image
    let mut bright_img = ImageBuffer::new(resized_img.width(), resized_img.height());

    // Loop through each pixel in the resized image and make it brighter
    for (x, y, pixel) in resized_img.pixels() {
        let new_rgb = Rgb([
            clamp(pixel[0] as f32 * 1.2, 0.0, 255.0) as u8,  // increase red channel by 20%
            clamp(pixel[1] as f32 * 1.2, 0.0, 255.0) as u8,  // increase green channel by 20%
            clamp(pixel[2] as f32 * 1.2, 0.0, 255.0) as u8,  // increase blue channel by 20%
        ]);
        bright_img.put_pixel(x, y, new_rgb);
    }

    // Save the brighter image to a file
    bright_img.save("brighter_image.jpg").unwrap();
}

// Helper function to clamp a value between a minimum and maximum
fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}
