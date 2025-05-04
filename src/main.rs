use std::default;

use image::DynamicImage;
use minifb::{Key,Window,WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut img = image::open("C:/Users/deeks/OneDrive/Documents/Rust_projects/yt_image-viewer/src/4direc.jpeg")?;
    let mut curr_rotation = 0;
    

    //Creating a window with dimensions matching the image
    let mut window = Window::new(
        "Image viewer",
        img.width() as usize,
        img.height() as usize, 
        WindowOptions { 
            resize: true, 
            ..WindowOptions::default()
        },
    )?;

    //Set up buffer for window display
    let mut buff = vec![0; img.width() as usize * img.height() as usize];

    //Controls
    println!("Controls");
    println!("'R' ->  Rotate image clockwise");
    println!("'L' ->  Rotate image counter-clockwise");
    println!("'S' ->  Save the current state of image");
    println!("'Q' ->  To Quit the image viewer");

    //Update buffer with image data
    update_buffer(&img, &mut buff);

    while window.is_open() && !window.is_key_down(Key::Q){

        //Handle rotation controls
        if window.is_key_pressed(Key::R ,minifb::KeyRepeat::No){
            img = img.rotate90();
            curr_rotation = (curr_rotation + 90) % 360;
            

            //Create new buffer with rotated dimensions
            buff = vec![0; img.width() as usize * img.height() as usize];
            update_buffer(&img, &mut buff);
        }

        if window.is_key_pressed(Key::L,minifb::KeyRepeat::No){
            img = img.rotate270();
            curr_rotation = (curr_rotation - 90 + 360) % 360;

            //new buffer
            buff = vec![0; img.width() as usize * img.height() as usize];
            update_buffer(&img, &mut buff);
        }

        if window.is_key_pressed(Key::S,minifb::KeyRepeat::No){
            let filename = format!("rotated_{}_degrees.png",curr_rotation);
            img.save(&filename)?;
            println!("saved image as {}",filename);
        }

        //update window with current buffer
        window.update_with_buffer(&buff, img.width()as usize, img.height() as usize);

    }

    Ok(())
}


//Function to update the buffer with image data

fn update_buffer(img: &DynamicImage, buff: &mut Vec<u32>){

    let rgba_img = img.to_rgba8();
    
    for (i,pixel) in rgba_img.pixels().enumerate() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let a = pixel[3] as u32;

        buff[i] = (a<<24) | (r<<16) | (g<<8) | b;
    }

    //Converting RGBA into a single u32 value (ARGB format for minifb)
    

}
