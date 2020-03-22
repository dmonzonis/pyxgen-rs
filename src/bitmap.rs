use rand::Rng;

pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl Bitmap {
    pub fn generate(width: usize, height: usize) -> Bitmap {
        let mut rng = rand::thread_rng();
        let mut data = vec![0; width * height];
        for elem in &mut data {
            *elem = rng.gen_range(0, 2);
        }
        Bitmap {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.data[y * self.width + x]
    }
}

fn count_neighbors(state: &Bitmap, x: usize, y: usize) -> u16 {
    let mut count = 0;
    if x > 0 && state.get(x, y) == 1 {
        count += 1;
    }
    if y > 0 && state.get(x, y - 1) == 1 {
        count += 1;
    }
    if x < state.width - 1 && state.get(x + 1, y) == 1 {
        count += 1;
    }
    if y < state.height - 1 && state.get(x, y + 1) == 1 {
        count += 1;
    }
    count
}

fn evolve(state: Bitmap) -> Bitmap {
    let mut evolved_state = Bitmap {
        data: state.data.clone(),
        ..state
    };
    for y in 0..state.height {
        for x in 0..state.width {
            let cell = state.get(x, y);
            let neighbors = count_neighbors(&state, x, y);
            let evolved_cell = evolved_state.get_mut(x, y);
            *evolved_cell = if (cell == 0 && neighbors <= 1)
                || (cell == 1 && (neighbors == 2 || neighbors == 3))
            {
                1
            } else {
                0
            }
        }
    }
    evolved_state
}

pub fn generate_sprite_bitmap() -> Bitmap {
    // TODO: Remove magic numbers throughout the function
    // Create a 10x10 matrix of numbers representing the sprite, where a 0 means background,
    // 1 means fill with color and 2 means outline
    let mut bitmap = Bitmap::generate(4, 8);
    bitmap = evolve(evolve(bitmap));
    // Grow 4x8 bitmap into 5x10 by adding borders on all directions except the mirror direction (left)
    // The new data vector will be constructed row by row
    let mut new_data = vec![0; 5]; // First row of 0s
    for chunk in bitmap.data.chunks(4) {
        let mut row = Vec::from(chunk);
        row.push(0); // Add new column to each row
        new_data.extend(row);
    }
    new_data.extend_from_slice(&[0; 5]);
    bitmap = Bitmap {
        width: 5,
        height: 10,
        data: new_data
    };

    // Add outline
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let cell = bitmap.get(x, y);
            if cell == 0 && count_neighbors(&bitmap, x, y) > 0 {
                *bitmap.get_mut(x, y) = 2; // Mark as outline
            }
        }
    }

    // Mirror the data on the OY axis
    let mut mirrored = Vec::with_capacity(10 * 10);
    for chunk in bitmap.data.chunks(5) {
        let mut row: Vec<u8> = chunk.iter().rev().cloned().collect();
        row.extend(chunk);
        mirrored.extend(row);
    }
    Bitmap {
        width: 10,
        height: 10,
        data: mirrored,
    }
}
