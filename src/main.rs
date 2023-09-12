use crate::finder::Pos;
use std::fs::File;
use std::io::{self, Write};
mod detector;
mod finder;
mod parser;

fn main() {
    // Загрузите изображение из файла, устанавливаем порог черноты
    // и создаем поле из 0 и 1
    let mut parser = parser::Parser::new("test_2.png");

    let pixels: Vec<Vec<u8>> = parser.set_darkness_threshold(42).create_field();
    // let mut matrix: Vec<Vec<u8>> = vec![vec![1; 100]; 100];
    // //matrix[4][4] = 0;

    let pixels_with_offset: Vec<Vec<u8>> = detector::make_offset(pixels, 2);

    let path = finder::find_path(&pixels_with_offset);

    let file_path = "matrix.txt";
    let mut file = File::create(file_path).expect("Cannot create file object");

    for (i, row) in pixels_with_offset.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if path.contains(&Pos {
                row: i as i32,
                column: j as i32,
            }) {
                file.write_all(b"*").expect("cannot write 1");
                file.write_all(b" ").expect("cannot write 2"); // Add space between elements.
            } else {
                file.write_all(value.to_string().as_bytes())
                    .expect("cannot write 3");
                file.write_all(b" ").expect("cannot write 4");
            }
        }
        file.write_all(b"\n").expect("cannot write 3"); // Add a newline character at the end of each row.
    }
}
