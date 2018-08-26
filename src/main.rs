trait Sized1 : Sized{}

#[derive(Copy, Clone)]
struct Cell {
    on: bool
}
impl Sized1 for Cell {}

fn main() {
//    use std::io::{stdin,stdout,Write};
//    let mut array_length = String::new();
//    print!("Enter grid length");
//    let _ = stdout().flush();
//    stdin().read_line(&mut array_length);
//
//    let mut array_width = String::new();
//    print!("Enter grid width");
//    let _ = stdout().flush();
//    stdin().read_line(&mut array_width);
//    print!("{}", array_length);
    let mut grid = [[Cell {on: false}; 8]; 8];

    // "block"
    grid[4][4].on = true;
    grid[4][5].on = true;
    grid[3][5].on = true;

    // "blinker"
    grid[1][0].on = true;
    grid[1][1].on = true;
    grid[1][2].on = true;
    start_game(grid);
}

fn initialize_grid(start_position_x: i16, start_position_y: i16){

}

fn start_game(mut grid: [[Cell; 8]; 8]) {
    use std::{thread};
//    initialize_grid(4,4);

    print_grid(&grid);
    println!();
    while true == true {
        grid = game_tick(grid);

        print_grid(&grid);
        println!();
        thread::sleep_ms(1000);
    }
}

fn game_tick(mut grid: [[Cell; 8]; 8]) -> [[Cell; 8]; 8]{
    let mut second_grid = [[Cell { on: false }; 8]; 8];

    for (index, row) in grid.iter().enumerate() {
        for (sub_index, cell) in row.iter().enumerate() {
            let count = count_neighbours(&grid, index, sub_index);
            let current_cell_alive = grid[index][sub_index].on;

            if count < 2 && current_cell_alive {
                second_grid[index][sub_index].on = false;
            } else if (count == 2 || count == 3) && current_cell_alive {
                second_grid[index][sub_index].on = true;
            } else if count == 3 && !current_cell_alive {
                second_grid[index][sub_index].on = true;
            } else if count > 3 && current_cell_alive {
                second_grid[index][sub_index].on = false;
            }
        }
    }
    second_grid
}

fn print_grid(grid: &[[Cell; 8]; 8]){
    for (index, row) in grid.iter().enumerate() {
        for (sub_index, cell) in row.iter().enumerate() {
            if grid[index][sub_index].on {
                print!("*");
            }
            else {
                print!(".");
            }
        }
        println!()
    }
}

fn count_neighbours(grid: &[[Cell; 8]; 8], x: usize, y: usize) -> i32 {
    let mut count = 0;

    if x > 0 && y > 0 && grid[x-1][y-1].on {
        count+=1;
    }
    if y > 0 && grid[x][y-1].on {
        count+=1;
    }
    if x < 7 && y > 0 && grid[x+1][y-1].on {
        count+=1;
    }
    if x > 0 && grid[x-1][y].on {
        count+=1;
    }
    if x < 7 && grid[x+1][y].on {
        count+=1;
    }
    if x > 0 && y < 7 && grid[x-1][y+1].on {
        count+=1;
    }
    if y < 7 && grid[x][y+1].on {
        count+=1;
    }
    if x < 7 && y < 7 && grid[x+1][y+1].on {
        count+=1;
    }
    count
}
