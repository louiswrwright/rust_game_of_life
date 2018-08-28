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

    start_game(grid);
}

fn initialize_grid(mut grid: &mut [[Cell; 8]; 8], start_position_x: usize, start_position_y: usize){

    //BLINKER
//    ((*grid)[start_position_x][start_position_y]).on = true;
//    ((*grid)[start_position_x-1][start_position_y]).on = true;
//    ((*grid)[start_position_x+1][start_position_y]).on = true;

    //BEACON
    ((*grid)[start_position_x-1][start_position_y-1]).on = true;
    ((*grid)[start_position_x-1][start_position_y]).on = true;
    ((*grid)[start_position_x-2][start_position_y-1]).on = true;

    ((*grid)[start_position_x-4][start_position_y+2]).on = true;
    ((*grid)[start_position_x-3][start_position_y+2]).on = true;
    ((*grid)[start_position_x-4][start_position_y+1]).on = true;

//    println!()
}

fn start_game(mut grid: [[Cell; 8]; 8]) {
    use std::{thread, time};
    initialize_grid(&mut grid, 4 as usize,4 as usize);

    while true == true {
        render_game_view(grid);
        grid = game_tick(grid);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn game_tick(mut grid: [[Cell; 8]; 8]) -> [[Cell;8] ;8] {
    let mut second_grid: [[Cell; 8]; 8] = [[Cell {on: false}; 8]; 8];
    second_grid.clone_from_slice(&grid);

    for (index, col) in second_grid.iter_mut().enumerate() {
        for (sub_index, cell) in col.iter_mut().enumerate() {
            cell.on = evaluate_cell_state(grid, index, sub_index);
        }
    }
//    render_game_view(second_grid);

    return second_grid;
}

fn render_game_view(grid: [[Cell; 8]; 8]) {
    for (index, col) in grid.iter().enumerate() {
        for (sub_index, cell) in col.iter().enumerate() {
            if cell.on {
                print!("{}", '■');
            } else {
                print!("{}", '□');
            }
        }
        println!();
    }
}

fn evaluate_cell_state(mut grid: [[Cell; 8]; 8], x: usize, y: usize) -> bool {
    let cell: Cell = grid[x][y];
    let neighbour_count: usize = get_live_neighbour_count(grid, x, y);

    if cell.on {
        if neighbour_count < 2 || neighbour_count > 3 {
            return false;
        } else {
            return true;
        }
    } else {
        if neighbour_count == 3 {
            return true;
        }
    }

    return false;
}

fn get_neighbours(grid: [[Cell; 8]; 8], x: usize, y:usize) -> Vec<Cell> {
    let mut neighbours = Vec::new();

    let horizontal_neighbours: Vec<Cell> = get_horizontal_neighbours(grid, x, y);
    let vertical_neighbours: Vec<Cell> = get_vertical_neighbours(grid, x, y);
    let diagonal_neighbours: Vec<Cell> = get_diagonal_neighbours(grid, x, y);

    for(index, neighbour) in horizontal_neighbours.iter().enumerate() {
        neighbours.push(*neighbour);
    }

    for(index, neighbour) in vertical_neighbours.iter().enumerate() {
        neighbours.push(*neighbour);
    }

    for(index, neighbour) in diagonal_neighbours.iter().enumerate() {
        neighbours.push(*neighbour);
    }

    return neighbours;
}

fn get_horizontal_neighbours(grid: [[Cell; 8]; 8], x: usize, y: usize) -> Vec<Cell> {
    let mut horizontal_neighbours: Vec<Cell> = Vec::new();

    if x < 8 - 1 {
        horizontal_neighbours.push(grid[x+1][y]);
    }

    if x > 0 {
        horizontal_neighbours.push(grid[x-1][y]);
    }

    return horizontal_neighbours;
}

fn get_vertical_neighbours(grid: [[Cell; 8]; 8], x: usize, y: usize) -> Vec<Cell> {
    let mut vertical_neighbours: Vec<Cell> = Vec::new();

    if y < 8 - 1 {
        vertical_neighbours.push(grid[x][y+1]);
    }

    if y > 0 {
        vertical_neighbours.push(grid[x][y-1]);
    }

    return vertical_neighbours;
}

fn get_diagonal_neighbours(grid: [[Cell; 8]; 8], x: usize, y: usize) -> Vec<Cell> {
    let mut diagonal_neighbours: Vec<Cell> = Vec::new();

    if x > 0 && y < 7 {
        diagonal_neighbours.push(grid[x-1][y+1]);
    }

    if x < 7 && y < 7 {
        diagonal_neighbours.push(grid[x+1][y+1]);
    }

    if y > 0 && x > 0 {
        diagonal_neighbours.push(grid[x-1][y-1]);
    }

    if y > 0 && x < 7 {
        diagonal_neighbours.push(grid[x+1][y-1]);
    }

    return diagonal_neighbours;
}

fn get_live_neighbour_count(grid: [[Cell; 8]; 8], x: usize, y: usize) -> usize {
    let neighbours: Vec<Cell> = get_neighbours(grid, x, y);

    let mut live_neighbours: usize = 0;

    for(index, neighbour) in neighbours.iter().enumerate() {
//        if x == 4 as usize && y == 4 as usize {
//            println!("Left Cell is {}", grid[3][4].on);
//            println!("Centre Cell is {}", grid[4][4].on);
//            println!("Right Cell is {}", grid[5][4].on);
//        }
        if neighbour.on {
            live_neighbours += 1;
        }
    }

//    println!("Cell at {} {} has {} live neighbours", x, y, live_neighbours);

    return live_neighbours;
}

//fn fewer_than_two_live_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8) -> bool {
//    return get_live_neighbour_count(grid, x, y) < 2; //die scenario
//}
//
//fn two_or_three_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8) -> bool {
//    let neighbour_count: i8 = get_live_neighbour_count(grid, x, y);
//
//    return neighbour_count <= 3 && neighbour_count >= 2; //remain alive scenario
//}
//
//fn more_than_three_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8) -> bool {
//    return get_live_neighbour_count(grid, x, y) > 3; //die scenario
//}
//
//fn three_live_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8) -> bool {
//    return get_live_neighbour_count(grid, x, y) == 3; //birth scenario
//}

//fn dead_and_three_live_neighbours(grid: [[Cell; 8]; 8], x: i8, y: i8) -> bool {
//    return get_live_neighbour_count(grid, x, y) < 2; //birth scenario
//    // TODO: this
//}
