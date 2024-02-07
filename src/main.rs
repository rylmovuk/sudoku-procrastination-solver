#![feature(iter_intersperse)]
mod sudoku;
use sudoku::Grid;


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
