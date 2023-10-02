/*
Créez un programme qui affiche la position de l’élément le plus en haut à gauche (dans l’ordre) d’une forme au sein d’un plateau.

Exemples d’utilisation :
$> cat board.txt
0000
1111
2331
$> cat to_find.txt
11
 1
$> cat unfindable.txt
00
00

$> ruby exo.rb board.txt to_find.txt
Trouvé !
Coordonnées : 2,1
----
--11
---1

$> ruby exo.rb board.txt unfindable.txt
Introuvable

Vous devez gérer les potentiels problèmes d’arguments et de lecture de fichiers.
*/

use std::collections::VecDeque;
fn format_file_input(input: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for elm in input.trim().split('\n') {
        output.push(elm.trim().to_string());
    }
    output
}

fn find_str(item: &&str, line: &str, index: usize) -> Option<Vec<(usize, usize)>> {
    let mut substr: String = String::new();
    let mut idx: Vec<(usize, usize)> = vec![];
    let mut k: usize = 0;
    let sub: Vec<char> = line.chars().collect();
    while k != line.chars().count() {
        substr.push(sub[k]);
        if item.len() == substr.len() {
            if substr.as_str() == *item {
                idx.push((index, k));
            } else {
                if k > 2 {
                    if k % 2 == 0 {
                        k -= 2;
                    } else {
                        k -= 1;
                    }
                }
            }
            substr.clear();
        }
        k += 1;
    }
    
    if idx.len() > 0 {
        Some(idx)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = std::env::args().collect();
    if args.len() == 3 {
        args.pop_front();
        if let (Some(fn1), Some(fn2)) = (args.pop_front(), args.pop_front()) {
            if let Ok(board) = std::fs::read_to_string(fn1) {
                if let Ok(to_find) = std::fs::read_to_string(fn2) {
                    
                    // Board
                    let pattern = format_file_input(board);
                    let pattern: Vec<&str> = pattern.iter().map(|x| x.as_ref()).collect();
                    
                    // Token to find
                    let input = format_file_input(to_find);
                    let input: Vec<&str> = input.iter().map(|x| x.as_ref()).collect();

                    // Result
                    let mut output: Vec<String> = Vec::new();
                    
                    // Tmp vars
                    let mut lineout: String = String::new();
                    let mut saved_idx: Vec<Vec<usize>> = Vec::new();

                    for item in input.iter() {
                        for (i, line) in pattern.iter().enumerate() {
                            if line.contains(item) {
                                if line.len() == item.len() {
                                    output.push(line.to_string());
                                } else {
                                    if let Some(idx) = find_str(item, line, i) {
                                        let index = idx.last().unwrap();
                                        let mut sortline: VecDeque<char> = VecDeque::new();
                                        let mut exists = false;
                                        for l in saved_idx.iter() {
                                            if l.contains(&index.0) {
                                                exists = true;
                                            }
                                        }
                                        if !exists {
                                            if let Some(substr) = line.get(index.1-(item.len()-1)..index.1+1) {
                                                while sortline.len() != line.len() {
                                                    sortline.push_front('-');
                                                }
                                                let rng = index.1-(substr.len()-1)..index.1+1;
                                                for (i, chr) in line.chars().enumerate() {
                                                    if rng.contains(&i) {
                                                        sortline[i] = chr;
                                                    }
                                                }
                                                for chr in sortline {
                                                    lineout.push(chr);
                                                }
                                                output.push(lineout.clone());
                                                lineout.clear();
                                                saved_idx.push(vec![index.0, index.1]);
                                            } else {
                                                println!("index error");
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }      
                            }
                        }
                    }

                    let mut tmp: (usize, usize) = (0, 0);
                    let mut output_idx: Vec<Vec<usize>> = Vec::new();
                    
                    // removes duplicate
                    for val in saved_idx.iter() {
                        if tmp.0 == val[0] && tmp.1 == val[1] || tmp.0 == val[0] {
                            continue;
                        } else {
                            output_idx.push(vec![val[0], val[1]]);
                        }
                        tmp = (val[0], val[1]);
                    }
                    
                    // sort output to match initial board
                    for (i, line) in pattern.iter().enumerate() {
                        let mut found = false;
                        for idx in output_idx.iter() {
                            if idx[0] == i {
                                found = true;
                            }
                        }
                        if found {
                            continue;
                        } else {
                            if i < output.len() && output[i].len() > 0  {
                                output.push(line.to_string());
                                while output[i] != line.to_string() {
                                    output.rotate_right(1);
                                }
                            } else {
                                output.push(line.to_string());
                            }
                        }
                    }
                    
                    // show result
                    if output.len() > 0 && output_idx.len() == input.len() {
                        let first = output_idx.first().unwrap();
                        println!("Trouvé !");
                        println!("Coordonnées : {},{}", first[1]-1, first[0]);
                        for line in output.iter() {
                            println!("{}", line);
                        }
                    } else {
                        println!("Introuvable");
                    }

                } else {
                    println!("error (reading file 2)");
                    std::process::exit(1);
                }
            } else {
                println!("error (reading file 1)");
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