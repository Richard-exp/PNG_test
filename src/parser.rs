// parser.rs
use image::GenericImageView;

pub struct Parser{
    img: image::DynamicImage,
    width: u32,
    height: u32,
    darkness_threshold: u8,
    pixels_u1: Vec<u8>,
}

impl Parser{
  
    pub fn new(path: &str) -> Self {
        // Загрузите изображение из файла
        let img = image::open(path)
            .expect("Не удалось загрузить изображение");

        // Получите размер изображения
        let (width, height) = img.dimensions();

        print!("Width is: {width},{height}");
        
        // Создайте вектор для хранения 0 и 1
        let mut pixels_u1: Vec<u8> = Vec::with_capacity((width * height) as usize);

        // darkness_threshold
        let mut darkness_threshold: u8 = 0;

        Self {
            img,
            width, 
            height,
            darkness_threshold,
            pixels_u1,
        }
    }

    pub fn set_darkness_threshold(&mut self, threshold: u8) -> &mut Self {
        self.darkness_threshold = threshold;
        self
    }

    pub fn create_field(&mut self) -> Vec<Vec<u8>> {
        // Заполните вектор значениями пикселей изображения 0 или 1 в зависимости от того, является ли пиксель черным
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels_u1.push(self.is_pixel_black(self.img.get_pixel(x, y)) as u8);
            }
        }
        self.transform_to_2d()
    }

    fn is_pixel_black(&self, pixel: image::Rgba<u8>) -> bool {
        // Вычисляем среднюю интенсивность цветовых компонент (R, G, B)
        let intensity = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;

        // Проверяем, является ли интенсивность меньше заданного порога черноты
        return !(intensity <= (self.darkness_threshold as u16));
    }

    fn transform_to_2d(&mut self) -> Vec<Vec<u8>> {
        // Create a 2D vector by iterating and splitting the one-dimensional vector
        let mut two_d_vector: Vec<Vec<u8>> = Vec::with_capacity(self.height as usize);
        
        for chunk in self.pixels_u1.chunks(self.width as usize) {
            let row: Vec<u8> = chunk.to_vec();
            two_d_vector.push(row);
        }

        two_d_vector
    }
}


    // Path: src\main.rs
