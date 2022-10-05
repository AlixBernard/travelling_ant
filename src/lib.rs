//! # Travelling Ant
//!
//! Author: Alix Bernard
//!
//! Crate implementing strucures, enums, and functions to solve the
//! travelling ant problem.

use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn sum_of_digits_x(&self) -> u32 {
        self.x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum()
    }

    pub fn sum_of_digits_y(&self) -> u32 {
        self.y
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum()
    }

    pub fn sum_of_digits(&self) -> u32 {
        self.sum_of_digits_x() + self.sum_of_digits_y()
    }
}

impl FromStr for Cell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = match s
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(',') {
                Some((a, b)) => (a, b),
                None => ("a", "b"),
                // ! If None return a value that will fail later,
                // ! it should however return an error directly
            };

        let x_fromstr = x.parse::<u32>()?;
        let y_fromstr = y.parse::<u32>()?;

        Ok(Self { x: x_fromstr, y: y_fromstr })
    }
}

#[derive(Debug)]
pub enum CellState {
    Attainable,
    NotAttainable,
    Uknown,
}

/// Prompts from the user the value of `max_sum` to use.
pub fn get_max_sum() -> u32 {
    loop {
        println!("Input max value of sum of digits: ");
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let max_sum: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return max_sum;
    }
}

/// Prompts from the user the coordinates x and y of the source cell.
pub fn get_source_cell() -> Cell {

    loop {
        println!("Input source cell coordinates as `(x,y)`:");

        let mut input = String::new();
        let trimmed_input: &str = match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim(),
            Err(_) => continue,
        };

        let cell: Cell = match trimmed_input.parse() {
            Ok(cell) => cell,
            Err(_) => continue,
        };
        
        return cell;
    }
}

/// Returns the adjacent cells that have positive coordinates
///
/// # Examples
///
/// ```
/// use travelling_ant::*;
///
/// let cell = Cell::new(0, 0);
/// let adjacent_cells = get_adjacent_cells(&cell);
///
/// assert_eq!(adjacent_cells.len(), 2);
/// assert!(adjacent_cells.contains(&Cell { x: 0, y: 1 }));
/// assert!(adjacent_cells.contains(&Cell { x: 1, y: 0 }));
/// ```
pub fn get_adjacent_cells(cell: &Cell) -> Vec<Cell> {
    let mut adjacent_cells: Vec<Cell> = Vec::new();

    adjacent_cells.push(Cell::new(cell.x + 1, cell.y));
    adjacent_cells.push(Cell::new(cell.x, cell.y + 1));
    if cell.x > 0 {
        adjacent_cells.push(Cell::new(cell.x - 1, cell.y));
    }
    if cell.y > 0 {
        adjacent_cells.push(Cell::new(cell.x, cell.y - 1));
    }

    adjacent_cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_sum_of_digits_is_23() {
        let cell = Cell::new(25, 2010184);
        assert_eq!(cell.sum_of_digits(), 23);
    }

    #[test]
    fn adjacent_cells_except_bottom() {
        let cell = Cell::new(1027, 0);
        let adjacent_cells = get_adjacent_cells(&cell);
        assert_eq!(adjacent_cells.len(), 3);
        assert!(adjacent_cells.contains(&Cell { x: 1026, y: 0 }));
        assert!(adjacent_cells.contains(&Cell { x: 1028, y: 0 }));
        assert!(adjacent_cells.contains(&Cell { x: 1027, y: 1 }));
    }
}
