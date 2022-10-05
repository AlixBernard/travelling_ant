use std::collections::{HashMap, VecDeque};

use clap::Parser;

use travelling_ant::{Cell, CellState};


/// Program to solve the travelling ant problem
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Value of the maximum sum to not exceed
   #[arg(short, long)]
   max_sum: Option<u32>,

   /// Coordinates of the source cell
   #[arg(short, long, value_parser = clap::value_parser!(Cell))]
   source_cell: Option<Cell>,
}


fn main() {
    let args = Args::parse();
    let max_sum: u32 = match args.max_sum {
        Some(num) => num,
        None => travelling_ant::get_max_sum(),
    };
    let source_cell: Cell = match args.source_cell {
        Some(cell) => cell,
        None => travelling_ant::get_source_cell(),
    };

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
