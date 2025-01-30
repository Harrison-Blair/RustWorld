use rand::prelude::*;

pub struct Herbavore {
    pub symbol: char,
    pub position: (usize, usize),
    pub has_reproduced: bool,
}

pub struct Carnivore {
    pub symbol: char,
    pub position: (usize, usize),
    pub has_reproduced: bool,
}

pub trait Organism {
    fn get_symbol(&self) -> char;
    fn get_position(&self) -> (usize, usize);

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>) -> Vec<(usize, usize, char)>;

    fn move_to_empty(&mut self, grid: &mut Vec<Vec<char>>);
    fn eat(&mut self, grid: &mut Vec<Vec<char>>);
    fn reproduce(&mut self, grid: &mut Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
        let mut offspring = Vec::new();
        let neighbors = self.get_neighbors(&grid);
        let mut empty_neighbors = Vec::new();
        for (x, y, cell) in neighbors {
            if cell == ' ' {
                empty_neighbors.push((x, y));
            }
        }

        if let Some(&(dx, dy)) = empty_neighbors.choose(&mut rand::thread_rng()) {
            grid[dx][dy] = self.get_symbol();
            offspring.push((dx, dy, self.get_symbol()));
        }
        offspring
    }

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

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
        let neighbors = self.get_neighbors(&grid);
        let mut can_move = false;
        let mut can_eat = false;
        let mut can_reproduce = false;
        let mut offspring = Vec::new();

        for cell in neighbors {
            if cell.2 == ' ' {
                can_move = true;
            } else if cell.2 == 'ϡ' {
                can_eat = true;
            } else if cell.2 == self.symbol {
                can_reproduce = true;
            }
        }

        let rng = rand::thread_rng().gen_range(0.0..=1.0);
        if rng < 0.01 && can_reproduce && can_move && self.has_reproduced == false {
            offspring = self.reproduce(grid);
        } else if rng < 0.45 && can_eat {
            self.eat(grid);
        } else if rng < 1.0 && can_move {
            self.move_to_empty(grid);
        }
        offspring
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
}

impl Organism for Carnivore {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn do_turn(&mut self, grid: &mut Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
        let neighbors = self.get_neighbors(&grid);
        let mut can_move = false;
        let mut can_eat = false;
        let mut can_reproduce = false;
        let mut offspring = Vec::new();

        for cell in neighbors {
            if cell.2 == ' ' {
                can_move = true;
            } else if cell.2 == 'Π' {
                can_eat = true;
            } else if cell.2 == self.symbol {
                can_reproduce = true;
            }
        }

        let rng = rand::thread_rng().gen_range(0.0..=1.0);
        if rng < 0.01 && can_reproduce && can_move && self.has_reproduced == false {
            offspring = self.reproduce(grid);
        } else if rng < 0.45 && can_eat {
            self.eat(grid);
        } else if rng < 1.0 && can_move {
            self.move_to_empty(grid);
        }
        offspring
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
}