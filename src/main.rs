mod parser;

use parser::Parser;

fn main() {
    // Загрузите изображение из файла, устанавливаем порог черноты 
    // и создаем поле из 0 и 1
    let mut parser = Parser::new("example.png");

    let pixels:&Vec<u8> = 
        parser
        .set_darkness_threshold(42)
        .create_field();

    // Теперь у вас есть вектор pixels, содержащий пиксели изображения в формате [R, G, B, A].
    let mut counter = 0;
    
    pixels.iter().for_each(|&pix| if pix == 1 {counter+=1});

    println!("{} pixels are black", counter);
}