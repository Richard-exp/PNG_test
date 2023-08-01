use std::io::BufReader;
use std::path;

use image::ImageFormat;
use image::GenericImageView;
use std::fs::File;
use std::path::Path;

fn main() {
    // Загрузите изображение из файла
    let img = image::open("example.png").expect("Не удалось загрузить изображение");

    // Получите размер изображения
    let (width, height) = img.dimensions();

    // Создайте вектор для хранения пикселей
    let mut pixels: Vec<[u8; 4]> = Vec::with_capacity((width * height) as usize);

    // Заполните вектор значениями пикселей изображения
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            pixels.push(pixel.0);
        }
    }

    // Теперь у вас есть вектор pixels, содержащий пиксели изображения в формате [R, G, B, A].
    let mut counter = 0;
    pixels.iter().for_each(|pix| if pix[0] == 0 {counter+=1; print!("{} ", counter)});
}