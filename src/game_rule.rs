pub struct GameRule {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl GameRule {
    pub fn set_alive(&mut self, x: usize, y: usize) {
        self.cells[y][x] = true;
    }

    pub fn step(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbours = self.count_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = new_cells;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for j in (y as isize - 1)..=(y as isize + 1) {
            for i in (x as isize - 1)..=(x as isize + 1) {
                if i >= 0 && i < self.width as isize && j >= 0 && j < self.height as isize {
                    if !(i as usize == x && j as usize == y) && self.cells[j as usize][i as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
