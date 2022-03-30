use super::Data;
use super::Color;

/// 
/// 
/// [reference](http://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm)
/// 
pub fn line(data: &mut Data, c0:(u8,u8), c1:(u8,u8), color: Color) {
    let (x0,y0) = c0;
    let (x1,y1) = c1;

    let x0 = x0 as i16;
    let y0 = y0 as i16;
    let x1 = x1 as i16;
    let y1 = y1 as i16;

    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else {-dy} / 2;
    let mut err2;

    loop {
        data[x0 as usize][y0 as usize] = color;

        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }
}

pub fn line_with_width(data: &mut Data, c0:(u8,u8), c1:(u8,u8), w:u8, color: Color) {
    let (x0,y0) = c0;
    let (x1,y1) = c1;

    fill_square(data, (x0,y0) , w, color);
    fill_square(data, (x1,y1) , w, color);
    let x0 = x0 as i16;
    let y0 = y0 as i16;
    let x1 = x1 as i16;
    let y1 = y1 as i16;
    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset 
    // Get slopes
    let (dx, sx) = if x0>x1 { (x0-x1, -1) } else { (x1-x0, 1) };
    let (dy, sy) = if y0>y1 { (y0-y1, -1) } else { (y1-y0, 1) };

    // Initialize error
    let mut err = dx-dy;
    let w = w as f32;
    let mut err2;
    let errd = ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt();
    let w = (w+1.0)/2.0;
    let mut x2;
    let mut y2;
    loop {
        data[x0 as usize][y0 as usize] = color;

        err2 = err;
        x2 = x0;
        if 2*err2 >= -dx { 
            err2 += dy;
            y2 = y0;
            while (err2 as f32) < errd*w && (y1 != y2 || dx > dy) {
                y2 += sy;
                set_color_safe(data, x0, y2, color);
                err2 += dx;
            }
            if x0==x1 {break;}
            err2 = err; 
            err -= dy; 
            x0 += sx; 
        }
        if 2*err2 <= dy {
            err2 += dx-err2;
            while (err2 as f32) < errd*w && (x1 != x2 || dx < dy) {
                x2 += sx;
                set_color_safe(data, x2, y0, color);
                err2 += dy;
            }
            if y0==y1 {break;}
            err += dx; 
            y0 += sy; 
        }
    }
}

#[inline]
pub fn point(data: &mut Data, c:(u8,u8), color: Color) {
    let (x,y) = c;
    data[x as usize][y as usize] = color;
}

#[inline]
pub fn fill_square(data: &mut Data, c:(u8,u8), w:u8, color: Color) {
    let left = (c.0 as i16) - ((w/2) as i16);
    let right = (c.0 as i16) + ((w/2) as i16);
    let top = (c.1 as i16) - ((w/2) as i16);
    let bottom = (c.1 as i16) + ((w/2) as i16);
    for x in left..right{
        for y in top..bottom {
            set_color_safe(data, x, y, color);
        }
    }
}


/// this function will clamp x and y
/// 
#[inline]
pub fn set_color_safe(data: &mut Data, x: i16, y:i16, color: Color) {
    let y = y.clamp(0, 127);
    let x = x.clamp(0, 127);
    data[x as usize][y as usize] = color;
}