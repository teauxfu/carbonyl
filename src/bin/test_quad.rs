use std::{
    fs::File,
    io::{self, Write},
};

use carbonyl::{gfx::Color, output::quad};
use image::{io::Reader as ImageReader, EncodableLayout};

fn main() -> io::Result<()> {
    println!("Opening image..");
    let img = ImageReader::open("test.png")?.decode().unwrap();
    let scaled = img.resize(
        img.width() / 8,
        img.height() / 8,
        image::imageops::FilterType::Lanczos3,
    );
    let bytes = scaled.as_bytes();

    // let decoder = png::Decoder::new(File::open("test.png")?);
    // let mut reader = decoder.read_info()?;
    // let mut buf = vec![0; reader.output_buffer_size()];
    // let info = reader.next_frame(&mut buf)?;
    let row = scaled.width();
    let width = scaled.width() / 2;
    let height = scaled.height() / 2;
    let get = |x: u32, y: u32| {
        Color::new(
            bytes[((x + y * row) * 4 + 0) as usize],
            bytes[((x + y * row) * 4 + 1) as usize],
            bytes[((x + y * row) * 4 + 2) as usize],
        )
    };

    let mut vec = Vec::<u8>::with_capacity((width * height * 64) as usize);

    println!("Encoding..");

    for y in 0..height {
        for x in 0..width {
            let (char, bg, fg) = quad::binarize(
                get(x * 2 + 0, y * 2 + 0),
                get(x * 2 + 1, y * 2 + 0),
                get(x * 2 + 1, y * 2 + 1),
                get(x * 2 + 0, y * 2 + 1),
            );

            write!(vec, "\x1b[{};{}H", y + 1, x + 1)?;
            write!(vec, "\x1b[48;2;{};{};{}m", bg.r, bg.g, bg.b)?;
            write!(vec, "\x1b[38;2;{};{};{}m", fg.r, fg.g, fg.b)?;
            write!(vec, "{char}")?;
        }
    }

    write!(vec, "\n")?;

    io::stdout().write(&vec)?;
    io::stdout().flush()?;

    Ok(())
}
