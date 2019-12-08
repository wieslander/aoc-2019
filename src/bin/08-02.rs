use std::fmt;
use aoc::get_input;

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn from_pixels(pixels: &Vec<u8>, width: usize, height: usize) -> Image {
        let layer_size = width * height;
        let layer_count = pixels.len() / layer_size;
        let mut img = Image { width, height, layers: vec![] };

        for layer_index in 0..layer_count {
            let mut layer = Layer::new(width, height);
            for pixel_offset in 0..layer_size {
                let x = pixel_offset % width;
                let y = pixel_offset / width;
                let offset = layer_index * layer_size + pixel_offset;
                let pixel = pixels[offset];
                layer.set(x, y, pixel);
            }
            img.layers.push(layer);
        }

        img
    }

    fn render(&self) -> Layer {
        let mut rendered_layer = Layer::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.visible_pixel(x, y);
                rendered_layer.set(x, y, pixel);
            }
        }

        rendered_layer
    }

    fn visible_pixel(&self, x: usize, y: usize) -> u8 {
        let mut rendered_color = 2;
        for layer in &self.layers {
            let pixel = layer.get(x, y);
            if pixel != 2 {
                rendered_color = pixel;
                break;
            }
        }
        rendered_color
    }
}

struct Layer {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Layer {
    pub fn new(width: usize, height: usize) -> Layer {
        Layer {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }

    fn render(&self) -> String {
        let mut lines = vec![];

        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let pixel = self.get(x, y);
                let c = match pixel {
                    0 => ' ',
                    1 => 'x',
                    2 => '_',
                    _ => '?',
                };
                line.push(c);
            }
            lines.push(line);
        }

        lines.join("\n")
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: u8) {
        let offset = self.pixel_offset(x, y);
        self.pixels[offset] = pixel;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        let offset = self.pixel_offset(x, y);
        self.pixels[offset]
    }

    fn pixel_offset(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

fn main() {
    let input = get_input();
    let pixels = input.chars().map(|c| c as u8 - '0' as u8).collect();
    let image = Image::from_pixels(&pixels, 25, 6);
    let layer = image.render();

    println!("{}", layer);
}
