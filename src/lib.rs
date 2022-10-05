//! # Travelling Ant
//!
//! Author: Alix Bernard
//!
//! Crate implementing strucures, enums, and functions to solve the
//! travelling ant problem.

use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
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

#[derive(Debug)]
pub enum CellState {
    Attainable,
    NotAttainable,
    Uknown,
}

/// Prompts from the user the value of `max_sum` to use.
pub fn get_max_sum() -> u32 {
    println!("Input max value of sum of digits: ");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let max_sum: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("Error: {e}"),
    };

    max_sum
}

/// Prompts from the user the coordinates x and y of the source cell.
pub fn get_source_cell() -> Cell {
    let mut input: String = String::new();

    loop {
        println!("Input source cell coordinates:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let values: Vec<u32> = input
            .split(',')
            .map(|val| match val.trim().parse() {
                Ok(num) => num,
                Err(e) => panic!("Error: {e}"),
            })
            .collect();

        if values.len() == 2 {
            return Cell::new(values[0], values[1]);
        }

        println!("Enter only 2 positive integers separated by a comma, e.g.: 8, 7")
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
