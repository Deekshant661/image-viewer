use image::{ImageBuffer,Rgb};
use image::imageops;
use std::fmt::Error;
use std::io::Cursor;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    //Creating Image 1 : creating and manipulating image directly in memory
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(300,300);

    //Draw a red square in the middle
    for x in 100..200{
        for y in 100..200{
            img.put_pixel(x,y,Rgb([255_u8,0_u8,0_u8]));
        }
    }
    img.save("red_square.png")?;
    println!("created and saved red square image");

    //Resizing Image: Loading from file and modifying

    if let Ok(existing_image) = image::open("red_square.png"){
        let resized = imageops::resize(
            &existing_image,
            existing_image.width() / 2,
            existing_image.height() / 2,
            imageops::FilterType::Lanczos3,
        );
        resized.save("req_square_small.png")?;
        println!("We have created resized version : red_square_small.png");
    }

    //Load Image 2 : use file to memory loading
    //we first read the file onto memory
    let file_contents = std::fs::read("red_square.png")?;
    let img_from_memory = image::load_from_memory(&file_contents)?;

    //create a blue version using memory loaded image
    let mut blue_img = img_from_memory.to_rgb8();
    for pixel in blue_img.pixels_mut(){
        *pixel = Rgb([0_u8,0_u8,255_u8]);
    }
    blue_img.save("blue_square.png")?;
    println!("Created blue version using memory loading");

    //Manipulating image 3 using Cursor for streaming like operations

    let cursor = Cursor::new(file_contents);
    let img_from_cursor = image::load(cursor,image::ImageFormat::Png)?;

    //creating composite image using cursor-loaded image
    let mut composite = img_from_cursor.to_rgb8();

    //Add a green diagonal line
    for i in 0..300{
        if i < composite.width() && i < composite.height(){
            composite.put_pixel(i,i,Rgb([0_u8,255_u8,0_u8]));
        }
    }
    composite.save("composite_square.png")?;
    println!("Created composite image using cursor approach");

    //printing dimensions of all:
    println!("Original : {}*{}",img.width(),img.height());
    println!("Memory - loaded : {}*{}",img_from_memory.width(),img_from_memory.height());
    println!("Cursor - loaded : {}*{}",img_from_cursor.width(),img_from_cursor.height());

    Ok(())

}