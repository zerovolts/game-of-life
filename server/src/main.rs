extern crate rand;

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            grid: vec![vec![false; width]; height]
        }
    }

    fn randomize_mut(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_mut(x, y, rand::random());
            }
        }
        self
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for element in row.iter() {
                print!("{} ", if *element == true { 1 } else { 0 });
            }
            println!();
        }
    }

    fn set_mut(&mut self, x: usize, y: usize, value: bool) -> Result<&mut Self, &str> {
        if x < self.width && y < self.height {
            (*self).grid[y][x] = value;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = Vec::new();
        let mut sides = 0;

        if x > 0 && y > 0 {
            neighbors.push(self.get(x - 1, y - 1));
        }
        if x > 0 {
            neighbors.push(self.get(x - 1, y));
        }
        if x > 0 && y < self.height {
            neighbors.push(self.get(x - 1, y + 1));
        }
        if y > 0 {
            neighbors.push(self.get(x, y - 1));
        }
        if y < self.height {
            neighbors.push(self.get(x, y + 1));
        }
        if x < self.width && y > 0 {
            neighbors.push(self.get(x + 1, y - 1));
        }
        if x < self.width {
            neighbors.push(self.get(x + 1, y));
        }
        if x < self.width && y < self.height {
            neighbors.push(self.get(x + 1, y + 1));
        }

        neighbors.iter()
            .map(|x| x.unwrap_or(false))
            .filter(|x| *x)
            .count()
    }

    fn step_mut(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.neighbors(x, y);

                if self.get(x, y).unwrap() {
                    if neighbors < 2 || neighbors > 3 {
                        self.set_mut(x, y, false);
                    }
                } else if neighbors == 3 {
                    self.set_mut(x, y, true);
                }
                //print!("{} ", neighbors);
            }
            //println!();
        }
        self
    }

    fn run(&mut self) {
        let mut count = 0;
        loop {
            count += 1;

            self.print();
            println!();
            self.step_mut();

            if count > 100 {
                //break;
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new(16, 16);
    grid.set_mut(2, 2, true).unwrap()
        .set_mut(1, 2, true).unwrap()
        .set_mut(3, 2, true).unwrap();
    grid.randomize_mut();
    grid.run();
}