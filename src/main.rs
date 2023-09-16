#![feature(iter_intersperse)]
use std::num::NonZeroU8;
use std::fmt::{self, Display};

type Digit = NonZeroU8; // TODO may use an enum with 9 variants if we're feeling extra spicy

type Cell = Option<Digit>;

#[derive(Debug, Clone)]
struct Grid {
    cells: [[Cell; 9]; 9], // TODO I am using magic numbers. Deal with it.
}

impl Grid {

    // TODO better error
    fn from_string(spec: &str) -> Result<Grid, &'static str> {
        let mut cells: [[Cell; 9]; 9] = [[None; 9]; 9];
        let mut lines = spec.lines();
        for row in &mut cells {
            let line = lines.next().ok_or("too few lines")?;
            if line.len() < 9 {
                return Err("line too short");
            } else if line.len() > 9 {
                return Err("extra characters at end of line");
            }
            for (i, c) in line.chars().enumerate() {
                match c {
                    ' ' => {},
                    '1'..='9' => { row[i] = Some(c.to_digit(10).and_then(|d| (d as u8).try_into().ok()).unwrap()); }
                    _ => { return Err("unexpected character"); }
                }
            }
        }
        if lines.next().is_some() {
            return Err("too many lines");
        }

        Ok(Grid{cells})
    }

    fn is_row_ok(&self, r: u8) -> bool {
        cells_ok(&self.cells[r as usize])
    }

    fn is_col_ok(&self, c: u8) -> bool {
        cells_ok(self.cells.iter().map(|row| &row[c as usize]))
    }

    fn is_house_ok(&self, h: u8) -> bool {
        let r = ((h / 3) * 3) as usize;
        let c = ((h % 3) * 3) as usize;
        let cells = &self.cells;
        
        cells_ok((r..r+3).flat_map(|r| &cells[r][c..c+3]))
    }

    fn is_ok(&self) -> bool {
        (0..9).all(|i| self.is_row_ok(i) && self.is_col_ok(i) && self.is_house_ok(i))
    }

    fn is_solved(&self) -> bool {
        self.cells.iter().flatten().all(|c| c.is_some())
            && self.is_ok()
    }

    fn solved(&self) -> Option<Self> {
        let mut coords = (0..9usize).flat_map(|r| (0..9usize).map(move |c| (r, c)));
        let unfilled = coords.find(|&(r, c)| self.cells[r][c].is_none());

        if let Some((r, c)) = unfilled {
            for digit in 1..=9 {
                let mut new_grid = self.clone();
                new_grid.cells[r][c] = Some(digit.try_into().unwrap());
                let (r, c) = (r as u8, c as u8);
                if !(new_grid.is_row_ok(r) && new_grid.is_col_ok(c) && new_grid.is_house_ok(Self::house(r, c))) {
                    continue;
                }
                if let Some(solved) = new_grid.solved() {
                    return Some(solved);
                }
            }

            None
        } else {
            Some(self.clone())
        }
    }

    fn house(r: u8, c: u8) -> u8 {
        (r / 3) * 3 + (c / 3)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let show_cell = |c: &Cell| c.map_or('.', |n| char::from_digit(u8::from(n).into(), 10).unwrap());
        let is_alt = f.alternate();
        let show_row = |row: &[Cell]| {
            let row_chars = row.iter().map(show_cell);
            if is_alt {
                let mut separators = [' ', ' ', '│'].into_iter().cycle();
                row_chars.intersperse_with(|| separators.next().unwrap()).collect::<String>()
            } else {
                row_chars.intersperse(' ').collect::<String>()
            }
        };
        for (i, row) in self.cells.iter().enumerate() {
            if is_alt && (i == 2 || i == 5) {
                write!(f, "\x1b[4m{}\x1b[0m\n", show_row(row))?;
            } else {
                write!(f, "{}\n", show_row(row))?;
            }
        }

        Ok(())
    }
}

fn cells_ok<'a, I>(cell_iter: I) -> bool
where I: 'a + IntoIterator<Item = &'a Cell>
{
    let mut present = [false; 9];
    // filter `None` and unwrap `Some`
    for digit in cell_iter.into_iter().filter_map(|&x| x) {
        let index = u8::from(digit) as usize - 1;
        if present[index] {
            return false;
        }
        present[index] = true;
    }

    true
}

fn main() -> Result<(), &'static str> {
    println!("Hello, world!");

    let grid = Grid::from_string(concat!(
        "   89    \n",
        " 84 3  61\n",
        "  2 1 75 \n",
        "     18 9\n",
        "7 9 48   \n",
        "8 63   75\n",
        " 3  7 1  \n",
        " 4 15  97\n",
        "97   4  2"
    ))?;

    println!("{:#}", grid);

    println!("");
    println!("{:#}", grid.solved().unwrap());

    let grid_hard = Grid::from_string(concat!(
        "   4     \n",
        "    9 31 \n",
        " 2  574 6\n",
        "      7 4\n",
        " 7  6  2 \n",
        "  9      \n",
        "7 481  6 \n",
        " 63  5   \n",
        " 5   2   "
    ))?;
    println!("");
    println!("{:#}", grid_hard);

    println!("");
    println!("{:#}", grid_hard.solved().unwrap());

    Ok(())
}
