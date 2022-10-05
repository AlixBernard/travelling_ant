use std::collections::{HashMap, VecDeque};

use travelling_ant::{Cell, CellState};

fn main() {
    let max_sum: u32 = travelling_ant::get_max_sum();
    let source_cell = travelling_ant::get_source_cell();
    let mut grid: HashMap<Cell, CellState> = HashMap::new();

    let mut cells2check: VecDeque<Cell> = VecDeque::from([source_cell]);
    grid.insert(source_cell, CellState::Attainable);
    let mut counter: u32 = 1;

    while cells2check.len() > 0 {
        let current_cell: Cell = cells2check.pop_front().unwrap();

        for adj_cell in travelling_ant::get_adjacent_cells(&current_cell) {
            let cell_checked: bool = match grid.get(&adj_cell).unwrap_or(&CellState::Uknown) {
                CellState::Attainable => true,
                CellState::NotAttainable => true,
                CellState::Uknown => false,
            };

            if !cell_checked && adj_cell.sum_of_digits() <= max_sum {
                cells2check.push_back(adj_cell);
                grid.insert(adj_cell, CellState::Attainable);
                counter += 1;
            }
        }
    }

    println!("The ant can reach a maximum of {counter} cells!");
}
