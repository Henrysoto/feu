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

fn format_file_input(input: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for elm in input.trim().split('\n') {
        output.push(elm.trim().to_string());
    }
    output
}

// Must be a standard 9x9 board
fn check_board_length(input: &Vec<&str>) -> bool {
    if input.len() == 9 {
        for row in input.iter() {
            if row.chars().count() != 9 {
                return false;
            }
        }
    } else {
        return false;
    }

    true
}

fn subdiv_board(input: Vec<&str>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = Vec::new();
    
    let mut i: usize = 0;
    let mut k: usize = 3;
    let mut lap: usize = 0;

    let mut str_arr: Vec<String> = Vec::new();
    let mut str = String::new();

    while output.len() != 9 {
        if i % 3 == 0 && i > 0 {
            if k == 9 {
                k = 3;
                lap += 3;
                i = lap;
            } else {
                k += 3;
                i = lap;
            }
            output.push(str_arr.clone());
            str_arr.clear();
        }

        if i == 9 {
            str.push_str(&input[i-1][k-3..k]);
        } else {
            str.push_str(&input[i][k-3..k]);
        }
        str_arr.push(str.clone());
        str.clear();

        i += 1;
    }
    
    output
}

fn problem_count(input: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for region in input.iter() {          // chaque tableau de String
        for str in region.iter() { 
            for chr in str.chars() {
                if chr == '.' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_all(input: &Vec<Vec<String>>) -> Option<Vec<Vec<String>>> {

    /*  La règle du jeu  : 
            chaque ligne, colonne et région ne doit contenir qu’une seule fois tous les chiffres de un à neuf.
            Formulé autrement, chacun de ces ensembles doit contenir tous les chiffres de un à neuf.
        La solution :
            si '.' verifier verticalement ainsi qu'horizontalement qu'un chiffre X 
            allant de 1..9 ne soit pas present, si c'est le cas, vérifier la région autour de '.'
            une région est un carré 3x3, si le chiffre X n'est pas présent c'est le bon !
    */

    let mut output: Vec<Vec<String>> = input.clone();
    let mut count = 0;
    for (i, region) in input.iter().enumerate() {
        for (k, str) in region.iter().enumerate() {
            for (x, chr) in str.chars().enumerate() {
                if chr == '.' {
                    for rndm in '1'..='9' {
                        if !str.contains(rndm) {
                            // Check region (3x3)
                            if solve_region(&output[i], rndm) {
                                // Check line
                                if solve_row(&output, i, k, rndm) {
                                    // Check col
                                    if solve_col(&output, i, x, rndm) {
                                        let mut answer = String::new();
                                        for (pos, rhc) in output[i][k].chars().enumerate() {
                                            if pos == x {
                                                answer.push(rndm);
                                            } else {
                                                answer.push(rhc);
                                            }
                                        }
                                        output[i][k] = answer;
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if count == problem_count(input) {
        Some(output)
    } else {
        None
    }
    
}

fn solve_region(region: &Vec<String>, number: char) -> bool {
    
    let mut checkmark = true;
    for str in region.iter() {
        if str.contains(number) {
            checkmark = false;
        }
    } 
    
    checkmark
}

fn solve_row(input: &Vec<Vec<String>>, region_row: usize, line_row: usize, number: char) -> bool {

    let mut checkmark = true;
    let mut i: usize = region_row;

    while checkmark {
        if i % 3 == 0 {
            for region in input[i..i+2].iter() {
                for (line, str) in region.iter().enumerate() {
                    if line == line_row {
                        if str.contains(number) {
                            checkmark = false;
                            break;
                        }
                    }
                }
            }
            break;
        } else {
            i -= 1;
        }
    }

    checkmark
}

fn solve_col(input: &Vec<Vec<String>>, region_idx: usize, col: usize, number: char) -> bool {
    
    let mut checkmark = true;
    let offset: usize = region_idx % 3;
    
    for (pos, region) in input.iter().enumerate() {
        if pos % 3 == offset {
            for str in region.iter() {
                for (y, chr) in str.chars().enumerate() {
                    if y == col {
                        if chr == number {
                            checkmark = false;
                        }
                    } 
                }
            }
        }
    }

    checkmark
}

fn print_solution(input: &Vec<Vec<String>>) {
    let mut lap: usize = 0;
    let mut i: usize = 0;

    print!("\n");
    loop {
        for region in input[0+i..=2+i].iter() {
            print!("{}", region[lap]);
        }
        print!("\n");
        lap += 1;
        if lap % 3 == 0 {
            lap = 0;
            i += 3;
        }
        if i == 9 {
            break;
        }
    }
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = std::env::args().collect();
    if args.len() == 2 {
        args.pop_front();
        if let Some(fn1) = args.pop_front() {
            if let Ok(board) = std::fs::read_to_string(fn1) {
                
                // Board
                let pattern = format_file_input(board.clone());
                let pattern: Vec<&str> = pattern.iter().map(|x| x.as_ref()).collect();
                
                // Make sure board is standard 9*9 square
                if check_board_length(&pattern) {

                    // divides board to make regions (3*3)
                    let div_board = subdiv_board(pattern);

                    // Find solutions
                    if let Some(solution) = solve_all(&div_board) {
                        print!("{}\n---------", board);
                        print_solution(&solution);
                    } else {
                        println!("error (solving board)");
                        std::process::exit(1);
                    }
                } else {
                    println!("error (board dimensions)");
                    std::process::exit(1);
                }
            } else {
                println!("error (reading file)");
                std::process::exit(1);
            }
        } else {
            println!("error (file doesn't exist)");
            std::process::exit(1);
        }
    } else {
        println!("error");
        std::process::exit(1);
    }
    
    Ok(())
}