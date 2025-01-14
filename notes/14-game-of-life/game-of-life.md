# Game of Life

# Project - Conway's Game of Life

![](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAJUAAACTCAIAAAAImcwFAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAALeSURBVHhe7d0/bhpBGEDxJdUegJrO1IjaknFPwQ3cWOIauPMZUnEEF7inoI3rlFS44Q5kzIzJH61EIu2O9k3eT5bykWa0fvJSfZrB6XSqhPUl/Ssm+8GF9+dFXdfpf9Vjw+EwBTudfvv+GwwGaVK/Xar5/mRr/vtbLKrJJI7tW6+r/T7Ns9lsPB6nD63abDaHwyHO3Z0SZDvo21mcL9Wa+4Vf8cNDHNt3f19tt2lerVbz+Tx9aNVyubw8bXenBNkO+noWZ9+fhbAfm/3Y7MdmPzb7sdmPzX5s9mOzH5v92OzHZj82+7HZj81+bPZjsx+b/djsx2Y/Nvux2Y/Nfmz2Y7Mfm/3Y7MdmP7bm/aO7u+rmJo7te32t3t/TPJ1OR6NR+tCq3W53PB7j3N0pQbaDvp/F+cr+mHrO/bFC2I+t+f2ZbeG4u0XtPFvaQZ7HCbbbn3vLV77/si0cd7eonWdLO8jzOMHTU3iQNPv9Vwj7sdmPzX5s9mOzH5v92OzHZj82+7HZj81+bPZjsx+b/djsx2Y/Nvux2Y/Nfmz2Y7Mfm/3Y7MdmPzb7sdmPzX5szfsr2RZWu1v0zbPlG+R5nODt7eMnurJ/pJ5z/6gQ9mNrfn9mu7+4sP3b/+7+4sL2b72/WP/Gfmz2Y7Mfm/3Y7MdmPzb7sdmPzX5s9mOzH5v92OzHZj82+7HZj81+bPZjsx+b/djsx2Y/Nvux2Y/Nfmz2Y7Mfm/3YmvePst1fXNj+tPcX66+4P1YI+7E1vz8LWDjOdq1wnnXwoI/3F3d3ULZrhfOsgwfeX1wg+7HZj81+bPZjsx+b/djsx2Y/Nvux2Y/Nfmz2Y7Mfm/3Y7MdmPzb7sdmPzX5s9mOzH5v92OzHZj82+7HZj81+bPZja94/WiwWk84Wodbr9f5z46q7g3495fGxur2NY/uen6vPreZuD3p5+fiJruyPqefcHyuE/Xjquk7TH+9P4fj3R1ZVPwB9bng/Ow6FrgAAAABJRU5ErkJggg==)

The game of life was developed by British mathematician John Horton Conway in 1970. It involves a grid (the world) of cells. The grid of cells evolves according to the following rules:

1. **Death**. If a cell is alive (state = 1) it will die (state becomes 0) under the following circumstances.

	- Overpopulation: If the cell has four or more alive neighbours, it dies.
	- Loneliness: If the cell has one or fewer alive neighbours, it dies.

2. **Birth**. If a cell is dead (state = 0) it will come to life (state becomes 1) if it has exactly three alive neighbours (no more, no less).

3. **Stasis**. In all other cases, the cell state does not change. To be thorough, let's describe those scenarios.

	- Staying Alive: If a cell is alive and has exactly two or three live neighbours, it stays alive.
	- Staying Dead: If a cell is dead and has anything other than three live neighbours, it stays dead.

We are going to implement a version of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).  The [starter code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=66c823de3b26810d205e540ad471dad9) for this project can be found below.

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    fn from_pattern(pattern: &[&str]) -> Grid {
        let height = pattern.len();
        let width = pattern[0].len();
        let cells = pattern
            .iter()
            .flat_map(|&row| row.chars())
            .map(|c| if c == 'O' { Cell::Alive } else { Cell::Dead })
            .collect();

        Grid {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            Cell::Dead
        } else {
            self.cells[y * self.width + x]
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        // Row above
        if x > 0 && y > 0 && self.get(x - 1, y - 1) == Cell::Alive { count += 1; }
        if y > 0 && self.get(x, y - 1) == Cell::Alive { count += 1 }
        if x < self.width - 1 && y > 0 && self.get(x + 1, y - 1) == Cell::Alive { count += 1; }
        //
        // TODO
        //
        count
    }

    fn next_generation(&self) -> Grid {
        let new_grid = self.clone();
        //
        // TODO
        //
        new_grid
    }

    fn output(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = match self.get(x, y) {
                    Cell::Alive => 'O',
                    Cell::Dead => '.',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}

  

fn main() {
    let pattern = vec![
        "........",
        "....O...",
        "...OO...",
        "...O....",
        "........"];
  
    let grid = Grid::from_pattern(&pattern);

    println!("Initial Generation:");
    grid.output();
    println!("Number of Neightbours: {}", grid.count_alive_neighbors(4,2) );
}
```

with the following output,

```
Initial Generation:
........
....O...
...OO...
...O....
........
Number of Neightbours: 1
```

**Note:**  The starter code only detects neighbours in the row above a particular cell.

**Note:** The `get()` function allows us to model a 2-dimensional grid using a 1-dimensional [vector](notes/05-vectors/vectors.md) in **Rust**.  See this [note](/notes/14-game-of-life/get.md) for a brief explanation of how it does this.