use image::io::Reader as ImageReader;
use std::fs;
use std::io::Cursor;


fn main() -> Result<(), image::ImageError> {
    //Creating a directory to store images ( in this case, the current working directory)
    fs::create_dir_all("./").map_err(|e| {
        println!("Error creating directory: {}",e);
        return e;
    })?;


    //Method 1 : Loading Image from a file
    let img = ImageReader::open("C:/Users/deeks/OneDrive/Documents/Rust_projects/yt_image-viewer/src/sol.png")?.decode()?;
    println!("First image loaded from a file successfully");
    println!("Image Dimensions: {}x{}",img.width(),img.height());

    //Method 2: Loading Image from Memory
    let image_bytes= include_bytes!("C:/Users/deeks/OneDrive/Documents/Rust_projects/yt_image-viewer/src/sol.png");
    let img2 = ImageReader::new(Cursor::new(image_bytes))
    .with_guessed_format()?
    .decode()?;
    println!("Second image loaded from a file successfully");
    println!("Image Dimensions: {}x{}",img.width(),img.height());

    //Save both images to verify they loaded and saved correctly
    img.save("./img_file.png")?;
    img2.save("./img_memory.png")?;
    println!("Both images saved successfully.");

    Ok(())
}