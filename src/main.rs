// This was all a test to get more practice in rust!
// The website where this code was found at is below
// https://dev.to/abhishek2010devsingh/ascii-art-code-in-rust-48hn#:~:text=Full%20code,%7B%20for%20x%20in%200..
// I take no credit in making any of this, all I added was more comments based on what the article explained
use clap::Parser;
use image::{DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ImageValue{
    // Path of the Image to convert
    #[arg(short, long)]
    path: String,

    // Scale factor for ASCII art (Default is 3)
    #[arg(short, long, default_value_t = 3)]
    scale: u32,
}

fn get_str_ascii(intent: u8) -> &'static str {
    let index: u8 = intent / 32;
    let ascii: [&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32){
    // Opens the image at the path specified
    // TODO: .unwrap() panics if the image isn't there, maybe make a proper error system
    let img: DynamicImage = image::open(dir).unwrap();
    let (width, height) = img.dimensions();
    for y in 0..height{
        for x in 0..width{
            // Checks if the pixel should be sampled based on current image dimensions
            // Reduces the image resolution by skipping pixels
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                // Gets a grayscale intesity value from the three RGB values present
                let mut intent: u8 = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                // Checks if the pixel is transparent and sets the intensity to 0 if true
                if pix[3] == 0{
                    intent = 0;
                }
               // Prints the ASCII charecter based on the given intentsity value
               print!("{}", get_str_ascii(intent));
            }
        }
        // Prints a new line every scale * 2 rows to make sure everything lines up good
        if y % (scale * 2) == 0{
            println!("");
        }
    }
}
fn main() {
    let value = ImageValue::parse();
    get_image(&value.path, value.scale);
}