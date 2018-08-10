trait Sized1 : Sized{}

#[derive(Copy, Clone)]
struct Cell {
    on: bool
}
impl Sized1 for Cell {}

fn main() {
    use std::io::{stdin,stdout,Write};
    let mut array_length = String::new();
    print!("Enter grid length");
    let _ = stdout().flush();
    stdin().read_line(&mut array_length);

    let mut array_width = String::new();
    print!("Enter grid width");
    let _ = stdout().flush();
    stdin().read_line(&mut array_width);
    print!("{}", array_length);

    let mut grid = [[Cell {on: false}; 8]; 8];

    start_game(grid);
}

fn initialize_grid(start_position_x: i16, start_position_y: i16){

}

fn start_game(grid: [[Cell; 8]; 8]) {
    use std::{thread};
    initialize_grid(4,4);

//    while true == true {
        game_tick(grid);
        thread::sleep_ms(250);
//    }
}

fn game_tick(mut grid: [[Cell; 8]; 8]) {
    let second_grid = [[Cell {on: false}; 8]; 8];
    grid.clone_from_slice(&second_grid);

    for (index, row) in second_grid.iter().enumerate() {
        for (sub_index, cell) in row.iter().enumerate() {
            println!("Index 1: {}, Index 2: {}", index, sub_index);
        }
    }

}

fn fewer_than_two_live_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8){
    
}