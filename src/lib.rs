#![no_std]

pub mod body;
pub mod color;
pub mod draw;
pub mod math;
pub mod physics;

use crate::color::Rgba;
use crate::draw::draw_rect;

pub const PIXELS_PER_METER: usize = 50;

#[repr(C)]
#[derive(Default)]
pub struct State {
    initialized: bool,
}

pub struct Framebuffer<'a> {
    width: usize,
    height: usize,
    bpp: usize,
    size: usize,
    byte_len: usize,
    data: &'a mut [u8],
}

impl<'a> Framebuffer<'a> {
    pub fn new(width: usize, height: usize, bpp: usize, data: &'a mut [u8]) -> Self {
        Self {
            width,
            height,
            bpp,
            size: width * height,
            byte_len: width * height * bpp,
            data,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn bpp(&self) -> usize {
        self.bpp
    }

    fn size(&self) -> usize {
        self.size
    }

    fn byte_len(&self) -> usize {
        self.byte_len
    }
}

pub fn update_and_render(state: &mut State, mut buffer: Framebuffer, dt: f32) {
    if !state.initialized {
        // ...

        state.initialized = true;
    }

    let bw = buffer.width();
    let bh = buffer.height();

    draw_rect(&mut buffer, 0, 0, bw, bh, Rgba::BLACK);
}
