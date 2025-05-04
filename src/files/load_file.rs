
use std::path::Path;
fn main() {
    let image_path = "C:/Users/deeks/OneDrive/Documents/Rust_projects/lifetime.png";
    
    // Print current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    
    // Checking if file exists and is readable
    let path = Path::new(image_path);
    println!("Image path: {:?}", path.canonicalize().unwrap());
    
    // Try to read the file content first
    let img_result = image::open(path);
    match img_result {
        Ok(img) => {
            println!("Image loaded successfully!");
            println!("Dimensions: {}x{}", img.width(), img.height());
        },
        Err(e) => {
            println!("Failed to load image: {:?}", e);
        }
    }
}


