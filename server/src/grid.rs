extern crate rand;

pub struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            grid: vec![vec![false; width]; height]
        }
    }

    pub fn clone(&self) -> Grid {
        Grid {
            width: self.width,
            height: self.height,
            grid: self.grid.to_vec()
        }
    }

    pub fn get_grid(&self) -> &[Vec<bool>] {
        self.grid.as_slice()
    }

    pub fn clear(&mut self) -> &mut Self {
        *self = Grid::new(self.width, self.height);
        self
    }

    pub fn randomize(&mut self) -> &mut Self {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_mut(x, y, rand::random());
            }
        }
        self
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    pub fn print(&self) {
        for row in self.grid.iter() {
            for element in row.iter() {
                print!("{} ", if *element == true { 1 } else { 0 });
            }
            println!();
        }
    }

    pub fn set_mut(&mut self, x: usize, y: usize, value: bool) -> Result<&mut Self, &str> {
        if x < self.width && y < self.height {
            (*self).grid[y][x] = value;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = Vec::new();

        if x > 0 && y > 0 {
            neighbors.push(self.get(x - 1, y - 1));
            //if self.get(x - 1, y - 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if x > 0 {
            neighbors.push(self.get(x - 1, y));
            //if self.get(x - 1, y).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if x > 0 && y < self.height {
            neighbors.push(self.get(x - 1, y + 1));
            //if self.get(x - 1, y + 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if y > 0 {
            neighbors.push(self.get(x, y - 1));
            //if self.get(x, y - 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if y < self.height {
            neighbors.push(self.get(x, y + 1));
            //if self.get(x, y + 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if x < self.width && y > 0 {
            neighbors.push(self.get(x + 1, y - 1));
            //if self.get(x + 1, y - 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if x < self.width {
            neighbors.push(self.get(x + 1, y));
            //if self.get(x + 1, y).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        if x < self.width && y < self.height {
            neighbors.push(self.get(x + 1, y + 1));
            //if self.get(x + 1, y + 1).unwrap_or(false) {print!("1")} else {print!("0")}
        }
        println!();
        //println!("{:?}", neighbors);

        neighbors.iter()
            .map(|x| x.unwrap_or(false))
            .filter(|x| *x)
            .count()
    }

    pub fn should_live(&self, x: usize, y: usize) -> Option<bool> {
        let value = self.get(x, y).unwrap();
        let neighbors = self.neighbors(x, y);
        //print!("{} ", neighbors);

        if value && (neighbors < 2 || neighbors > 3) {
            Some(false)
        } else if !value && neighbors == 3 {
            Some(true)
        } else {
            // stay as is
            None
        }
    }

    fn should_live_vn(value: bool, neighbors: usize) -> Option<bool> {
        if value && (neighbors < 2 || neighbors > 3) {
            Some(false)
        } else if !value && neighbors == 3 {
            Some(true)
        } else {
            // stay as is
            None
        }
    }

    pub fn step_mut(&mut self) -> &mut Self {
        let clone = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let result = match clone.should_live(x, y) {
                    Some(true) => self.set_mut(x, y, true),
                    Some(false) => self.set_mut(x, y, false),
                    None => Err("pass through"),
                };
            }
            println!();
        }
        println!();

        // println!("start");
        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         print!("{} ", if self.get(x, y).unwrap() {1} else {0});
        //     }
        //     println!();
        // }
        // println!();
        self
    }

    pub fn run(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_correct_field() {
        let mut grid = Grid::new(4, 4);
        assert!(grid.get(2, 2) == Some(false));
        grid.set_mut(2, 2, true);
        assert!(grid.get(2, 2) == Some(true));
    }

    #[test]
    fn cannot_get_out_of_bounds() {
        let mut grid = Grid::new(4, 4);
        println!("{:?}", grid.get(2, 2));
        assert!(grid.get(2, 2) == Some(false));
        assert!(grid.get(4, 0) == None);
        assert!(grid.get(0, 4) == None);
    }

    #[test]
    fn correct_neighbors() {
        let mut grid = Grid::new(5, 5);
        assert!(grid.neighbors(2, 2) == 0);
        grid.set_mut(1, 1, true);
        assert!(grid.neighbors(2, 2) == 1);
        grid.set_mut(2, 1, true);
        assert!(grid.neighbors(2, 2) == 2);
        grid.set_mut(3, 1, true);
        assert!(grid.neighbors(2, 2) == 3);
        grid.set_mut(1, 2, true);
        assert!(grid.neighbors(2, 2) == 4);
        grid.set_mut(3, 2, true);
        assert!(grid.neighbors(2, 2) == 5);
        grid.set_mut(1, 3, true);
        assert!(grid.neighbors(2, 2) == 6);
        grid.set_mut(2, 3, true);
        assert!(grid.neighbors(2, 2) == 7);
        grid.set_mut(3, 3, true);
        assert!(grid.neighbors(2, 2) == 8);
    }

    #[test]
    fn alive_should_live() {
        let mut grid = Grid::new(5, 5);
        grid.set_mut(2, 2, true);
        grid.set_mut(1, 3, true);
        grid.set_mut(3, 3, true);
        assert!(grid.should_live(2, 2) == None);
        assert!(Grid::should_live_vn(true, 2) == None);
        assert!(Grid::should_live_vn(true, 3) == None);
    }

    #[test]
    fn alive_should_die() {
        assert!(Grid::should_live_vn(true, 0) == Some(false));
        assert!(Grid::should_live_vn(true, 1) == Some(false));
        assert!(Grid::should_live_vn(true, 4) == Some(false));
        assert!(Grid::should_live_vn(true, 5) == Some(false));
        assert!(Grid::should_live_vn(true, 6) == Some(false));
        assert!(Grid::should_live_vn(true, 7) == Some(false));
        assert!(Grid::should_live_vn(true, 8) == Some(false));
    }

    #[test]
    fn dead_should_live() {
        assert!(Grid::should_live_vn(false, 3) == Some(true));
    }

    #[test]
    fn dead_should_die() {
        assert!(Grid::should_live_vn(false, 0) == None);
        assert!(Grid::should_live_vn(false, 1) == None);
        assert!(Grid::should_live_vn(false, 2) == None);
        assert!(Grid::should_live_vn(false, 4) == None);
        assert!(Grid::should_live_vn(false, 5) == None);
        assert!(Grid::should_live_vn(false, 6) == None);
        assert!(Grid::should_live_vn(false, 7) == None);
        assert!(Grid::should_live_vn(false, 8) == None);
    }

    #[test]
    fn clone_should_create_copy() {
        let mut grid = Grid::new(3, 3);
        grid.set_mut(1, 1, true);
        let mut grid_clone = grid.clone();
        grid.set_mut(0, 2, true);

        assert!(grid.get(1, 1) == grid_clone.get(1, 1));
        assert!(grid.get(0, 2) != grid_clone.get(0, 2));
    }
}
