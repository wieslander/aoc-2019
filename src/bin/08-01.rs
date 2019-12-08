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

    pub fn set(&mut self, x: usize, y: usize, pixel: u8) {
        let offset = self.width * y + x;
        self.pixels[offset] = pixel;
    }
}

fn main() {
    let input = get_input();
    let pixels = input.chars().map(|c| c as u8 - '0' as u8).collect();
    let image = Image::from_pixels(&pixels, 25, 6);

    let mut smallest_layer_zeroes = image.width * image.height;
    let mut ones = 0;
    let mut twos = 0;

    for layer in &image.layers {
        let zeroes = layer.pixels.iter().filter(|&p| *p == 0).count();
        if zeroes < smallest_layer_zeroes {
            smallest_layer_zeroes = zeroes;
            ones = layer.pixels.iter().filter(|&p| *p == 1).count();
            twos = layer.pixels.iter().filter(|&p| *p == 2).count();
        }

    }

    println!("{}", ones * twos);
}
