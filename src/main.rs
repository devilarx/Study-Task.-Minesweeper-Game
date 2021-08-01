use std::error::Error;
use minesweeper_core::field::{Field};

fn main() -> Result<(), Box<dyn Error>> {
    let mut field = Field::new(20, 20, 0, 0, 80)?;
    for i in 0..20{
        for j in 0..20{
            let flag = field.is_mine(j, i);
            if flag{
                print!("* ");
            }else{
                let ma = field.get_mines_around(j, i);
                if ma == 0{
                    print!("  ");
                }else{
                    print!("{} ", ma);
                }
            }
        }
        println!("");
    }

    Ok(())
}
