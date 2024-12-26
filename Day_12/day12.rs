use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::BTreeMap;

// https://www.imdb.com/title/tt0333766/
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ZachBraff {
    visited: bool,
    plant: char
}

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn garden_blazing(garden: &mut Vec<Vec<ZachBraff>>, current_location: (usize, usize), max_y: usize, count: &mut usize, perimeter: &mut usize, corners: &mut usize){
    //                                             Right    Down       Left      Up
    const ADJANCENT_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    //                                            DRight   URight   DLeft    ULeft
    //const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    
    let mut DIAGONAL_DIRECTIONS = HashMap::new();

    DIAGONAL_DIRECTIONS.insert((1, 1), [(1, 0), (0, 1)]);
    
    DIAGONAL_DIRECTIONS.insert((1, -1), [(1, 0), (0, -1)]);
    
    DIAGONAL_DIRECTIONS.insert((-1, 1), [(0, 1), (-1, 0)]);
    
    DIAGONAL_DIRECTIONS.insert((-1, -1), [(-1, 0), (0, -1)]);

    let current_value = garden[current_location.0][current_location.1].plant;
    
    garden[current_location.0][current_location.1].visited = true;
    
    *count += 1;
    
    // For each adjancent plot of the same type we our decrement the perimeter_modifier
    let mut perimeter_modifier = 4;
    
    for dir in ADJANCENT_DIRECTIONS{
        let x = (current_location.0 as i32 + dir.0) as usize;
        let y = (current_location.1 as i32 + dir.1) as usize;
        
        if (x >= 0 && x < garden.len()) && (y >= 0 && y < max_y){
            if garden[x][y].plant == current_value{
                perimeter_modifier -= 1;
                //println!("({:?},{:?}) = {:?}", current_location.0, current_location.1, perimeter_modifier);
            }
        }
    }
    
    
    for (key, value) in DIAGONAL_DIRECTIONS{
        let x = (current_location.0 as i32 + key.0) as usize;
        let y = (current_location.1 as i32 + key.1) as usize;
        
        let adj_x_0 = (current_location.0 as i32 + value[0].0) as usize;
        let adj_y_0 = (current_location.1 as i32 + value[0].1) as usize;
        
        let adj_x_1 = (current_location.0 as i32 + value[1].0) as usize;
        let adj_y_1 = (current_location.1 as i32 + value[1].1) as usize;

        if (x >= 0 && x < garden.len()) && (y >= 0 && y < max_y){
            if garden[x][y].plant != current_value{
                // Inside corner if two adjacent sides are the same and diagonal is not
                if garden[adj_x_0][adj_y_0].plant == current_value && garden[adj_x_1][adj_y_1].plant == current_value {
                    *corners += 1;
                } 
                
                // Outside corner if two adjacent sides are not the same
                if garden[adj_x_0][adj_y_0].plant != current_value && garden[adj_x_1][adj_y_1].plant != current_value {
                    *corners += 1;
                } 
                
            // Wasn't accounting for sneaky gaps in regions where the diagonal's are the same... 
            } else if garden[x][y].plant == current_value {
                
                // But we have a corner if two adjacent sides are not the same (a gap in the region)
                if garden[adj_x_0][adj_y_0].plant != current_value && garden[adj_x_1][adj_y_1].plant != current_value {
                    *corners += 1;
                }
            }
        } else {
            // Need to account for outside corners off the map here 

            if (adj_x_0 >= garden.len()) || (adj_y_0 >= max_y) {
            
                if (adj_x_1 >= garden.len()) || (adj_y_1 >= max_y) {
                    *corners += 1;
                } 
                
                 if (adj_x_1 >= 0 && adj_x_1 < garden.len()) && (adj_y_1 >= 0 && adj_y_1 < max_y){
                    if garden[adj_x_1][adj_y_1].plant != current_value{
                        *corners += 1;
                    } 
                 }

                
            } else if (adj_x_1 >= garden.len()) || (adj_y_1 >= max_y) {
            
                if (adj_x_0 >= garden.len()) || (adj_y_0 >= max_y) {
                    *corners += 1;
                }
                
                if (adj_x_0 >= 0 && adj_x_0 < garden.len()) && (adj_y_0 >= 0 && adj_y_0 < max_y){
                    if garden[adj_x_0][adj_y_0].plant != current_value{
                        *corners += 1;
                    } 
                 }
            }
        }
    }
    
    *perimeter += perimeter_modifier;
    
    //println!("{:?}", current_location);
    for dir in ADJANCENT_DIRECTIONS{
        let x = (current_location.0 as i32 + dir.0) as usize;
        let y = (current_location.1 as i32 + dir.1) as usize;
        
        if (x >= 0 && x < garden.len()) && (y >= 0 && y < max_y){
            if garden[x][y].plant == current_value && garden[x][y].visited == false{
                //println!("{:?}", garden[x][y]);
                garden_blazing(garden, (x, y), max_y, count, perimeter, corners);
            }
        }
    }
}


fn part_one_and_two(){
    let string_value = read_example_txt();

    let mut garden: Vec<Vec<ZachBraff>> = vec![];
    
    let mut max_y = 0;
    
    // This isn't very pretty - But I was getting issues trying to use map with a struct type
    // So had to revert to manually doing it
    for line in string_value.lines() {
        let mut temp = Vec::new();
        max_y = line.len();
        for c in line.chars(){
            temp.push(ZachBraff {
                    visited: false,
                    plant: c,
                });
        }
        garden.push(temp);
    }
    
    let mut sum_1 = 0;
    
    let mut sum_2 = 0;
    
    for row_idx in 0..garden.len(){
        for char_idx in 0..garden[row_idx].len(){
            let current_char = garden[row_idx][char_idx];
            if !current_char.visited{
                let mut count = 0;
                let mut perimeter = 0;
                let mut corners = 0;
                garden_blazing(&mut garden, (row_idx, char_idx), max_y, &mut count, &mut perimeter, &mut corners);
                //println!("{} = {} = {} == {}", current_char.plant, count, perimeter, (count * perimeter));
                sum_1 += count * perimeter;
                //println!("{} = {} = {} == {}", current_char.plant, count, corners, (count * corners));
                
                sum_2 += count * corners;
                
            }
            
        }
    }
    
    println!("The answer to part one is {}", sum_1);
    
    println!("The answer to part two is {}", sum_2);
}


fn main() {    
    part_one_and_two();

    //part_two();
}