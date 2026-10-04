use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use clap::Parser;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage, imageops};

#[derive(Parser)]
#[command(name = "ASCII filter", about = "Generates an image made of ASCII")]
struct Args {
    input_path: String,

    #[arg(short, long, default_value_t = 1280)]
    width: u32,

    #[arg(short, long, default_value_t = 30.0)]
    contrast: f32,

    #[arg(short, long, default_value_t = 24.0)]
    font_size: f32,
}

fn main() {
    let args = Args::parse();

    let output_path = format!(
        "{}_ascii.png",
        args.input_path.split('.').next().unwrap_or("output")
    );

    let image = match image::open(&args.input_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to load image from {}: {e}", args.input_path);
            return;
        }
    };
    let ascii = match to_ascii(image, args.width, args.contrast, args.font_size) {
        Ok(chars) => chars,
        Err(e) => {
            eprintln!("Failed to convert to ASCII: {e}");
            return;
        }
    };

    match ascii_to_image(&ascii, &output_path, args.font_size) {
        Ok(_) => println!("ASCII image saved successfully as {output_path}"),
        Err(e) => {
            eprintln!("Failed to generate ASCII image: {e}");
            return;
        }
    };
}

fn to_ascii(
    img: DynamicImage,
    width: u32,
    contrast: f32,
    font_size: f32,
) -> Result<String, Box<dyn std::error::Error>> {
    let img = img.adjust_contrast(contrast);
    let font_data = include_bytes!("font.ttf");
    let font = FontRef::try_from_slice(font_data)?;
    let scale = PxScale::from(font_size);

    let scaled_font = font.as_scaled(scale);
    let char_w = scaled_font.h_advance(font.glyph_id('M'));
    let char_h = scaled_font.height();

    let (img_width, img_height) = img.dimensions();

    let aspect_ratio = img_width as f32 / img_height as f32;
    let font_ratio = char_h / char_w;

    let calc_height = (width as f32 / aspect_ratio / font_ratio).round() as u32;

    let ascii_chars = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let resized = img.resize_exact(width, calc_height, imageops::FilterType::Lanczos3);
    let imgbw = resized.to_luma8();

    let mut ascii = String::with_capacity((width * calc_height) as usize);
    for y in 0..calc_height {
        for x in 0..width {
            let pixel = imgbw.get_pixel(x, y);
            let brightness = pixel[0] as f32 / 255.0;
            let char_index = (brightness * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii.push(ascii_chars[char_index] as char);
        }
        ascii.push('\n');
    }
    Ok(ascii)
}

fn ascii_to_image(
    ascii: &str,
    out_path: &str,
    font_size: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let font_data = include_bytes!("font.ttf");
    let font = FontRef::try_from_slice(font_data)?;
    let scale = PxScale::from(font_size);

    let scaled_font = font.as_scaled(scale);
    let char_w = scaled_font.h_advance(font.glyph_id('M'));
    let char_h = scaled_font.height();

    let lines: Vec<&str> = ascii.lines().collect();

    let img_width = (lines[0].len() as f32 * char_w).ceil() as u32;
    let img_height = (lines.len() as f32 * char_h).ceil() as u32;

    let mut out_img = RgbaImage::from_pixel(img_width, img_height, Rgba([0, 0, 0, 255]));
    let text_color = Rgba([255, 255, 255, 255]);

    for (i, line) in lines.iter().enumerate() {
        imageproc::drawing::draw_text_mut(
            &mut out_img,
            text_color,
            0,
            (i as f32 * char_h).round() as i32,
            scale,
            &font,
            line,
        );
    }

    out_img.save(out_path)?;
    Ok(())
}
