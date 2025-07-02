type Cell = u8;
const SIZES_COUNT: usize = 4;
const DEFAULT_SIZE: usize = 9;
const SIZES: [usize; SIZES_COUNT] = [6, 9, 12, 16];

pub struct Sudoku {
    grid_size: usize,
    quadrants_in_row: u8,
    quadrants_in_column: u8,
    cells: Vec<Cell>,
}

impl Sudoku {
    pub fn new(grid_size: usize) -> Self {
        let size: usize = if SIZES.contains(&grid_size) {
            grid_size
        } else {
            DEFAULT_SIZE
        };
        let quadrants_in_row: f64 = (size as f64).sqrt().ceil();
        let quadrants_in_column: f64 = (size as f64).sqrt().floor();
        Sudoku {
            grid_size: size,
            quadrants_in_row: quadrants_in_row as u8,
            quadrants_in_column: quadrants_in_column as u8,
            cells: vec![0; size * size],
        }
    }
    pub fn print(&self) {
        println!(
            "Sudoku grid: grid size={}, in column={}, in row={}",
            self.grid_size, self.quadrants_in_column, self.quadrants_in_row
        );
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                print!("{} ", self.cells[i * self.grid_size + j]);
            }
            println!();
        }
    }
}
