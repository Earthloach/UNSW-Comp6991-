use std::path::Path;
use bmp::{Image, Pixel};

pub fn load_or_new_bmp(path: &str, width: u32, height: u32) -> Result<Image, String> {
 if Path::new(path).exists() {
        // File exists, try to load it as BMP
        match bmp::open(path) {
            Ok(img) => Ok(img),
            Err(_) => Err(format!("Error: File {} exists but is not a valid BMP", path))
        }
    } else {
        // File doesn't exist, create a new image
        Ok(Image::new(width, height))
    }   
}

pub fn draw_pixel(path: &str) {
    match load_or_new_bmp(path, 100, 100) {
        Ok(mut image) => {
                image.set_pixel(50, 50, Pixel::new(255, 255, 255));
                image.save(path).expect("Failed to save the image");
                println!("Successfully drew pixel to {}", path);
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn draw_diagonal(path: &str) {
    match load_or_new_bmp(path, 100, 100) {
        Ok(mut image) => {
            // Check if image is square
            if image.get_height() != image.get_width() {
                eprintln!("Error: Canvas must be square for diagonal operation");
                return;
            }
            
            let size = image.get_width();
            for i in 0..size {
                image.set_pixel(i, i, Pixel::new(0, 255, 0));
            }
            
            image.save(path).expect("Failed to save the image");
            println!("Successfully drew diagonal to {}", path);
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn draw_x(path: &str) {
    match load_or_new_bmp(path, 100, 100) {
        Ok(mut image) => {
            let width = image.get_width();
            let height = image.get_height();
            
            // Draw diagonal from top-left to bottom-right
            for i in 0..width {
                let y = (i * height) / width;
                image.set_pixel(i, y, Pixel::new(255, 0, 0)); // Red line
            }
            
            // Draw diagonal from bottom-left to top-right
            for i in 0..width {
                let y = height - 1 - (i * height) / width;
                image.set_pixel(i, y, Pixel::new(255, 0, 0)); // Red line
            }
            
            image.save(path).expect("Failed to save the image");
            println!("Successfully drew X shape to {}", path);
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn draw_house(path: &str) {
    match load_or_new_bmp(path, 100, 100) {
        Ok(mut image) => {
            let width = image.get_width();
            let height = image.get_height();
            let color = Pixel::new(0, 0, 255); // Blue house
            
            // Calculate dimensions for the house
            let house_width = width * 2 / 3;
            let house_height = height * 2 / 3;
            let house_x = (width - house_width) / 2;
            let house_y = height - house_height;
            
            // Draw the square base
            for x in house_x..(house_x + house_width) {
                // Draw horizontal lines
                image.set_pixel(x, house_y, color);
                image.set_pixel(x, height - 1, color);
            }
            
            for y in house_y..height {
                // Draw vertical lines
                image.set_pixel(house_x, y, color);
                image.set_pixel(house_x + house_width - 1, y, color);
            }
            
            // Draw the triangular roof
            let roof_top_y = house_y - house_height / 3;
            
            // Draw lines from the top of the roof to each side
            for x in house_x..(house_x + house_width) {
                let progress = (x - house_x) as f32 / (house_width - 1) as f32;
                let y = house_y as f32 - progress * (1.0 - progress) * 4.0 * (house_y - roof_top_y) as f32;
                image.set_pixel(x, y as u32, color);
            }
            
            image.save(path).expect("Failed to save the image");
            println!("Successfully drew house to {}", path);
        },
        Err(e) => eprintln!("{}", e),
    }
}
