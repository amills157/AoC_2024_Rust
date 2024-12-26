use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
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
fn garden_blazing(garden: &mut Vec<Vec<ZachBraff>>, current_location: (usize, usize), max_y: usize, count: &mut usize, perimeter: &mut usize){
    //                                  Right    Down       Left      Up
    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    let current_value = garden[current_location.0][current_location.1].plant;
    
    garden[current_location.0][current_location.1].visited = true;
    
    *count += 1;
    
    // For each adjancent plot of the same type we remove our perimeter_addition
    let mut perimeter_addition = 4;
    
    for dir in DIRECTIONS{
        let x = (current_location.0 as i32 + dir.0) as usize;
        let y = (current_location.1 as i32 + dir.1) as usize;
        
        if (x >= 0 && x < garden.len()) && (y >= 0 && y < max_y){
            if garden[x][y].plant == current_value{
                perimeter_addition -= 1;
                //println!("({:?},{:?}) = {:?}", current_location.0, current_location.1, perimeter_addition);
            }
        }
    }
    
    *perimeter += perimeter_addition;
    
    //println!("{:?}", current_location);
    for dir in DIRECTIONS{
        let x = (current_location.0 as i32 + dir.0) as usize;
        let y = (current_location.1 as i32 + dir.1) as usize;
        
        if (x >= 0 && x < garden.len()) && (y >= 0 && y < max_y){
            if garden[x][y].plant == current_value && garden[x][y].visited == false{
                //println!("{:?}", garden[x][y]);
                garden_blazing(garden, (x, y), max_y, count, perimeter);
            }
        }
    }
}


fn part_one(){
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
    
    //println!("{:?}", garden[1][1]);
    
    let mut sum = 0;
    
    for row_idx in 0..garden.len(){
        for char_idx in 0..garden[row_idx].len(){
            let current_char = garden[row_idx][char_idx];
            if !current_char.visited{
                let mut count = 0;
                let mut perimeter = 0;
                garden_blazing(&mut garden, (row_idx, char_idx), max_y, &mut count, &mut perimeter);
                //println!("{} = {} = {} == {}", current_char.plant, count, perimeter, (count * perimeter));
                sum += count * perimeter;
            }
            
        }
    }
    
    println!("The answer to part one is {}", sum);
}

// https://www.onlinegdb.com/online_rust_compiler#
fn main() {    
    part_one();

    //part_two();
}