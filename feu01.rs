/*
Créez un programme qui affiche un rectangle dans le terminal.

Exemples d’utilisation :
$> python exo.py 5 3
o---o
|   |
o---o

$> python exo.py 5 1
o---o

$> python exo.py 1 1
o

Gérer les problèmes potentiels d’arguments.
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: std::collections::VecDeque<String> = std::env::args().collect();
    if args.len() == 3 {
        args.pop_front();
        let width = match args.pop_front() {
            Some(x) => {
                match x.parse::<usize>() {
                    Ok(x) => x,
                    _ => {
                        println!("error");
                        std::process::exit(1);                
                    }
                }
            },
            _ => {
                println!("error");
                std::process::exit(1);                
            }
        };
        let height = match args.pop_front() {
            Some(x) => {
                match x.parse::<usize>() {
                    Ok(x) => x,
                    _ => {
                        println!("error");
                        std::process::exit(1);                
                    }
                }
            },
            _ => {
                println!("error");
                std::process::exit(1);                
            }
        };
        for k in 0..height {
            if k == 0 || k == height-1 {
                for i in 0..width {
                    if i == 0 || i == width-1 {
                        print!("o");
                    } else {
                        print!("-");
                    }
                }
            } else {
                for i in 0..width {
                    if i == 0 || i == width-1 {
                        print!("|");
                    } else {
                        print!(" ");
                    }
                }
            }
            print!("\n");
        }
        print!("\n");
    } else {
        println!("error");
        std::process::exit(1);
    }
    
    Ok(())
}
