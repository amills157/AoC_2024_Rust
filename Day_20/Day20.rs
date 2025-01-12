use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    
    fn distance(&self, other: &Point) -> f64 {
        let x = self.x.abs_diff(other.x).pow(2);
        let y = self.y.abs_diff(other.y).pow(2);
        (x as f64).add(y as f64).sqrt()
    }
    
    fn grid_distance(&self, other: &Point) -> u32 {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        x + y
    }

    
}


impl Add for Point{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn race_map(string_value: String) -> HashMap<Point, char>{

    let mut map: HashMap<Point, char> = HashMap::new();
    
    for (y, line) in string_value.lines().enumerate() {
    
        for (x, c) in line.chars().enumerate() {
            map.entry(Point::new(x as i32, y as i32)).or_insert(c);
        }
        
    } 
        
    return map;
}

// https://doc.rust-lang.org/std/option - Allows return in case value isn't found
fn navigator(map: &HashMap<Point, char>, marker: char) -> Option<Point> {
    
    for (point, char) in map.into_iter(){
        if *char == marker{
            return Some(*point);
        }
    }
    
    None
    
}


fn print_map(map: &HashMap<Point, char>) {
    let min_x = map.keys().map(|p| p.x).min().unwrap_or(0);
    let max_x = map.keys().map(|p| p.x).max().unwrap_or(0);
    let min_y = map.keys().map(|p| p.y).min().unwrap_or(0);
    let max_y = map.keys().map(|p| p.y).max().unwrap_or(0);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for (point, &ch) in map {
        let grid_x = point.x - min_x;
        let grid_y = point.y - min_y;
        grid[grid_y as usize][grid_x as usize] = ch;
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}


fn shortest_path(map: &HashMap<Point, char>, start: Point) -> HashMap<Point, usize> {
    let mut dist: HashMap<Point, usize> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let mut queue = VecDeque::new();
    
    let mut adj_dirs = Vec::new();
    
    adj_dirs.push(Point::new(1, 0));
    adj_dirs.push(Point::new(0, 1));
    adj_dirs.push(Point::new(-1, 0));
    adj_dirs.push(Point::new(0, -1));

    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        
        for dir in &adj_dirs {
            let neighbour = pos + *dir;
            //println!("{:?} == {:?}", neighbour, map.get(&neighbour));
            if map.get(&neighbour) == Some(&'.') {
                let alt = dist.get(&pos).unwrap_or(&usize::MAX) + 1;
                if alt < *dist.get(&neighbour).unwrap_or(&usize::MAX) {
                    dist.insert(neighbour, alt);
                    prev.insert(neighbour, pos);
                    queue.push_back(neighbour);
                }
            }
        }
        
    }

    return dist;
}

fn speed_run(map_points: Vec<Point>, dist_map: HashMap<Point, usize>, time_to_beat: usize, threshold: u32) -> usize{
    
    let mut cheats = Vec::new();
    
    for (i, point_a) in map_points.iter().enumerate() {
        for point_b in map_points.iter().skip(i) {
            if point_a.grid_distance(point_b) <= threshold {
                if let (Some(&dist_a), Some(&dist_b)) = (dist_map.get(&point_a), dist_map.get(&point_b)) {
                    let diff = dist_a.abs_diff(dist_b)  - point_a.grid_distance(point_b) as usize;
    
                    if diff >= time_to_beat {
                        cheats.push(diff);
                    }
                } else {
                    //println!("Warning: Key missing for points {:?} or {:?}", point_a, point_b);
                    continue;
                }
            }
        }
    }
    
    return cheats.len();
}



fn part_one_and_two(){
    let string_value = read_example_txt();
    
    let mut map = race_map(string_value);
    
    // for row in map{
    //     println!("{:?}", row);
    // }
    
    let start = navigator(&map, 'S').unwrap();
    let end = navigator(&map, 'E').unwrap();
    
    let dist_map = shortest_path(&map, start);
    
    //println!("{:?}", dist);
    
    // for (pos, dist) in dist_map{
    //     //println!("{:?}", pos);
    //     let wrapped_value = dist % 10;
    //     //println!("{:?}", char::from_digit(wrapped_value as u32, 10));
        
    //     map.insert(pos, char::from_digit(wrapped_value as u32, 10).unwrap() );
    // }
    
    // print_map(&map);
    
    let map_points: Vec<_> = map.keys().cloned().collect();
    
    
    let part_1_result = speed_run(map_points.clone(), dist_map.clone(), 100, 2);
    

    println!("The answer to part one is {:?}", part_1_result);
    
    let part_2_result = speed_run(map_points, dist_map, 100, 20);
    

    println!("The answer to part one is {:?}", part_2_result);
    
    //29102 - too low
    //982854 - 'wrong' (no idea if low or high)
    //987695 - ?
    //1631604 - too high
    
}


fn main() {    
    part_one_and_two();

}