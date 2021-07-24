use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;
use std::str::FromStr;

struct Point {
    x :f64,
    y: f64
}

type Points = Vec<Point>;

fn main() {
    println!("{}", solve("nn.txt"));
}

fn solve(file_name: &str) -> f64 {
    let (points, n) = read_file(file_name).expect("cannnot read the file");
    let mut current_index = 0;
    let mut visited = HashSet::new();
    let mut travelled_distance = 0f64;
    let mut last_index = 0;

    while visited.len() < n {
        visited.insert(current_index);
        let p1 = &points[current_index];

        let mut shortest_dist = f64::MAX;
        let mut shortest_index = 0;
        println!("{}", current_index);        
        for i in 0..n {
            if visited.contains(&i) {
                continue;
            }

            let p2 = &points[i];
            let distance = calc_squared_distance(p1, p2);
            let previous_shortest_dist = shortest_dist.clone();
            shortest_dist = distance.min(shortest_dist);
            if shortest_dist != previous_shortest_dist {
                shortest_index = i;
            }
        }
        let p_shortest = &points[shortest_index];
        travelled_distance += calc_distance(p1, p_shortest);

        current_index = shortest_index;
        last_index = shortest_index;
    }

    (travelled_distance + calc_distance(&points[0], &points[last_index])).floor()
}

fn calc_distance(p1: &Point, p2: &Point) -> f64 {
    calc_squared_distance(p1, p2).sqrt()
}

fn calc_squared_distance(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}

fn read_file(file_name: &str) -> Result<(Points, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut points: Points = vec![];
    let mut num = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            num = usize::from_str(first_line[0]).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let x = f64::from_str(split_line[1]).unwrap();
        let y = f64::from_str(split_line[2]).unwrap();
        points.push(Point{x, y})
    }

    Ok((points, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(solve("Testcase1.txt").round(), 15.0);
        assert_eq!(solve("Testcase2.txt").round(), 23.0);
    }
}