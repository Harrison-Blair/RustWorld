# RustWorld
## How to run
Download the repository and open a terminal window within the extracted repository. Ensure you have [Rust](https://www.rust-lang.org/learn/get-started) installed, and run the following commands: 

`cargo build`

`cargo run`

## Criteria
Create a unique program of your own interest (please do not just find a tutorial or copy example code; that will receive 0 points for this portion of the assignment) of a minimum of 100 lines of code in the language that shows off the language's unique syntax and features. Try to write a program that demonstrates what makes the language special. Walk us through the source code. Explain the key parts of the source code well. Do not feel that you need to explain every line, just the lines that demonstrate something you want to show us about the language. Grading will be based on your completion of the program as well as your ability to explain what it is doing. If you cannot explain clearly what each line is doing, I have to assume you did not write the code.

**Target Language:** [Rust](https://www.rust-lang.org/)

## Design
RustWorld simulates an ecosystem full of `Plants` and `Organisms` (`Herbavore` and `Carnivore`) on a grid of a user defined size. The program then loops through each of the `Organisms` on the grid, and allows them to make their turn `Eating`, `Moving` or `Reproducing`. Rustworld repopulates the grid in the event of total `Organism` extinction. 

Target Features to Display:
- Memory Ownership
- Reference Counting
- General Speed
- Traits/Structs

### Organisms
`Organisms` are a trait that defines the shared functionality and behaviors of the structs they are attached to. Functions within the trait that deal with direct member data (e.g. `self.variable`) must be implimented within the struct's implimentation of the trait. This is why setters and getters, while having identical implimentations in both `Herbavore` and `Carnivore` must be defined seperately.

Plants do not do anything, except for exist to be food for `Herbavores`

All `organisms` can:
- Move to an adjacent empty square
- Eat an object that is their food type
- Reproduce *once* with another member of their type

Shared/Genericized functions
- `get_neighbors`
- `move_to_empty`
- `eat`
- `reproduce`

#### Herbavore
`Herbavores` eat `Plants`.

Implimented Functions
- `get_symbol` <- Deal with member data
- `get_position` <- Deal with member data
- `set_position` <- Deal with member data
- `get_food` <- Deal with member data
- `do_turn` <- Cannot be genericized due to differences in Herbavores and Carnivores

#### Carnivore
`Carnivores` have a special characteristic called `Energy`. `Carnivores` spawn with a limited amount of `energy`, and must eat to increase their `energy`. `Energy` is expended when `moving` or `reproducing`, and in the event a `Carnivore` runs out of `energy` they are removed from the grid and list of creatures. `Carnivores` eat `Herbavores`.

Implimented Functions
- `get_symbol`
- `get_position`
- `set_position`
- `get_food`
- `do_turn`
