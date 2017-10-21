mod grid;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(16, 16);
    grid.randomize();
    grid.run();
}
