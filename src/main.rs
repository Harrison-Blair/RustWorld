use std::io;

fn print_grid(grid: &Vec<Vec<char>>) {
    let row_str = "-+".repeat(grid[0].len());
    for row in grid {
        println!("+{}", row_str);
        print!("|");
        for cell in row {
            print!("{}|", cell);
        }
        println!();
    }
    println!("+{}", row_str);
}

fn main() {
    println!("Hello, welcome to Rust World!");

    println!("Please enter the WIDTH of the grid: ");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Failed to read line");
    let width: u32 = width.trim().parse().expect("Please type a number!");

    println!("Please enter the HEIGHT of the grid: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: u32 = height.trim().parse().expect("Please type a number!");

    let mut grid = vec![vec![' '; width as usize]; height as usize];

    print_grid(&grid);
}
