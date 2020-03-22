use image::{ImageBuffer, Rgb};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
pub enum PixelType {
    Background,
    Color,
    Outline,
}

pub struct PixelMap {
    pub width: usize,
    pub height: usize,
    data: Vec<PixelType>,
}

impl PixelMap {
    pub fn generate(width: usize, height: usize) -> PixelMap {
        let mut rng = rand::thread_rng();
        let mut data = vec![PixelType::Background; width * height];
        for elem in &mut data {
            *elem = match vec![PixelType::Background, PixelType::Color].choose(&mut rng) {
                Some(p) => *p,
                None => PixelType::Background,
            };
        }
        PixelMap {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &PixelType {
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut PixelType {
        &mut self.data[y * self.width + x]
    }

    pub fn image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width as u32, self.height as u32);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                match self.get(x, y) {
                    PixelType::Background => {
                        *pixel = image::Rgb([255, 255, 255]);
                    }
                    PixelType::Color => {
                        *pixel = image::Rgb([0, 255, 0]);
                    }
                    PixelType::Outline => {
                        *pixel = image::Rgb([0, 180, 0]);
                    }
                }
            }
        }
        img
    }
}

fn count_neighbors(state: &PixelMap, x: usize, y: usize) -> u16 {
    let mut count = 0;
    if x > 0 && state.get(x - 1, y) == &PixelType::Color {
        count += 1;
    }
    if y > 0 && state.get(x, y - 1) == &PixelType::Color {
        count += 1;
    }
    if x < state.width - 1 && state.get(x + 1, y) == &PixelType::Color {
        count += 1;
    }
    if y < state.height - 1 && state.get(x, y + 1) == &PixelType::Color {
        count += 1;
    }
    count
}

fn evolve(state: PixelMap) -> PixelMap {
    let mut evolved_state = PixelMap {
        data: state.data.clone(),
        ..state
    };
    for y in 0..state.height {
        for x in 0..state.width {
            let cell = state.get(x, y);
            let neighbors = count_neighbors(&state, x, y);
            let evolved_cell = evolved_state.get_mut(x, y);
            *evolved_cell = if (cell == &PixelType::Background && neighbors <= 1)
                || (cell == &PixelType::Color && (neighbors == 2 || neighbors == 3))
            {
                PixelType::Color
            } else {
                PixelType::Background
            }
        }
    }
    evolved_state
}

pub fn generate_sprite_map() -> PixelMap {
    // TODO: Remove magic numbers throughout the function
    let mut pixel_map = PixelMap::generate(4, 8);
    pixel_map = evolve(evolve(pixel_map));
    // Grow 4x8 pixel map into 5x10 by adding borders on all directions except the mirror direction (left)
    // The new data vector will be constructed row by row
    let mut new_data = vec![PixelType::Background; 5]; // First row
    for chunk in pixel_map.data.chunks(4) {
        let mut row = Vec::from(chunk);
        row.push(PixelType::Background); // Add new column to each row
        new_data.extend(row);
    }
    new_data.extend_from_slice(&[PixelType::Background; 5]);
    pixel_map = PixelMap {
        width: 5,
        height: 10,
        data: new_data,
    };

    // Add outline
    for y in 0..pixel_map.height {
        for x in 0..pixel_map.width {
            let cell = pixel_map.get(x, y);
            if cell == &PixelType::Background && count_neighbors(&pixel_map, x, y) > 0 {
                *pixel_map.get_mut(x, y) = PixelType::Outline;
            }
        }
    }

    // Mirror the data on the OY axis
    let mut mirrored = Vec::with_capacity(10 * 10);
    for chunk in pixel_map.data.chunks_mut(5) {
        let mut row: Vec<PixelType> = chunk.iter().rev().cloned().collect();
        row.append(&mut Vec::from(chunk));
        mirrored.extend(row);
    }
    PixelMap {
        width: 10,
        height: 10,
        data: mirrored,
    }
}
