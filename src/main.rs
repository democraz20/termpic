use std::env;
//use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel, Rgb};
use term_size;
use colored::*;

#[cfg(target_os = "linux")]
static TEST_FILE_PATH: &str = "../../nufc_wallpaper.png";
#[cfg(target_os = "windows")]
// static  TEST_FILE_PATH: &str = r#"E:\demc\funni\irog.png"#;
static TEST_FILE_PATH: &str = "..\\unsorted\\maxresdefault.jpg";

#[derive(Debug)]
enum ImageLayout {
    Vertical,
    Horizontal,
    Square
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let img = image::open(TEST_FILE_PATH)?;
    let dimensions = img.dimensions();
    
    println!("color     : {:?}", img.color());
    let img = img.to_rgb8();
    
    let ratio = simplify_ratio(dimensions.0, dimensions.1);
    //let ratio = simplify_ratio(800, 200);
    let layout = get_layout(ratio)?;
    
    //let (rows, columns) = 
    let (r_rows, r_cols) = match term_size::dimensions() {
        Some(r) => { (r.0, r.1) },
        None => {
            eprintln!("ERROR : Unable to get terminal size");
            eprintln!("ERROR : Try specifying the size with --termsize or --size flags");
            std::process::exit(1);
        }
    };
    
    //this is because 2x1 terminal cell looks closest to a square, so we shrink the columns by 2 and when we render we x2 columns so it fills the screen
    //the dc_ means default converted
    //let (dc_rows, dc_cols) = (rows, cols/2);
    let (dc_rows, dc_cols): (u32, u32) = (r_rows.try_into()?, r_cols.try_into()?);

    let minimized = minimize_ratio((dimensions.0, dimensions.1), (r_rows.try_into()?, r_cols.try_into()?), true);

    
    let resize = image::imageops::resize(&img, dc_rows, dc_cols, image::imageops::FilterType::Gaussian);
    //let resize = image::DynamicImage::resize_exact(&img, ratio.0, (ratio.1)*2, image::imageops::FilterType::Lanczos3);
    //let resize = image::DynamicImage::resize_exact(&img, dc_rows as u32, dc_cols as u32, image::imageops::FilterType::Lanczos3);
    //resize.save("resized.jpg")?;
    
    let mut rgb_vec: Vec<Vec<(u8, u8,u8)>> = vec![]; //could preallocate this since size is known (at runtime)
    //populate yaxis //this is probably wrong
    for _ in 0..dc_cols {
        rgb_vec.push(vec![])
    }
    println!("dimensions: {:?}", dimensions);
    println!("ratio     : {:?}", ratio);
    println!("layout    : {:?}", layout);
    println!("term size : {}x{}", r_rows, r_cols);
    println!("resized   : {}x{}", dc_rows, dc_cols);
    println!("shrunk    : {}x{}", minimized.0, minimized.1);
    //printing
    for x in 0..dc_rows {
        for y in 0..dc_cols {
            let p = resize.get_pixel(x, y);
            let rgb = p.to_rgb();
            rgb_vec[y as usize].push((rgb.0[0],rgb.0[1],rgb.0[2]));
        }
    }

    for y in rgb_vec {
        for x in y {
            print!("{}", " ".on_truecolor(x.0, x.1, x.2));
        }
        println!();
    }

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

fn minimize_ratio(img: (u32, u32), term: (u32, u32), prio_width: bool) -> (u32, u32) {
    let aspect_ratio = img.0 as f64 / img.1 as f64;
    println!("aspectratio: {}", aspect_ratio);
    if prio_width {
        let new_height = (term.1 as f64 * aspect_ratio).floor() as u32;
        return (term.0, new_height)
    } else {
        let new_width = (term.0 as f64 * aspect_ratio).floor() as u32;
        return (new_width, term.1)
    }
}

fn get_layout(imageratio: (u32, u32)) -> Result<ImageLayout, String> {
    if imageratio.0 == imageratio.1 {
        return Ok(ImageLayout::Square)
    } else if imageratio.0 > imageratio.1 {
        return Ok(ImageLayout::Horizontal)
    } else if imageratio.0 < imageratio.1 {
        return Ok(ImageLayout::Vertical)
    } else { //if SOMEHOW
        // return Err(Box::new(format!("Unable to get image layout from ratio : {:?}", imageratio)));
        return Err(format!("Unable to get image layout from ratio : {:?}", imageratio))
    }
}
