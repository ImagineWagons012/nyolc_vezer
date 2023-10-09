use std::fs::File;
use std::io::prelude::*;
use std::env;

fn vezerek(n:usize, y:&mut Vec<i32>, i:usize, count:&mut u32, solutions:&mut Vec<Vec<i32>>, file:&mut File) -> std::io::Result<bool> {
    for lol in 0..n {
        match is_right(n, y, lol as isize, i) {
            Some(_) => {
                y[i] = lol as i32;
                if i == n-1 { 
                    *count += 1;
                    solutions.push(y.clone());
                    continue;
                }
                if vezerek(n, y, i+1, count, solutions, file).unwrap() {
                    if i == 0 {
                        println!("yes!!!!!");
                    }
                    return Ok(true);
                }
            },
            None => (),
        }
    }
    Ok(false)
}

fn is_right(_n: usize, y:&mut [i32], lol:isize, i:usize) -> Option<()> {
    if i == 0 { return Some(()); }
    for bruh in 0..i {
        if y[bruh] == lol as i32 {
            return None;
        }
        for j in 0..i {
            let test_against = y[j];
            if test_against == (lol - i as isize + j as isize) as i32 || test_against == (lol + i as isize - j as isize) as i32 {
                return None;
            }
        }
    }
    Some(())
}

fn print_solution(solution: &Vec<i32>) {
    for i in (0..solution.len()).rev() {
        for j in 0..solution.len() {
            if i as i32 != solution[j] {
                print!(" -");
            }
            else {
                print!(" x");
            }
        }
        println!();
    }
}

fn write_solution_to_file(solution: &Vec<i32>, file:&mut File) -> std::io::Result<()> {
    let mut buf = String::new();
    for i in (0..solution.len()).rev() {
        for j in 0..solution.len() {
            if i as i32 != solution[j] {
                buf += " -";
            }
            else {
                buf += " x";
            }
        }
        buf += "\n";
    }
    buf += "\n\n";
    file.write_all(buf.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut count = 0;
    let mut solutions = vec![];
    let n;
    let args = env::args().nth(1).expect("Not enough arguments provided.");
    n = args.parse().expect("Args not in correct format");
    let mut file = File::create(format!("{0}x{0}_megoldasok.txt", n))?;

    vezerek(n, &mut vec![0; n], 0, &mut count, &mut solutions, &mut file)?;
    println!("{}", count);
    for (i, solution) in solutions.iter().enumerate() {
        println!("{}. megoldás:", i+1);
        print_solution(solution);
        println!("\n\n");
    }
    println!("Nincs több megoldás.");
    
    for (i, solution) in solutions.iter().enumerate() {
        file.write_all(format!("{}. megoldás:\n", i+1).as_bytes())?;
        write_solution_to_file(solution, &mut file)?;
        file.write_all(b"\n\n")?;
    }
    file.write_all(b"Nincs t\xF6bb megold\xE1s.")?;


    Ok(())
}