#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn solve_test_positive(){
        let mut vec = vec![0u32;4];
        assert!(solve(&mut vec));
        vec = vec![0u32;9];
        assert!(solve(&mut vec));
        vec = vec![0u32;81];
        assert!(solve(&mut vec));
        vec = vec![0u32,1,0,0];
        assert!(solve(&mut vec));
        vec = vec![1u32,2,0,0];
        assert!(solve(&mut vec));
        vec = vec![1u32,2,0,1];
        assert!(solve(&mut vec));
        vec = vec![0u32,1,0,2,0,1,0,2,0];
        assert!(solve(&mut vec));
        vec = vec![0u32,0, 0, 0,
                    0, 1, 2, 0,
                    0, 3, 4, 0,
                    0, 0, 0, 0];
        assert!(solve(&mut vec));
    }

    #[test]
    fn solve_test_negative(){
        let mut vec = vec![0u32,1,1,0,0,0,0,0,0];
        assert!(!solve(&mut vec));
        vec = vec![0u32,1,0,0,0,0,0,1,0];
        assert!(!solve(&mut vec));
        vec = vec![0u32,0,0,2,0,2,0,0,0];
        assert!(!solve(&mut vec));
        vec = vec![0u32;8];
        assert!(!solve(&mut vec));
        vec = vec![0u32,0, 0, 0,
                    0, 0, 0, 0,
                    0, 1, 0, 0,
                    1, 0, 0, 0];
        assert!(!solve(&mut vec));
        vec = vec![0u32,0, 0, 0,
                    2, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 3, 4];
        assert!(!solve(&mut vec));
    }

    #[test]
    fn possible_test_positive(){
        assert!(check_if_possible(& vec![1,2,0],3));
        assert!(check_if_possible(& vec![0],1));
        assert!(check_if_possible(& vec![1,0],3));
    }

    #[test]
    fn possible_test_negative(){    
        assert!(!check_if_possible(& vec![1,0],1));
        assert!(!check_if_possible(& vec![1,2,0],1));
    }

    #[test]
    #[ignore]
    fn hard_to_bruteforce(){
    let mut test = vec![0,0,0,0,0,0,0,0,0,
                        0,0,0,0,0,3,0,8,5,
                        0,0,1,0,2,0,0,0,0,
                        0,0,0,5,0,7,0,0,0,
                        0,0,4,0,0,0,1,0,0,
                        0,9,0,0,0,0,0,0,0,
                        5,0,0,0,0,0,0,7,3,
                        0,0,2,0,1,0,0,0,0,
                        0,0,0,0,4,0,0,0,9];
        assert!(solve(&mut test));
    }


}

pub fn solve(puzzle: &mut Vec<u32>)-> bool {

    let max_value = f64::sqrt(puzzle.len() as f64) as u32;
    if max_value * max_value != puzzle.len() as u32{
        return false;
    }
    
    let inner_square = f64::sqrt(max_value as f64) as u32;
    let check_inner_square =  {inner_square*inner_square == max_value};
    
    let mut fixed_map: Vec<bool> = Vec::with_capacity(puzzle.len());
    
    
    for element in puzzle.iter(){
        if *element == 0{
            fixed_map.push(false);
        }
        else{
            fixed_map.push(true);
        };
        
    }
    
    let mut index:u32 = 0;
    let mut forward = true;
    
    while index < puzzle.len() as u32{
        if fixed_map[index as usize] == false{
            if puzzle[index as usize]+1 > max_value{
                if index == 0{
                    return false;
                }
                puzzle[index as usize] = 0;
                forward = false;
            }
            else{
            
                let row = index/max_value;
                let col = index%max_value;
                let mut squares: Vec<u32> = Vec::new();

                if check_inner_square{
                    let temp = index - index%inner_square;
                    let square = temp - ((temp/max_value)%inner_square) * max_value;
                    let square_row = square/max_value;
                    let square_col = square%max_value;                    
                    
                    squares = puzzle.chunks(inner_square as usize)
                              .skip((square_row*inner_square+square_col/inner_square)as usize)
                              .enumerate()
                              .filter(|&(i,_)|i%inner_square as usize==0)
                              .map(|(_,v)| v)
                              .take(inner_square as usize)
                              .flat_map(|x| x)
                              .cloned()
                              .collect();
                }
                
                let cols = puzzle.iter().enumerate().filter(|&(i,_)| i as u32%max_value == col).map(|(_,v)| v).cloned().collect::<Vec<u32>>();
                let candidate = puzzle[index as usize]+1;
                if !(check_if_possible(&puzzle[(row*max_value) as usize ..(row*max_value+max_value) as usize],candidate) &&
                    check_if_possible(&cols[..],candidate) &&
                    (!check_inner_square || check_if_possible(&squares[..],candidate))
                    )
                {
                    puzzle[index as usize] += 1;
                    continue;
                }
                puzzle[index as usize] += 1;
                forward = true;
            }
        }
        if forward == true{
            index += 1;
        }
        else{
            index -= 1;
        }
    }
    
    true
}

pub fn check_if_possible(vec_in: &[u32], candidate: u32) -> bool{
    for i in vec_in{
        if *i == candidate{
            return false;
        }
    }
    true
}

pub fn print_sudoku(puzzle: &Vec<u32>){
    let dim = f64::sqrt(puzzle.len() as f64) as u32;
    let small_dim = f64::sqrt(dim as f64) as u32;
    let check_small = {small_dim*small_dim == dim};
    
    for (i,v) in puzzle.iter().enumerate(){
        print!("{} ",v);
        if check_small && (i+1)%small_dim as usize == 0 {print!(" ");}
        if (i+1)%dim as usize == 0 {println!("");}
        if check_small && (i+1)%(small_dim*dim) as usize == 0 {println!("");}
    }
}