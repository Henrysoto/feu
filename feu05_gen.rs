/*
Générateur pour l'exercice: 05 - Trouver le plus grand carré
*/

use std::collections::VecDeque;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = std::env::args().collect();
    if args.len() == 4 {
        args.pop_front();
        let x = args
            .pop_front()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let y = args
            .pop_front()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let density = args
            .pop_front()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        println!("{}.xo", y);
        for _i in 0..y {
            let mut rng = rand::thread_rng();
            for _j in 0..x {
                if rng.gen_range(0..y) * 2 < density {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    } else {
        println!("params needed: x y density");
        std::process::exit(1);
    }

    Ok(())
}