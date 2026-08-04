use std::io;

struct Coordinate {
    r: i32,
    c: i32,
}

impl Coordinate {
    fn neighbors(&self) -> [Coordinate; 4] {
        [
            Coordinate {r: self.r - 1, c: self.c},
            Coordinate {r: self.r + 1, c: self.c},
            Coordinate {r: self.r, c: self.c - 1},
            Coordinate {r: self.r, c: self.c + 1},
        ]
    }
}

struct Grid {
    h: i32,
    w: i32,
}

impl Grid {
    fn get_hw() -> Self {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut hw = input.split_whitespace();
    let h: i32 = hw.next().unwrap().parse().unwrap();
    let w: i32 = hw.next().unwrap().parse().unwrap();

    Self { h, w }
    }

    fn contains(&self, coord: &Coordinate) -> bool {
        1 <= coord.r && coord.r <= self.h && 1 <= coord.c && coord.c <= self.w
    }
}

fn main() {
    let grid = Grid::get_hw();
    
    for r in 1..=grid.h {
        let mut row_result = Vec::new();
        for c in 1..=grid.w {
            let current_pos = Coordinate{r, c};
            let mut valid_neighbor_count = 0;
            for neighbor in current_pos.neighbors() {
                if grid.contains(&neighbor) {
                    valid_neighbor_count += 1;
                }
            }
            row_result.push(valid_neighbor_count.to_string());
        }
        println!("{}", row_result.join(" "));
    }
}
//0133_abc458_B_2回目