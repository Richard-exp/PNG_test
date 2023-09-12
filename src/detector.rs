pub fn make_offset(pixels: Vec<Vec<u8>>, offset: u8) -> Vec<Vec<u8>> {
    let size_x = pixels[0].len();
    let size_y = pixels.len();

    let mut new_pixels = vec![vec![1; size_x]; size_y];

    for y in 0..size_y {
        for x in 0..size_x {
            let current_pixel = pixels[y][x];

            if current_pixel == 0 {
                // Ищем верхний левый угол
                let offset_upper_x = if x >= offset as usize { x - offset as usize } else { 0 };
                let offset_upper_y = if y >= offset as usize { y - offset as usize } else { 0 };

                // Ищем нижний правый угол
                let offset_lower_x = if (x + offset as usize) < size_x { x + offset as usize } else { size_x - 1 };
                let offset_lower_y = if (y + offset as usize) < size_y { y + offset as usize } else { size_y - 1 };

                // Заполняем квадрат вокруг пикселя
                draw_square(&mut new_pixels, offset_upper_x, offset_lower_x, offset_upper_y, offset_lower_y);
            }
        }
    }

    new_pixels
}

fn draw_square(pixels: &mut Vec<Vec<u8>>, from_x: usize, to_x: usize, from_y: usize, to_y: usize) {
    for y in from_y..=to_y {
        for x in from_x..=to_x {
            pixels[y][x] = 0;
        }
    }
}