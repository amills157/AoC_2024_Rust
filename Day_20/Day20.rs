use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    const fn new(x: i32, y: i32) -> Point {
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


static DIRS: [Point; 4] = [
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, 1),
    Point::new(0, -1),
];


trait Neighbors {
    type Item;

    fn neighbors(&self, x: Self::Item) -> Vec<Self::Item>;
    fn cheats(&self, x: Self::Item, duration: usize) -> Vec<Self::Item>;
}


impl Neighbors for HashMap<Point, char> {
    type Item = Point;

    fn neighbors(&self, x: Self::Item) -> Vec<Self::Item> {
        DIRS.iter()
            .map(|&d| Point::new(x.x + d.x, x.y + d.y))
            .filter(|adj| self.get(adj) != Some(&'#'))
            .collect()
    }

    fn cheats(&self, x: Self::Item, duration: usize) -> Vec<Self::Item> {
        let mut neighbors = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back((0, x));

        while let Some((dist, pos)) = queue.pop_front() {
            if dist > duration {
                continue;
            }
            if !neighbors.insert(pos) {
                continue;
            }
            for next in DIRS.iter().map(|&d| Point::new(pos.x + d.x, pos.y + d.y)) {
                queue.push_back((dist + 1, next));
            }
        }

        let mut cheats = Vec::new();

        for adj in neighbors {
            if self.get(&adj) != Some(&'#') && adj != x {
                cheats.push(adj);
            }
        }
        
        return cheats;
        
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


fn dijkstra(map: &HashMap<Point, char>, start: &Point,) -> (HashMap<Point, usize>, HashMap<Point, Point>) {
    let mut visited = HashSet::new();
    let mut prev = HashMap::new();
    let mut dist = HashMap::new();
    
    dist.insert(*start, 0);

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), *start));

    while let Some((Reverse(score), node)) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }

        for adj in map.neighbors(node) {
            let next_score = score + 1;

            if let Some(current_score) = dist.get_mut(&adj) {
                if next_score < *current_score {
                    *current_score = next_score;
                    queue.push((Reverse(next_score), adj));
                    prev.insert(adj, node);
                }
            } else {
                dist.insert(adj, next_score);
                queue.push((Reverse(next_score), adj));
                prev.insert(adj, node);
            }
        }
    }

    (dist, prev)
}

fn shortest_path(end: &Point, prev: &HashMap<Point, Point>) -> Vec<Point> {
    let mut path = vec![*end];
    let mut u = *end;

    while let Some(&v) = prev.get(&u) {
        path.push(v);
        u = v;
    }

    path.iter().copied().collect()
}


fn speed_run_v3(map: &HashMap<Point, char>, duration: usize) -> usize {
    
    let start = navigator(&map, 'S').unwrap();
    
    let end = navigator(&map, 'E').unwrap();

    let (dist_map, prev) = dijkstra(map, &end);
    
    let path = shortest_path(&start, &prev);

    let mut valid_cheats = vec![];

    for (i, elem) in path.iter().enumerate() {
        for cheat in map.cheats(*elem, duration) {
            let shortcut = (elem.x - cheat.x).abs() as usize + (elem.y - cheat.y).abs() as usize;

            if cheat == end {
                valid_cheats.push(i + shortcut);
            } else if let Some(&d) = dist_map.get(&cheat) {
                valid_cheats.push(i + shortcut + d as usize);
            }
        }
    }

    let mut count = 0;

    for x in valid_cheats {
        if x < path.len() {
            let diff = path.len() - x;
            if diff >= 100 {
                count += 1;
            }
        }
    }
    
    return count;
}



fn part_one_and_two(){
    let string_value = read_example_txt();
    
    let mut map = race_map(string_value);
    
    // for row in map{
    //     println!("{:?}", row);
    // }
    
    let part_1_result = speed_run_v3(&map, 2);
    

    println!("The answer to part one is {:?}", part_1_result);
    
    let part_2_result = speed_run_v3(&map, 20);
    

    println!("The answer to part one is {:?}", part_2_result);
    
    //29102 - too low
    //982854 - 'wrong' (no idea if low or high)
    //983054 - ?
    //1631604 - too high
    
}


fn main() {    
    part_one_and_two();

}