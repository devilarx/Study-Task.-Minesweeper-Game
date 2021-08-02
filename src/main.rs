use std::error::Error;
use minesweeper_core::field::{Field};

fn print_field_numbers(field: &Field) -> Result<(), Box<dyn Error>>{
    for i in 0..10{
        for j in 0..10{
            if field.is_mine(j, i)?{
                print!("* ");
            }else{
                print!("{} ", field.get_mines_around(j, i)?);
            }
        }
        println!(" ");
    }
    Ok(())
}

fn print_field_cur_state(field: &Field) -> Result<(), Box<dyn Error>>{
    for i in 0..10{
        for j in 0..10{
            if field.is_closed(j, i)?{
                print!("# ");
            }else{
                if field.is_mine(j, i)?{
                    print!("* ");
                }else{
                    let ma = field.get_mines_around(j, i)?;
                    if ma == 0{
                        print!("  ");
                    }
                    else{
                        print!("{} ", ma);
                    }
                }
            }
        }
        println!(" ");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut field = Field::new(10, 10, 0, 0, 10)?;
    print_field_numbers(&field)?;
    println!("");
    print_field_cur_state(&field)?;
    println!("");
    field.open_cell(0, 0)?;
    print_field_numbers(&field)?;
    println!("");
    print_field_cur_state(&field)?;
    println!("");
    Ok(())
}
