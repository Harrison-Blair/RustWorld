use std::io;
mod organisms;

fn populate_grid(grid:&mut Vec<Vec<char>>, creatures:&mut Vec<Box<dyn organisms::Organism>>) {
    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            let s: f32 = rand::random();
            if s > 0.9 {
                let r: f32 = rand::random();
                if r < 0.33 {
                    *cell = 'ϡ';
                } else if r < 0.66 {
                    *cell = 'Δ';
                    creatures.push(Box::new(organisms::Carnivore{symbol: *cell, position: (x, y)}));
                } else {
                    *cell = 'Π';
                    creatures.push(Box::new(organisms::Herbavore{symbol: *cell, position: (x, y)}));
                }
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    let row_str = "-".repeat(grid[0].len());
    println!("+{}+", row_str);
    for row in grid {
        print!("|");
        for cell in row {
            print!("{}", cell);
        }
        print!("|");
        println!();
    }
    println!("+{}+", row_str);
}

fn main() {
    let mut creatures: Vec<Box<dyn organisms::Organism>> = Vec::new();

    println!("Hello, welcome to Rust World!");

    println!("Please enter the WIDTH of the grid: ");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Failed to read line");
    let width: usize = width.trim().parse().expect("Please type a number!");

    println!("Please enter the HEIGHT of the grid: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: usize = height.trim().parse().expect("Please type a number!");

    let mut grid = vec![vec![' '; width]; height];

    populate_grid(&mut grid, &mut creatures);
    
    print_grid(&grid);

    let mut iterations = 0;

    while iterations < 9000 {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        for creature in creatures.iter_mut() {
            creature.do_turn(&mut grid);
        }
        print_grid(&grid);
        iterations += 1;
    }

    print_grid(&grid);
}
