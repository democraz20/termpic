use std::env;
//use image::io::Reader as ImageReader;
use image::GenericImageView;
use term_size;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let img = image::open("../../nufc_wallpaper.png")?;
    let dimensions = img.dimensions();
    println!("dimensions: {:?}", dimensions);
    println!("color     : {:?}", img.color());
    
    let ratio = simplify_ratio(dimensions.0, dimensions.1);
    //let ratio = simplify_ratio(800, 200);
    println!("ratio     : {:?}", ratio);
    //let (rows, columns) = 
    let (rows, cols) = match term_size::dimensions() {
        Some(r) => { (r.0, r.1) },
        None => {
            eprintln!("ERROR : Unable to get terminal size");
            eprintln!("ERROR : Try specifying the size with --termsize or --size flags");
            std::process::exit(1);
        }
    };
    
    println!("term size : {}x{}", rows, cols);
    Ok(())
}

fn simplify_ratio(width: u32, height: u32) -> (u32, u32) {
    let mut a = width;
    let mut b = height;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    (width / a, height / a)
}
