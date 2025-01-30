use rand::prelude::*;

pub struct Herbavore {
    pub symbol: char,
    pub position: (usize, usize),
}

pub struct Carnivore {
    pub symbol: char,
    pub position: (usize, usize),
}

pub trait Organism {
    fn get_symbol(&self) -> char;
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, position: (usize, usize));

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>);

    fn move_to_empty(&mut self, grid: &mut Vec<Vec<char>>);
    fn eat(&mut self, grid: &mut Vec<Vec<char>>);
    fn reproduce(&mut self, grid: &mut Vec<Vec<char>>);

    fn get_neighbors(&self, grid: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
        let mut neighbors = Vec::new();
        let (x, y) = self.get_position();
            if x > 0 {
                neighbors.push((x - 1, y, grid[x - 1][y]));
            }
            if x < grid.len() - 1 {
                neighbors.push((x + 1, y, grid[x + 1][y]));
            }
            if y > 0 {
                neighbors.push((x, y - 1, grid[x][y - 1]));
            }
            if y < grid[0].len() - 1 {
                neighbors.push((x, y + 1, grid[x][y + 1]));
            }
        neighbors
    }
}

impl Organism for Herbavore {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        let mut food_neighbors = Vec::new();
        let mut herbavore_neighbors = Vec::new();

        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            } else if cell == 'ϡ' {
                food_neighbors.push((x, y));
            } else if cell == self.symbol {
                herbavore_neighbors.push((x, y));
            }
        }

        if !herbavore_neighbors.is_empty() && !empty_neighbors.is_empty() {
            self.reproduce(grid);
        } else if !food_neighbors.is_empty() {
            self.eat(grid);
        } else if !empty_neighbors.is_empty() {
            self.move_to_empty(grid);
        }
    }

    fn move_to_empty(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            }
        }
                
        if let Some(&(dx, dy)) = empty_neighbors.choose(&mut rand::thread_rng()) {
            grid[self.position.0][self.position.1] = ' ';
            grid[dx][dy] = self.symbol;
            self.position = (dx, dy);
        }
    }

    fn eat(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut food_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == 'ϡ' {
                food_neighbors.push((x, y));
            }
        }

        if let Some(&(dx, dy)) = food_neighbors.choose(&mut rand::thread_rng()) {
            grid[dx][dy] = ' ';
        }
    }

    fn reproduce(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            }
        }

        if let Some(&(dx, dy)) = empty_neighbors.choose(&mut rand::thread_rng()) {
            grid[dx][dy] = self.symbol;
        }
    }
}

impl Organism for Carnivore {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        let mut food_neighbors = Vec::new();
        let mut carnivore_neighbors = Vec::new();

        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            } else if cell == 'Π' {
                food_neighbors.push((x, y));
            } else if cell == self.symbol {
                carnivore_neighbors.push((x, y));
            }
        }

        if !carnivore_neighbors.is_empty() {
            self.eat(grid);
        } else if !empty_neighbors.is_empty() {
            self.move_to_empty(grid);
        }
    }

    fn move_to_empty(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            }
        }
                
        if let Some(&(dx, dy)) = empty_neighbors.choose(&mut rand::thread_rng()) {
            grid[self.position.0][self.position.1] = ' ';
            grid[dx][dy] = self.symbol;
            self.position = (dx, dy);
        }
    }

    fn eat(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut food_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == 'Π' {
                food_neighbors.push((x, y));
            }
        }

        if let Some(&(dx, dy)) = food_neighbors.choose(&mut rand::thread_rng()) {
            grid[dx][dy] = ' ';
        }
    }

    fn reproduce(&mut self, grid: &mut Vec<Vec<char>>) {
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            }
        }
        
        if let Some(&(dx, dy)) = empty_neighbors.choose(&mut rand::thread_rng()) {
            grid[dx][dy] = self.symbol;
        }
    }
}