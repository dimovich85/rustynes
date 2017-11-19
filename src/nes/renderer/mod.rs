use super::BackgroundField;
use super::Tile;

extern "C" {
    fn canvas_render(ptr: *const u8, len: usize);
}

pub fn render(background: &BackgroundField) {
    render_background(background);
}

fn render_background(background: &BackgroundField) {
    let mut data: Vec<u8> = vec![0; 256 * 224 * 4];
    for (i, bg) in background.into_iter().enumerate() {
        let x = (i % 33) * 8;
        let y = (i / 33) * 8;
        render_tile(&mut data, bg, x, y /* , palette */);
    }
    unsafe {
        canvas_render(data.as_ptr(), data.len());
    }    
}

fn render_tile(data: &mut Vec<u8>, bg: &Tile, x: usize, y: usize) {
    let offset_x = 0; // scroll_x % 8;
    let offset_y = 0; // scroll_y % 8;
    for i in 0..8 {
        for j in 0..8 {
            let palette_index = bg.palette_id * 4 + bg.sprite[i][j];
            // let color_id = palette[palette_index];
            // let color = colors[color_id];
            let x = x + j - offset_x;
            let y = y + i - offset_y;
            if x >= 0 as usize && 0xFF >= x && y >= 0 as usize && y < 224 {
                let index = (x + (y * 0x100)) * 4;
                data[index] = if bg.sprite[i][j] == 0 {0xff} else {0x00}; // color[0];
                data[index + 1] = if bg.sprite[i][j] == 0 {0xff} else {0x00}; // color[0]; // color[1];
                data[index + 2] = if bg.sprite[i][j] == 0 {0xff} else {0x00}; // color[0]; // color[2];
                data[index + 3] = 0xFF;
            }
        }
    }
}