fn main(){
    //Load image from file
    let img = image::open("C:/Users/deeks/OneDrive/Documents/Rust_projects/lifetime.png").unwrap();

    //Print the dimensions
    println!("Dimensions : {}x{}",img.width(),img.height());

    match img {
        Ok(img) => {
            println!("Image loaded successfully!");
            println!("Dimensions: {}x{}", img.width(), img.height());
        },
        Err(e) => {
            println!("Failed to load image: {:?}", e);
        }
    }
    
    //Save image
    img.save("output.jpeg").unwrap();
}

