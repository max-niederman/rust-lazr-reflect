#![feature(test)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(array_zip)]

mod direction;
mod laser;
mod tilemap;
mod vector;

extern crate test;

use direction::{Angle, Direction};
use minifb::{Key, Window, WindowOptions};
use tilemap::{Tile, Tilemap};
use tiny_skia::{Color, Pixmap};

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

const TILE_SIZE: u32 = 100;

fn main() {
    pretty_env_logger::init();

    let mut window = Window::new(
        "Rust Lazr Reflection Experiment",
        WIDTH as usize * TILE_SIZE as usize,
        HEIGHT as usize * TILE_SIZE as usize,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let window_size = window.get_size();
        let mut pixmap = Pixmap::new(window_size.0 as u32, window_size.1 as u32).unwrap();
        pixmap.fill(Color::from_rgba8(0xf7, 0x25, 0x85, 0xff));

        let mut tilemap = Tilemap::new(WIDTH, HEIGHT);
        for i in 0..tilemap.len() {
            tilemap[i as u32] = Tile::Mirror {
                normal: Direction::rotated_from_east(Angle::from_octants((i % 0xFF) as u8)),
            };
        }
        tilemap.render(&mut pixmap);

        draw_pixmap(&pixmap, &mut window);
    }
}

fn draw_pixmap(pixmap: &Pixmap, window: &mut Window) {
    // EXPL: pixmap stores pixels in RGBA order.
    //       minifb expects ARGB, but ignores the alpha channel.

    let (width, height) = (pixmap.width(), pixmap.height());

    let mut buffer = vec![0; pixmap.data().len() / 4];

    // SAFETY: reinterpreting a slice of u8 as u32 is safe as long as the slice's length is a multiple of 4.
    debug_assert!(pixmap.data().len() % 4 == 0);
    unsafe {
        buffer[1..].copy_from_slice(
            &std::mem::transmute::<&[u8], &[u32]>(pixmap.data())[..pixmap.data().len() / 4 - 1],
        );
    }

    window
        .update_with_buffer(&buffer, width as usize, height as usize)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn draw_pixmap(b: &mut Bencher) {
        let mut window = Window::new(
            "Rust Lazr Reflection Benchmark",
            1000,
            1000,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();

        b.iter(|| {
            let mut pixmap = Pixmap::new(1000, 1000).unwrap();
            pixmap.fill(Color::from_rgba8(0xf7, 0x25, 0x85, 0xff));
            super::draw_pixmap(&pixmap, &mut window);
        })
    }
    #[bench]
    fn draw_tilemap(b: &mut Bencher) {
        let mut window = Window::new(
            "Rust Lazr Reflection Experiment",
            1000,
            1000,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap();

        b.iter(|| {
            let mut pixmap = Pixmap::new(1000, 1000).unwrap();
            pixmap.fill(Color::from_rgba8(0xf7, 0x25, 0x85, 0xff));
            let mut tilemap = Tilemap::new(WIDTH, HEIGHT);
            for i in 0..tilemap.len() {
                tilemap[i as u32] = Tile::Mirror {
                    normal: Direction::rotated_from_east(Angle::from_octants(i as u8)),
                };
            }
            tilemap.render(&mut pixmap);
            super::draw_pixmap(&pixmap, &mut window);
        })
    }
}
