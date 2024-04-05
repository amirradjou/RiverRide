use std::io::{stdout, Write};

use crossterm::{
    style::{Print},
    ExecutableCommand,
};

fn main() -> std::io::Result<()> {
    // or using functions
    let mut sc = stdout();
    sc.execute(Print("Styled text here."))?;

    Ok(())
}