use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_test_positive() {
        let mut vec = vec![0u32;4];
        assert!(solve(&vec).is_ok());
        vec = vec![0u32;9];
        assert!(solve(&vec).is_ok());
        vec = vec![0u32;81];
        assert!(solve(&vec).is_ok());
        vec = vec![0u32, 1, 0, 0];
        assert!(solve(&vec).is_ok());
        vec = vec![1u32, 2, 0, 0];
        assert!(solve(&vec).is_ok());
        vec = vec![1u32, 2, 0, 1];
        assert!(solve(&vec).is_ok());
        vec = vec![0u32, 1, 0, 2, 0, 1, 0, 2, 0];
        assert!(solve(&vec).is_ok());
        vec = vec![0u32, 0, 0, 0, 0, 1, 2, 0, 0, 3, 4, 0, 0, 0, 0, 0];
        assert!(solve(&vec).is_ok());
    }

    #[test]
    fn solve_test_negative() {
        let mut vec = vec![0u32, 1, 1, 0, 0, 0, 0, 0, 0];
        assert!(solve(&vec).is_err());
        vec = vec![0u32, 1, 0, 0, 0, 0, 0, 1, 0];
        assert!(solve(&vec).is_err());
        vec = vec![0u32, 0, 0, 2, 0, 2, 0, 0, 0];
        assert!(solve(&vec).is_err());
        vec = vec![0u32;8];
        assert!(solve(&vec).is_err());
        vec = vec![0u32, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0];
        assert!(solve(&vec).is_err());
        vec = vec![0u32, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 3, 4];
        assert!(solve(&vec).is_err());
    }

    #[test]
    fn possible_test_positive() {
        assert!(check_if_possible(&vec![1, 2, 0], 3));
        assert!(check_if_possible(&vec![0], 1));
        assert!(check_if_possible(&vec![1, 0], 3));
    }

    #[test]
    fn possible_test_negative() {
        assert!(!check_if_possible(&vec![1, 1], 1));
        assert!(!check_if_possible(&vec![1, 2, 1], 1));
    }

    #[test]
    // #[ignore]
    fn hard_to_bruteforce() {
        let test = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8, 5, 0, 0, 1, 0, 2, 0, 0,
                        0, 0, 0, 0, 0, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0,
                        0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 7, 3, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                        0, 4, 0, 0, 0, 9];
        assert!(solve(&test).is_ok());
    }


}

pub fn solve_old(puzzle: &mut Vec<u32>) -> Result<Vec<u32>, String> {

    let max_value = f64::sqrt(puzzle.len() as f64) as u32;
    if max_value * max_value != puzzle.len() as u32 {
        return Err("Not a square".to_owned());
    }

    let inner_square = f64::sqrt(max_value as f64) as u32;
    let check_inner_square = {
        inner_square * inner_square == max_value
    };

    let mut fixed_map: Vec<bool> = Vec::with_capacity(puzzle.len());


    for element in puzzle.iter() {
        if *element == 0 {
            fixed_map.push(false);
        } else {
            fixed_map.push(true);
        };

    }

    let mut index: u32 = 0;
    let mut forward = true;

    while index < puzzle.len() as u32 {
        if !fixed_map[index as usize] {
            puzzle[index as usize] += 1;
            if puzzle[index as usize] > max_value {
                if index == 0 {
                    return Err("No solution.".to_owned());
                }
                puzzle[index as usize] = 0;
                forward = false;
            } else {

                let row = index / max_value;
                let col = index % max_value;

                let cols = (0..max_value)
                    .map(|x| puzzle[(col + x * max_value) as usize])
                    .collect::<Vec<u32>>();
                let candidate = puzzle[index as usize];
                if !(check_if_possible(&puzzle[(row*max_value) as usize ..(row*max_value+max_value) as usize],candidate) &&
                    check_if_possible(&cols[..],candidate)
                    )
                {
                   // puzzle[index as usize] += 1;
                    continue;
                }

                let mut squares: Vec<u32> = Vec::with_capacity(max_value as usize);
                if check_inner_square {
                    let temp = index - index % inner_square;
                    let square = temp - ((temp / max_value) % inner_square) * max_value;
                    let square_row = (square / max_value) / inner_square;
                    let square_col = (square % max_value) / inner_square;

                    for a in 0..inner_square {
                        for b in 0..inner_square {
                            squares.push(puzzle[((square_row*inner_square*max_value)+(inner_square*square_col)+b+(a*max_value))as usize]);
                        }
                    }
                }

                if !(!check_inner_square || check_if_possible(&squares[..], candidate)) {
                    // puzzle[index as usize] += 1;
                    continue;
                }

                // puzzle[index as usize] += 1;
                forward = true;
            }
        }
        if forward {
            index += 1;
        } else {
            if index > 0 {
                index -= 1;
            } else {
                return Err("No solution.".to_owned());
            }
        }
    }

    Ok(puzzle.clone())
}

pub fn solve(puzzle_in: &Vec<u32>) -> Result<Vec<u32>, String> {

    let puzzle = puzzle_in.clone();

    let max_value = f64::sqrt(puzzle.len() as f64) as u32;
    if max_value * max_value != puzzle.len() as u32 {
        return Err("Not a square".to_owned());
    }

    let inner_square = f64::sqrt(max_value as f64) as u32;
    let check_inner_square = {
        inner_square * inner_square == max_value
    };

    let mut fixed_map: Vec<bool> = Vec::with_capacity(puzzle.len());

    for element in puzzle.iter() {
        if *element == 0 {
            fixed_map.push(false);
        } else {
            fixed_map.push(true);
        };

    }

    let mut reversed_puzzle = puzzle.clone();
    reversed_puzzle.reverse();
    let mut fixed_map_reverse = fixed_map.clone();
    fixed_map_reverse.reverse();

    let anwser = false;
    let anwser_reverse = false;

    let mut jobs = Vec::new();
    jobs.push(Arc::new(Mutex::new((puzzle, fixed_map, anwser))));
    jobs.push(Arc::new(Mutex::new((reversed_puzzle, fixed_map_reverse, anwser_reverse))));

    let cont = Arc::new(AtomicBool::new(true));

    let mut threads = Vec::new();
    for i in 0..jobs.len() {
        let data = jobs[i].clone();
        let cont = cont.clone();
        threads.push(thread::spawn(move || {
        
        let (ref mut puzzle, ref mut fixed_map, ref mut anwser) = *data.lock().unwrap();
        
        let mut index: u32 = 0;
        let mut forward = true;
    
        while index < puzzle.len() as u32 {
            if !cont.load(Ordering::SeqCst) {
                    return;
                }
            if !fixed_map[index as usize] {
                puzzle[index as usize] += 1;
                if puzzle[index as usize] > max_value {
                    if index == 0 {
                        return;
                    }
                    puzzle[index as usize] = 0;
                    forward = false;
                } else {

                    let row = index / max_value;
                    let col = index % max_value;

                    let cols = (0..max_value)
                        .map(|x| puzzle[(col + x * max_value) as usize])
                        .collect::<Vec<u32>>();
                    let candidate = puzzle[index as usize];
                    if !(check_if_possible(&puzzle[(row*max_value) as usize ..(row*max_value+max_value) as usize],candidate) &&
                        check_if_possible(&cols[..],candidate)
                        )
                    {
                        continue;
                    }

                    let mut squares: Vec<u32> = Vec::with_capacity(max_value as usize);
                    if check_inner_square {
                        let temp = index - index % inner_square;
                        let square = temp - ((temp / max_value) % inner_square) * max_value;
                        let square_row = (square / max_value) / inner_square;
                        let square_col = (square % max_value) / inner_square;

                        for a in 0..inner_square {
                            for b in 0..inner_square {
                                squares.push(puzzle[((square_row*inner_square*max_value)+(inner_square*square_col)+b+(a*max_value))as usize]);
                            }
                        }
                    }

                    if !(!check_inner_square || check_if_possible(&squares[..], candidate)) {
                        continue;
                    }

                    forward = true;
                }
            }
            if forward {
                index += 1;
            } else {
                if index > 0 {
                    index -= 1;
                } else {
                    return;
                }
            }
        }
        *anwser = true;
        cont.store(false, Ordering::SeqCst);
    }));
    }

    for t in threads {
        t.join().unwrap();
    }

    for i in jobs.iter() {
        if i.lock().unwrap().2 {

            return Ok(i.lock().unwrap().0.clone());
        }
    }

    Err("No solution.".to_owned())
}

pub fn check_if_possible(vec_in: &[u32], candidate: u32) -> bool {
    let mut found = false;
    for i in vec_in {
        if *i == candidate {
            if !found {
                found = true
            } else {
                return false;
            }
        }
    }
    true
}

pub fn print_sudoku(puzzle: &[u32]) {
    let dim = f64::sqrt(puzzle.len() as f64) as u32;
    let small_dim = f64::sqrt(dim as f64) as u32;
    let check_small = {
        small_dim * small_dim == dim
    };

    for (i, v) in puzzle.iter().enumerate() {
        print!("{} ", v);
        if check_small && (i + 1) % small_dim as usize == 0 {
            print!(" ");
        }
        if (i + 1) % dim as usize == 0 {
            println!("");
        }
        if check_small && (i + 1) % (small_dim * dim) as usize == 0 {
            println!("");
        }
    }
}