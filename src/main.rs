use std::{thread, time};

fn main() {
    const WIDTH:usize = 9; 
    const HEIGHT:usize = 9; 
    let mut board:Vec<Vec<u8>> = Vec::new();
    //initialise the board
    for _ in 0..HEIGHT{
       board.push([1;WIDTH].to_vec()); 
    }
    pretty_print_arr(&board);
    loop{
        thread::sleep(time::Duration::from_secs(1));

        board = get_next_board(&board);
        
        
        pretty_print_arr(&board);
    }
}

fn get_alive_neighboours(board:&Vec<Vec<u8>> ,index:(usize,usize)) -> u8{
    //this function should have not taken this long to write
    let mut alive_count:u8 = 0;
    for i in 0..3{
        let tmp1 = index.0 as isize-(i-1);
        if tmp1 == -1 || tmp1 as usize == board.len(){
            continue;
        }
        for j in 0..3{
            let tmp2 = index.1 as isize-(j-1);
            if tmp2 == -1 || tmp2 as usize == board.len(){
                continue;
            }
            if board[tmp1 as usize][tmp2 as usize] == 1{
                alive_count+=1;
            }
        }
    } 
    if board[index.0][index.1] == 1{
        alive_count-=1;
    }
    alive_count 
}

fn get_next_board(board:&Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let mut next_neigbours:Vec<Vec<u8>> = Vec::new();
    let mut next_board:Vec<Vec<u8>> = Vec::new();
    //println!("{:?}", board);
    for (i, val1) in board.iter().enumerate(){
        let mut line_val:Vec<u8> = Vec::new();
        for (j, _val2) in val1.iter().enumerate(){
            line_val.push(get_alive_neighboours(board, (i,j))); 
//            println!("{:?},{i},{j}", board);
        }
        next_neigbours.push(line_val);
    }
    //println!("{:?}", next_neigbours);
    for (i, val1) in next_neigbours.iter().enumerate(){
        let mut line_val:Vec<u8> = Vec::new();  
        for (j, val2) in val1.iter().enumerate(){
            if board[i][j] == 1{
                if val2 < &2 {
                    line_val.push(0);
                }else if val2 == &2 || val2 == &3{
                    line_val.push(1);
                }else if val2 > &3 && val2 < &10{//greater than 3
                    line_val.push(0);
                }
            }else if board[i][j] == 0{
                if val2 == &3{
                    line_val.push(1);
                }else{
                    line_val.push(0);
                }
            }
        }
        next_board.push(line_val);
    } 
    next_board
}

fn pretty_print_arr<T: std::fmt::Display>(board:&Vec<Vec<T>>){
    for i in board{
        for j in i{
           print!("{} ", j); 
        }
        println!();
    }
    println!();
}
