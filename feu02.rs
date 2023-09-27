/*
Créez un programme qui reçoit une expression arithmétique dans une chaîne de caractères et en retourne le résultat après l’avoir calculé.
Vous devez gérer les 5 opérateurs suivants : “+” pour l’addition, “-” pour la soustraction, “*” la multiplication, “/” la division et “%” le modulo.

Exemple d’utilisation :

$> ruby exo.rb “4 + 21 * (1 - 2 / 2) + 38”
42

Vous pouvez partir du principe que la chaîne de caractères donnée en argument sera valide.
*/

use std::collections::{VecDeque, HashMap};

// RPN implementation
fn eval_postfix_expr(arr: Vec<&str>) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let operators: Vec<char> = vec!['+', '-', '*', '/', '%'];
    
    for token in arr.iter() {
        if let Ok(x) = token.parse::<i32>() {
            stack.push(x);
        } else {
            let chr = token.chars().nth(0).unwrap();
            if operators.contains(&chr) {
                if let (Some(o1), Some(o2)) = (stack.pop(), stack.pop()) {
                    match chr {
                        '+' => stack.push(o2 + o1),
                        '-' => stack.push(o2 - o1),
                        '*' => stack.push(o2 * o1),
                        '/' => {
                            if o1 != 0 {
                                stack.push(o2 / o1);
                            } else {
                                return None;
                            }
                        },
                        '%' => {
                            if o1 != 0 {
                                stack.push(o2 % o1);
                            } else {
                                return None;
                            }
                        },
                        _ => return None,
                    }
                } else {
                    return None;
                }   
            }
        }
    }
    if stack.len() == 1 {
        Some(stack.pop().unwrap())
    } else {
        None
    }
}

// Shunting yard implementation
fn eval_infix_expr(arr: Vec<&str>) -> Option<Vec<String>> {
    let mut operators: HashMap<char, (i32, &str)> = HashMap::new();
    operators.insert('+', (1, "left"));
    operators.insert('-', (1, "left"));
    operators.insert('%', (3, "left"));
    operators.insert('*', (3, "left"));
    operators.insert('/', (3, "left"));

    let mut f_: Vec<String> = Vec::new(); // output
    let mut p_: Vec<char> = Vec::new(); // stack
    let mut paren: Vec<char> = Vec::new();

    for symbol in arr.iter() {
        if let Ok(x) = symbol.parse::<i32>() {
            f_.push(x.to_string());
        } else {
            let o1 = symbol.chars().nth(0).unwrap();
            if operators.contains_key(&o1) {
                let o1h = operators.get(&o1).unwrap();
                while !p_.is_empty() {
                    if let Some(op) = p_.pop() {
                        if op == '(' {
                            p_.push(op);
                            break;
                        }
                        let o2 = operators.get(&op).unwrap();
                        if (o1h.1 == "left" && o1h.0 <= o2.0) || (o1h.1 == "right" && o1h.0 < o2.0) {
                            f_.push(op.to_string());
                        } else {
                            p_.push(op);
                            break;
                        }
                    }
                }
                p_.push(o1);
            } else if o1 == '(' {
                p_.push(o1);
                paren.push(o1);
            } else if o1 == ')' {
                while let Some(op) = p_.pop() {
                    if op != '(' {
                        f_.push(op.to_string());
                    } else {
                        paren.pop();
                        break;
                    }
                }
            }
        }
    }
    while let Some(op) = p_.pop() {
        f_.push(op.to_string());
    }

    if paren.is_empty() {
        Some(f_)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = std::env::args().collect();
    if args.len() == 2 {
        args.pop_front();
        let arg = args.pop_front().unwrap();
        let mut emb: String = String::new();
        for chr in arg.trim().chars() {
            if chr == '(' || chr == ')' {
                emb.push(' ');
                emb.push(chr);
                emb.push(' ');
                continue;
            }
            emb.push(chr);
        }
        let expr: Vec<&str> = emb.split(" ").filter(|c| !c.is_empty()).collect();
        let infix_res = eval_infix_expr(expr);
        if let Some(infix) = infix_res {
            let tr_infix_res: Vec<&str> = infix.iter().map(|x|x.as_ref()).collect();
            match eval_postfix_expr(tr_infix_res) {
                Some(x) => println!("{}", x),
                _ => println!("error (div by zero)"),
            };
        } else {
            println!("error parentheses");
            std::process::exit(1);    
        }
    } else {
        println!("error");
        std::process::exit(1);
    }
    
    Ok(())
}
