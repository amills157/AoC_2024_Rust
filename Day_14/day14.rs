use std::io::prelude::*;
use std::fs::File;

static MAX_X: i32 = 101;
static MAX_Y: i32 = 103;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


// https://futurama.fandom.com/wiki/MomCorp
fn MomCorp(string_value: String) -> Vec<Robot>{
    let mut robots = Vec::new();
    
    for line in string_value.lines(){
        let split = line.split_once(' ').unwrap();
        
        let pos = split.0;
        let (pos_x, pos_y) = pos[2..].split_once(',').unwrap();
        
        let vol = split.1;
        
        let (vol_x, vol_y) = vol[2..].split_once(',').unwrap();
        
        //println!("x: {}, y: {}, vx: {}, vy: {}", pos_x, pos_y, vol_x, vol_y);
        
        robots.push(Robot {
                    px: pos_x.parse::<i32>().unwrap(),
                    py: pos_y.parse::<i32>().unwrap(),
                    vx: vol_x.parse::<i32>().unwrap(),
                    vy: vol_y.parse::<i32>().unwrap(),
                });
        
    }
    
    return robots;
}


fn part_one(){
    let string_value = read_example_txt();
    
    let mut robots = MomCorp(string_value);
    
    //println!("{:?}", robots);
    
    for _i in 0..100 {
        for robot in robots.iter_mut() {
            //https://www.iainmaitland.com/remainder - Wrap around for the map
            robot.px = (robot.px + robot.vx).rem_euclid(MAX_X);
            robot.py = (robot.py + robot.vy).rem_euclid(MAX_Y);
        }
    }
    
    let mut robot_positions = Vec::new();
    
    for robot in robots.iter_mut() {
        robot_positions.push((robot.px, robot.py));
    }
    
    //println!("{:?}", robot_positions);
    
    let q1 = robot_positions.iter().filter(|&&(px, py)| px < MAX_X / 2 && py < MAX_Y / 2).count();
    let q2 = robot_positions.iter().filter(|&&(px, py)| px > MAX_X / 2 && py < MAX_Y / 2).count();
    let q3 = robot_positions.iter().filter(|&&(px, py)| px < MAX_X / 2 && py > MAX_Y / 2).count();
    let q4 = robot_positions.iter().filter(|&&(px, py)| px > MAX_X / 2 && py > MAX_Y / 2).count();

    println!("The answer to part one is {}", (q1 * q2 * q3 * q4));
    
}


fn part_two(){
    let string_value = read_example_txt();

    let mut robots = MomCorp(string_value);
    
    let mut robot_positions = Vec::new();
    
    let mut sum = 0;
    
    while true{
    
        sum += 1;
        
        for robot in robots.iter_mut() {
            robot.px = (robot.px + robot.vx).rem_euclid(MAX_X);
            robot.py = (robot.py + robot.vy).rem_euclid(MAX_Y);
            robot_positions.push((robot.px, robot.py));
        }
        
        let mut temp = robot_positions.clone();
        
        temp.sort();
        temp.dedup();
        
        // if our dedup is the same length as our original...
        if temp.len() == robot_positions.len(){
            // then all the robots are in a unique position
            break;
        }
        
        robot_positions.clear();
    }
    
    //println!("{:?}", robot_positions);

    println!("The answer to part two is {}", sum);

}


fn main() {    
    part_one();

    part_two();
}