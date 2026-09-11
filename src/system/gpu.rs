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

/// The size of the Video RAM in bytes.
pub const VRAM_SIZE: usize =
    PALETTE_ENTRY_COUNT * 2
    + TILESET_ENTRY_COUNT * 2
    + (SCREEN_COLUMNS / TILE_COLUMNS) * (SCREEN_ROWS / TILE_ROWS);

impl System {
    /// Render the current state of VRAM to an SDL canvas.
    pub fn render_sdl(&self, canvas: &mut sdl3::render::Canvas<sdl3::video::Window>) -> Result<(), Box<dyn std::error::Error>> {
        let palette = self.vram_palette();
        let tileset_pointers = self.vram_tileset();
        let tiles = self.vram_tiles();

        for row in 0..SCREEN_ROWS {
            for col in 0..SCREEN_COLUMNS {
                let tile_index = tiles[row * SCREEN_COLUMNS + col] as usize;
                let tile_pointer = tileset_pointers[tile_index];

                let tile_colors = (0..TILESET_ENTRY_COLORS as u16)
                    .map(|i| {
                        let color_index = self.get_memb(tile_pointer.wrapping_add(i as u16));
                        color_index.map(|i| palette[i as usize])
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let tile_data = (0..TILESET_ENTRY_DATA_SIZE as u16)
                    .map(|i| {
                        self.get_memb(tile_pointer.wrapping_add(TILESET_ENTRY_COLORS as u16).wrapping_add(i)).map(|x| [((x >> 4) & 15), (x & 15)])
                    })
                    .collect::<Result<Vec<[u8; 2]>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>()
                    .as_chunks::<TILE_COLUMNS>().0
                    .to_vec();
                for tile_row in 0..TILE_ROWS {
                    for tile_col in 0..TILE_COLUMNS {
                        let color_index = tile_data[tile_row][tile_col];
                        let color: u16 = tile_colors[color_index as usize];
                        // RGB565 to 24-bit conversion
                        let color: (u8, u8, u8) = (
                            ((color >> 11) & 0x1F) as u8 * (255 / 31),
                            ((color >> 5) & 0x3F) as u8 * (255 / 63),
                            (color & 0x1F) as u8 * (255 / 31),
                        );
                        canvas.set_draw_color(sdl3::pixels::Color::RGB(color.0, color.1, color.2));
                        canvas.draw_point(sdl3::rect::Point::new(
                            (col * TILE_COLUMNS + tile_col) as i32,
                            (row * TILE_ROWS + tile_row) as i32,
                        ))?;
                    }
                }
            }
        }
        self.frames.update(|f| f.wrapping_add(1));
        Ok(())
    }
}