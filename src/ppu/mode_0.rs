use crate::memory::Memory;
use crate::ppu::Ppu;

impl Ppu{
    pub fn render_mode_0_scanline(&mut self, mem: &mut Memory){
        let base_idx = (mem.vertical_count as usize)*240;

        let mut bg_order = [(0usize, mem.bg_control[0].priority),
                                         (1, mem.bg_control[1].priority),
                                         (2, mem.bg_control[2].priority),
                                         (3, mem.bg_control[3].priority)];
        bg_order.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let order = bg_order.map(|x| x.0);

        let default_p =  (mem.palette_ram[0] as u16) | ((mem.palette_ram[1] as u16) << 8);
        let default_r = ((default_p & 0b011111) as u8) << 3;
        let default_g = (((default_p>>5) & 0b011111) as u8) << 3;
        let default_b = (((default_p>>10) & 0b011111) as u8) << 3;

        for x in 0usize..240 {
            self.r[x + base_idx] = default_r;
            self.g[x + base_idx] = default_g;
            self.b[x + base_idx] = default_b;
            for bg_idx in order{
                if !mem.bg_enabled[bg_idx]{
                    continue;
                }
                let map_block_width;
                let map_block_height;
                // look up map block size in tiles, based on bg control register
                match mem.bg_control[bg_idx].map_size{
                    0 => {
                        map_block_width = 32;
                        map_block_height = 32;
                    },
                    1 => {
                        map_block_width = 64;
                        map_block_height = 32;
                    },
                    2 => {
                        map_block_width = 32;
                        map_block_height = 64;
                    },
                    3 => {
                        map_block_width = 64;
                        map_block_height = 64;
                    },
                    _ => {
                        map_block_width = 32;
                        map_block_height = 32;
                    }
                }
                let x_scroll =  mem.bg_scroll_regs[bg_idx*2] as usize;
                let y_scroll = mem.bg_scroll_regs[bg_idx*2 + 1] as usize;
                let map_x = (x + x_scroll) % (map_block_width*8);
                let map_y = (mem.vertical_count as usize + y_scroll) % (map_block_height*8);

                let tile_x = map_x / 8;
                let tile_y = map_y / 8;

                let block = mem.bg_control[bg_idx].map_block + (tile_y/32)*(map_block_width/32) + (tile_x/32);
                let offset = 2 * ((32 * (tile_y % 32)) + (tile_x %32));

                let map_entry_addr = 0x800*block + offset;

                let map_entry = (mem.vram[map_entry_addr] as u16) | ((mem.vram[map_entry_addr+1] as u16) << 8);
                let tile_number =  (map_entry & 0b11_1111_1111) as usize;
                let flip_x = (tile_number & (1 << 10)) != 0;
                let flip_y = (tile_number & (1 << 11)) != 0;
                let palette_bank = ((map_entry >> 12) & 0xF) as usize;

                let tile_addr = 0x4000*mem.bg_control[bg_idx].tile_block + 32*tile_number;

                let px;
                if flip_x{
                    px = 7 - ((x + x_scroll) % 8);
                }else{
                    px = (x + x_scroll) % 8;
                }

                let py;
                if flip_y {
                    py = 7 - ((mem.vertical_count as usize + y_scroll) % 8);
                } else{
                    py = (mem.vertical_count as usize + y_scroll) % 8;
                }

                let byte_addr = tile_addr + 4*py + px/2;

                let mut data = mem.vram[byte_addr];

                if px % 2 == 1{
                    data >>= 4;
                }

                data = data & 0xF;

                if data == 0{
                    continue;
                }

                let p_low = mem.palette_ram[palette_bank*32 + data as usize*2];
                let p_high = mem.palette_ram[palette_bank*32 + data as usize*2 +1];

                let p = ((p_high as u16)<<8) | (p_low as u16);
                self.r[x + base_idx] = ((p & 0b011111) as u8) << 3;
                self.g[x + base_idx] = (((p>>5) & 0b011111) as u8) << 3;
                self.b[x + base_idx] = (((p>>10) & 0b011111) as u8) << 3;

                break;
            }

            /*let p_low = mem.vram[i*2];
            let p_high = mem.vram[i*2 +1];

            let p = ((p_high as u16)<<8) | (p_low as u16);
            self.r[i] = ((p & 0b011111) as u8) << 3;
            self.g[i] = (((p>>5) & 0b011111) as u8) << 3;
            self.b[i] = (((p>>10) & 0b011111) as u8) << 3;*/
        }
    }
}