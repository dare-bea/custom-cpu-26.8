use std::collections::HashMap;

use crate::system::System;

/// The number of tiles on the screen vertically.
pub const SCREEN_ROWS: usize = 32;
/// The number of tiles on the screen horizontally.
pub const SCREEN_COLUMNS: usize = 32;

/// The number of rows in a tile.
pub const TILE_ROWS: usize = 8;
/// The number of columns in a tile.
pub const TILE_COLUMNS: usize = 8;

/// The number of entries in the palette, 16 bits per entry.
pub const PALETTE_ENTRY_COUNT: usize = 256;
/// The number of tiles in the tileset.
pub const TILESET_ENTRY_COUNT: usize = 256;
/// The number of colors in each tileset entry.
pub const TILESET_ENTRY_COLORS: usize = 16;
/// The size of each tileset entry in bytes.
pub const TILESET_ENTRY_DATA_SIZE: usize = TILE_ROWS * TILE_COLUMNS / 2;
const RGB_PIXEL_SIZE: usize = 3;
const RGB_TILE_SIZE: usize = TILE_ROWS * TILE_COLUMNS * RGB_PIXEL_SIZE;

/// The size of the Video RAM in bytes.
pub const VRAM_SIZE: usize = 0x10000;

/// The interval at which V-Blanks occur, in cycles.
pub const VBLANK_INTERVAL: u64 = 2_000_000 / 60;

impl System {
    /// Render the current state of VRAM to an SDL canvas.
    /// 
    /// # Errors
    /// 
    /// An error may occur if an SDL error occurs, or memory access to the system fails
    pub fn render_sdl(
        &self,
        canvas: &mut sdl3::render::Canvas<sdl3::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let palette: Box<[u16]> = self.vram()[0x0000..0x0200]
            .as_chunks::<2>().0.iter()
            .map(|&x| u16::from_be_bytes(x)).take(256).collect();
        let tiles: Box<[u16]> = self.vram()[0x0200..0x0200+SCREEN_COLUMNS*SCREEN_ROWS*2]
            .as_chunks::<2>().0.iter()
            .map(|&x| u16::from_be_bytes(x)).take(SCREEN_COLUMNS*SCREEN_ROWS).collect();
        let mut pixels =
            vec![0; SCREEN_COLUMNS * TILE_COLUMNS * SCREEN_ROWS * TILE_ROWS * RGB_PIXEL_SIZE];
        let mut tile_cache = HashMap::new();

        for row in 0..SCREEN_ROWS {
            for col in 0..SCREEN_COLUMNS {
                let tile_pointer = tiles[row * SCREEN_COLUMNS + col];
                let decoded_tile = tile_cache.entry(tile_pointer).or_insert_with(|| {
                    let mut tile_colors = [0; TILESET_ENTRY_COLORS];
                    for (index, color) in tile_colors.iter_mut().enumerate() {
                        let color_index: u8 = self.vram()[tile_pointer.wrapping_add(index as u16) as usize];
                        *color = palette[color_index as usize];
                    }
                    let mut decoded_tile = [0; RGB_TILE_SIZE];
                    for tile_row in 0..TILE_ROWS {
                        for tile_col in 0..TILE_COLUMNS {
                            let data_offset = TILESET_ENTRY_COLORS as u16
                                + (tile_row * TILE_COLUMNS + tile_col) as u16 / 2;
                            let data = self.vram()[tile_pointer.wrapping_add(data_offset) as usize];
                            let color_index = if tile_col % 2 == 0 {
                                data >> 4
                            } else {
                                data & 15
                            };
                            let color = tile_colors[color_index as usize];
                            let pixel = (tile_row * TILE_COLUMNS + tile_col) * RGB_PIXEL_SIZE;
                            decoded_tile[pixel..pixel + RGB_PIXEL_SIZE].copy_from_slice(&[
                                ((color >> 11) & 0x1F) as u8 * (255 / 31),
                                ((color >> 5) & 0x3F) as u8 * (255 / 63),
                                (color & 0x1F) as u8 * (255 / 31),
                            ]);
                        }
                    }
                decoded_tile
                });
                for tile_row in 0..TILE_ROWS {
                    let source_start = tile_row * TILE_COLUMNS * RGB_PIXEL_SIZE;
                    let destination_start =
                        ((row * TILE_ROWS + tile_row) * SCREEN_COLUMNS * TILE_COLUMNS
                            + col * TILE_COLUMNS)
                            * RGB_PIXEL_SIZE;
                    pixels[destination_start..destination_start + TILE_COLUMNS * RGB_PIXEL_SIZE]
                        .copy_from_slice(
                            &decoded_tile[source_start..source_start + TILE_COLUMNS * RGB_PIXEL_SIZE],
                        );
                }
            }
        }
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture_streaming(
            sdl3::pixels::PixelFormat::RGB24,
            (SCREEN_COLUMNS * TILE_COLUMNS) as u32,
            (SCREEN_ROWS * TILE_ROWS) as u32,
        )?;
        texture.update(None, &pixels, SCREEN_COLUMNS * TILE_COLUMNS * 3)?;
        canvas.copy(&texture, None, None)?;
        self.frames.update(|f| f.wrapping_add(1));
        Ok(())
    }
}
