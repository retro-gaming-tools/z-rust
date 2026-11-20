//! The text of the runtime's menus and dialogs, in DejaVu Sans.
//!
//! SDL's built-in debug font is 8 by 8 pixels, fixed width and ASCII only,
//! and stretched over a large window it was hard to read. This draws a
//! proportional font from a glyph atlas made by `tools/make_font.py` at 36
//! pixels, three times the 12 pixels it has in the 640x480 layout, which is
//! the scale the menus' surface is drawn at (`OVERLAY_SCALE`). DejaVu Sans is
//! under the Bitstream Vera licence, which allows embedding and passing it on.
//!
//! Positions are in the 640x480 layout. A call's `y` is the top of the
//! 8-pixel box the debug font filled, so callers lay out as before: the text
//! is centred on that box's middle.

use super::*;

mod glyphs {
    include!("font_glyphs.rs");
}
use glyphs::{ASCENT, ATLAS_WIDTH, GLYPHS, LINE};

static ATLAS: &[u8] = include_bytes!("font.alpha");

/// Atlas pixels to one layout pixel.
const SCALE: f32 = 3.0;
const PIXELFORMAT_ARGB8888: u32 = 0x1636_2004;
const TEXTUREACCESS_STATIC: c_int = 0;
const SCALEMODE_LINEAR: c_int = 1;

fn glyph(c: char) -> Option<&'static (char, u32, u32, i32, u32)> {
    GLYPHS.iter().find(|glyph| glyph.0 == c).or_else(|| GLYPHS.iter().find(|glyph| glyph.0 == '?'))
}

/// How wide a text is in the layout.
pub fn width(text: &str) -> f32 {
    text.chars().filter_map(glyph).map(|glyph| glyph.4 as f32).sum::<f32>() / SCALE
}

impl Video {
    /// The atlas as a texture of the menus' renderer: white, the glyphs'
    /// coverage as alpha, to be tinted when drawn.
    fn font_texture(&self) -> Pointer {
        let made = self.font_texture.get();
        if !made.is_null() {
            return made;
        }
        let pixels: Vec<u32> = ATLAS.iter().map(|&alpha| (alpha as u32) << 24 | 0x00ff_ffff).collect();
        unsafe {
            let texture = (self.create_texture)(self.renderer, PIXELFORMAT_ARGB8888, TEXTUREACCESS_STATIC, ATLAS_WIDTH as c_int, LINE as c_int);
            if texture.is_null() {
                return texture;
            }
            (self.update_texture)(texture, std::ptr::null(), pixels.as_ptr() as *const c_void, (ATLAS_WIDTH * 4) as c_int);
            (self.texture_blend)(texture, BLEND_ALPHA);
            (self.texture_scale_mode)(texture, SCALEMODE_LINEAR);
            self.font_texture.set(texture);
            texture
        }
    }

    /// Draws a text in the renderer's current draw colour, its left edge at
    /// `x`, centred on the middle of the 8 pixels below `y`.
    pub(super) fn text(&self, x: f32, y: f32, text: &str) {
        let texture = self.font_texture();
        if texture.is_null() {
            return;
        }
        let (mut red, mut green, mut blue, mut alpha) = (0u8, 0u8, 0u8, 0u8);
        unsafe { (self.get_color)(self.renderer, &mut red, &mut green, &mut blue, &mut alpha) };
        unsafe { (self.texture_color)(texture, red, green, blue) };
        // The middle of the capitals (half of DejaVu's cap height, 0.73 of
        // the 36-pixel em, above the baseline) goes where the debug font's
        // middle was.
        let middle_of_capitals = ASCENT as f32 - 0.365 * 36.0;
        let top = y + 4.0 - middle_of_capitals / SCALE;
        let mut pen = x;
        for c in text.chars() {
            let Some(&(_, at, cell, bearing, advance)) = glyph(c) else { continue };
            let source = [at as f32, 0.0, cell as f32, LINE as f32];
            let target = [pen + bearing as f32 / SCALE, top, cell as f32 / SCALE, LINE as f32 / SCALE];
            unsafe { (self.render_texture)(self.renderer, texture, source.as_ptr() as *const c_void, target.as_ptr() as *const c_void) };
            pen += advance as f32 / SCALE;
        }
    }
}
