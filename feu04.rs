/*
Créez un programme qui trouve et affiche la solution d’un Sudoku.

Exemples d’utilisation :
$> cat s.txt
1957842..
3.6529147
4721.3985
637852419
8596.1732
214397658
92.418576
5.8976321
7612358.4

$> ruby exo.rb s.txt
195784263
386529147
472163985
637852419
859641732
214397658
923418576
548976321
761235894

Afficher error et quitter le programme en cas de problèmes d’arguments.
*/

use std::collections::VecDeque;

const DOT: char = '.';
const EMPTY: usize = 0;
const N: usize = 9;

fn format_file_input(input: String) -> Vec<Vec<usize>> {
    let mut strarray: Vec<Vec<String>> = Vec::new();
    for (i, elm) in input.trim().split('\n').enumerate() {
        strarray.push(Vec::new());
        strarray[i].push(elm.trim().to_string());
    }

    let mut output: Vec<Vec<usize>> = Vec::new();
    for (i, elm) in strarray.iter().enumerate() {
        output.push(Vec::new());
        for str in elm.iter() {
            for chr in str.chars() {
                if chr == DOT {
                    output[i].push(0);
                } else {
                    output[i].push(chr.to_digit(10).unwrap() as usize);
                }
            }
        }
    }
    output
}

fn find_empty_col(grid: &mut Vec<Vec<usize>>) -> Option<(usize, usize)> {
    for row in 0..N {
        for col in 0..N {
            if grid[row][col] == EMPTY {
                return Some((row, col));
            }
        }
    }

    None
}

fn is_input_valid(grid: &mut Vec<Vec<usize>>, row: usize, col: usize, num: usize) -> bool {
    return !row_exists(grid, row, num) 
        && !col_exists(grid, col, num) 
        && !region_exists(grid, row - row % 3, col - col % 3, num) 
        && grid[row][col] == EMPTY;
}

fn row_exists(grid: &mut Vec<Vec<usize>>, row: usize, num: usize) -> bool {
    for col in 0..N {
        if grid[row][col] == num {
            return true;
        }
    }

    false
}

fn col_exists(grid: &mut Vec<Vec<usize>>, col: usize, num: usize) -> bool {
    for row in 0..N {
        if grid[row][col] == num {
            return true;
        }
    }

    false
}

fn region_exists(grid: &mut Vec<Vec<usize>>, start_row: usize, start_col: usize, num: usize) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if grid[row + start_row][col + start_col] == num {
                return true;
            }
        }
    }

    false
}

fn solve_grid(grid: &mut Vec<Vec<usize>>) -> bool {
    if let Some((row, col)) = find_empty_col(grid) {
        for num in 1..=9 {
            if is_input_valid(grid, row, col, num) {
                grid[row][col] = num;
                if solve_grid(grid) {
                    return true;
                }
                grid[row][col] = EMPTY;
            }
        }
        false
    } else {
        true
    }
}

fn print_grid(grid: Vec<Vec<usize>>) {
    for row in 0..N {
        for col in 0..N {
            print!("{} ", grid[row][col]);
        }
        print!("\n");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = std::env::args().collect();
    if args.len() == 2 {
        args.pop_front();
        if let Some(fn1) = args.pop_front() {
            if let Ok(board) = std::fs::read_to_string(fn1) {
                
                // Grid
                let mut grid = format_file_input(board.clone());

                print_grid(grid.clone());
                println!("-----------------");

                if solve_grid(&mut grid) {
                    print_grid(grid);
                } else {
                    println!("no solution");
                }
                
            } else {
                println!("error reading file");
                std::process::exit(1);
            }
        } else {
            println!("error");
            std::process::exit(1);
        }
    }

    Ok(())
}