use crate::constant::{WS2812_HIGH_DUTY, WS2812_LOW_DUTY};
pub fn write_8x8_bitmap<'a>(
    data: u64,
    buf: &'a mut [u16; 1537],
    color: &[u8; 3],
) -> &'a [u16; 1537] {
    let mut offset = 0;
    let mut mask = 1 << 63;
    for _ in 0..8 {
        for _ in 0..8 {
            for color_ch in (0..3).rev() {
                let color = color[color_ch];
                for i in 0..8 {
                    let colorbit = color & (0b10000000 >> i);
                    if data & mask != 0 {
                        buf[offset] = if colorbit != 0 {
                            WS2812_HIGH_DUTY
                        } else {
                            WS2812_LOW_DUTY
                        };
                    } else {
                        buf[offset] = WS2812_LOW_DUTY;
                    }
                    offset += 1;
                }
            }
            mask >>= 1;
        }
    }
    buf[1536] = 0;
    buf
}
