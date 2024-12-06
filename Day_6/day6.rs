use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn get_next_position(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> char {
    let mut next_position = ' ';
    if let Some(row) = grid.get(row_index) {
        if let Some(value) = row.get(col_index) {
            next_position = *value;
        }
    }
    return next_position
}


fn part_one(){
    let string_value = read_example_txt();

    let mut guard_row_index = 0;
    let mut guard_column_index = 0;
    let mut direction = '^';
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (row, line) in string_value.split("\n").enumerate() {
        let mut grid_row: Vec<char> = Vec::new();

        for (column, ch) in line.char_indices() {
            if ch == '^' || ch =='v' || ch == '>' || ch == '<' {
                direction = ch;
                grid_row.push('.');
                guard_row_index = row;
                guard_column_index = column;
            } else {
                grid_row.push(ch);
            }
        }
        grid.push(grid_row);
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    'outer: for i in 0..grid.len() {
        'inner: for j in 0..grid[i].len() {

            let mut here_be_dragons = false;
            let mut recorded_positions: HashSet<Vec<usize>> = HashSet::new();
            
            // Ah ha! Wasn't recording the starting position. D'oh on my part
            recorded_positions.insert(vec![guard_row_index, guard_column_index]);

            while !here_be_dragons {
                // println!("Guard position == {} {}", guard_row_index, guard_column_index);
                // println!("Guard direction ==  {}", direction);
                match direction {
                    '^' => {
                        if guard_row_index == 0 {
                            here_be_dragons = true;
                            break;
                        }
                        guard_row_index -= 1;
                        let position = get_next_position(&grid, guard_row_index, guard_column_index);
                        //println!("Position ==  {}", position);
                        match position {
                            ' ' => here_be_dragons = true,
                            '.' => {
                                recorded_positions.insert(vec![guard_row_index, guard_column_index]);
                            },
                            '#' => {
                                guard_row_index += 1;
                                direction = '>';
                            }
                            _ => todo!()
                        }
                    },
                    '>' => {
                        guard_column_index += 1;
                        let position = get_next_position(&grid, guard_row_index, guard_column_index);
                        //println!("Position ==  {}", position);
                        match position {
                            ' ' => here_be_dragons = true,
                            '.' => {
                                recorded_positions.insert(vec![guard_row_index, guard_column_index]);
                            },
                            '#' => {
                                guard_column_index -= 1;
                                direction = 'v';
                            }
                            _ => todo!()
                        }
                    },
                    'v' => {
                        guard_row_index += 1;
                        let position = get_next_position(&grid, guard_row_index, guard_column_index);
                        //println!("Position ==  {}", position);
                        match position {
                            ' ' => here_be_dragons = true,
                            '.' => {
                                recorded_positions.insert(vec![guard_row_index, guard_column_index]);
                            },
                            '#' => {
                                guard_row_index -= 1;
                                direction = '<';
                            }
                            _ => todo!()
                        }
                    },
                    '<' => {
                        if guard_column_index == 0 {
                            here_be_dragons = true;
                            break;
                        }
                        guard_column_index -= 1;
                        let position = get_next_position(&grid, guard_row_index, guard_column_index);
                        //println!("Position ==  {}", position);
                        match position {
                            ' ' => here_be_dragons = true,
                            '.' => {
                                recorded_positions.insert(vec![guard_row_index, guard_column_index]);
                            },
                            '#' => {
                                guard_column_index += 1;
                                direction = '^';
                            }
                            _ => todo!()
                        }
                    },
                    _ => todo!()
                }
            }

            println!("The answer to part one is {}", recorded_positions.len());
            break 'outer;
        }
    }

}


// fn part_two(){
//     let string_value = read_example_txt();

//     let (mut int_vec_1, mut int_vec_2) = string_to_int_vectors(&string_value);

//     int_vec_1.sort();
//     int_vec_2.sort();

//     let mut sum = 0;

//     for (pos, e) in int_vec_1.iter().enumerate() {
//         // println!("Element at position {}: {:?}", pos, e);

//         let value_count = (int_vec_2.iter().filter(|&n| *n == *e).count()) as i32;

//         sum += (value_count * e);

//         //println!("{}", (value_count * e));
//     }

//     println!("The answer to part two is {}", sum);

// }



fn main() {    
    part_one();

    //part_two();
}