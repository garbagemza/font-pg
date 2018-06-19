extern crate freetype as ft;
extern crate unicode_normalization;
extern crate bmp;

use unicode_normalization::UnicodeNormalization;
use bmp::Image;
use bmp::Pixel;

const WIDTH: usize = 32;
const HEIGHT: usize = 48;

fn draw_bitmap(bitmap: ft::Bitmap, x: i32, y: i32) -> [[u8; WIDTH]; HEIGHT] {
    let mut figure = [[0; WIDTH]; HEIGHT];
    let mut p = 0;
    let mut q = 0;

    let x_max = x + bitmap.width();
    let y_max = y + bitmap.rows();

    for i in x .. x_max {
        for j in y .. y_max {
            if i >= 0 && j >= 0 && i < WIDTH as i32 && j < HEIGHT as i32 {
                let index = (q * bitmap.width() + p) as usize;
                figure[j as usize][i as usize] |= bitmap.buffer()[index];
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }
    figure
}

fn main() {
    let ref mut args = std::env::args();
    let mut img = Image::new(WIDTH as u32, HEIGHT as u32);

    if args.len() != 3 {
        let exe = args.next().unwrap();
        println!("Usage: {} font character", exe);
        return
    }

    let ref font = args.nth(1).unwrap();
    let character = args.next().and_then(|s| s.nfc().next()).unwrap() as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();

    face.set_char_size(18 * 64, 0, 100, 100).unwrap();
    face.load_char(character, ft::face::LoadFlag::RENDER).unwrap();

    let glyph = face.glyph();
    let x = glyph.bitmap_left();
    let y = glyph.bitmap_top();
    let figure = draw_bitmap(glyph.bitmap(), x, y);

    for j in 0 .. HEIGHT {
        for i in 0 .. WIDTH {
            let value: u8 = figure[j][i];
            img.set_pixel(i as u32, j as u32, Pixel::new(value, value, value));
        }
    }
    let _ = img.save("./output/img.bmp");

}
