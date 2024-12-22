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

fn trail_blazing(map: &Vec<Vec<i32>>, current_location: (usize, usize), max_y: usize, nine_coords: &mut HashSet<(usize, usize)>){
    //                                  Right    Down       Left      Up
    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    let current_value = map[current_location.0][current_location.1];
    
    if current_value == 9{
        nine_coords.insert((current_location.0, current_location.1));
        return;
    }
    
    //println!("{:?}", current_location);
    for dir in DIRECTIONS{
        let x = (current_location.0 as i32 + dir.0) as usize;
        let y = (current_location.1 as i32 + dir.1) as usize;
        
        if (x >= 0 && x < map.len()) && (y >= 0 && y < max_y){
            if map[x][y] == current_value +1{
                //println!("{:?}", map[x][y])
                trail_blazing(&map, (x, y), max_y, nine_coords);
            }
        }
    }
}


fn part_one(){
    let string_value = read_example_txt();
    
    let mut map: Vec<Vec<i32>> = vec![];
    
    let mut max_y = 0;
    
    for line in string_value.lines() {
        let row: Vec<i32> = line.chars().map(|v| v.to_digit(10).unwrap() as i32).collect();
        map.push(row);
    }
    
    //println!("{:?}", map);
        
    let mut trailheads = Vec::new();

    for x in 0..map.len(){
        let row = &map[x];
        println!("{:?}", row);
        for y in 0..row.len(){
            max_y = row.len();
            if row[y] == 0{
                //println!("{} {}", x, y);
                trailheads.push((x, y));
            }
            
        }
    }
    
    //println!("{:?}", trailheads);
    
    let mut sum = 0;
    
    for trailhead in trailheads{
        let mut nine_coords: HashSet<(usize, usize)> = HashSet::new();
        trail_blazing(&map, trailhead, max_y, &mut nine_coords);
        //println!("{:?}", nine_coords);
        sum += nine_coords.len();
    }

    println!("The answer to part one is {}", sum);
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

// https://www.onlinegdb.com/online_rust_compiler#
fn main() {    
    part_one();

    //part_two();
}