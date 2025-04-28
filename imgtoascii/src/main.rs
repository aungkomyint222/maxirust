use image::open;
use hex;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if path argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-image>", args[0]);
        std::process::exit(1);
    }
    
    // Use the first argument as image path
    let img_path = &args[1];
    let img = open(img_path)?;
    
    // Convert to RGB8 format
    let rgb_img = img.to_rgb8();
    
    // Get dimensions before converting to raw pixels
    let (width, height) = rgb_img.dimensions();
    
    // Clone rgb_img before converting to raw pixels
    let raw_pixels = rgb_img.clone().into_raw();
    
    // Convert pixel data to hexadecimal (optional, for reference)
    let hex_string = hex::encode(&raw_pixels);
    println!("Hex representation (first 100 chars): {:.100}", hex_string);
    
    // Define ASCII characters from dark to light (denser = darker)
    let ascii_chars = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    //let ascii_chars = ['M', 'a', 'x', 'i', 'm', 'u', 's', 'a', 'u', 'n', 'g'];
    
    // Terminal characters are typically about twice as tall as they are wide
    // This factor helps correct the aspect ratio - adjusted to make image wider
    let term_aspect_ratio = 2.0; // Increased to make the image wider
    
    // Define target dimensions (adjusted for better aspect ratio)
    let max_width = 150; // Increased width
    let max_height = 40; // Maintained height
    
    // Calculate target dimensions while preserving aspect ratio
    let image_aspect = (width as f32) / (height as f32);
    
    // Calculate dimensions with correction for terminal character aspect
    let (scaled_width, scaled_height) = if (max_width as f32 / max_height as f32) > (image_aspect * term_aspect_ratio) {
        // Height constrained
        let h = max_height;
        let w = ((max_height as f32 * image_aspect) * term_aspect_ratio) as u32;
        (w, h)
    } else {
        // Width constrained
        let w = max_width;
        let h = ((max_width as f32 / image_aspect) / term_aspect_ratio) as u32;
        (w, h)
    };
    
    println!("ASCII art dimensions: {}x{}", scaled_width, scaled_height);
    
    // Process each pixel row
    for y in 0..scaled_height {
        for x in 0..scaled_width {
            // Sample pixel at scaled coordinates
            let src_x = (x as f32 * width as f32 / scaled_width as f32) as u32;
            let src_y = (y as f32 * height as f32 / scaled_height as f32) as u32;
            
            // Ensure coordinates are within bounds
            let src_x = src_x.min(width - 1);
            let src_y = src_y.min(height - 1);
            
            let pixel = rgb_img.get_pixel(src_x, src_y);
            
            // Convert RGB to grayscale intensity (luminance formula)
            let intensity = (0.299 * pixel[0] as f32 +
                            0.587 * pixel[1] as f32 +
                            0.114 * pixel[2] as f32) as u8;
            
            // Map intensity (0-255) to an ASCII character (inverted)
            let ascii_idx = ((ascii_chars.len() - 1) as f32 * (1.0 - intensity as f32 / 255.0)) as usize;
            let ascii_char = ascii_chars[ascii_idx];
            
            // Print the ASCII character (no newline)
            print!("{}", ascii_char);
        }
        // Newline after each row
        println!();
    }
    
    Ok(())
}