use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use itertools::Itertools;

struct Point {
    x :f64,
    y: f64
}

type Points = Vec<Point>;

fn main() {
    println!("{}", solve("tsp.txt"));
}

fn solve(file_name: &str) -> f64 {
    let (points, n) = read_file(file_name).expect("cannnot read the file");
    let mut a = initialize(&n);    

    for m in 1..n {
        let (bitmasks, subsets, len) = create_bitmasks_and_subsets(&n, &m);
        for i in 0..len {
            let bitmask = &bitmasks[i];
            let subset = &subsets[i];
            for j in subset {
                if *j != 0 {
                    let mut min_val = f64::MAX;
                    for k in subset {
                        if k != j {
                            min_val = min_val.min(a[subtract_bitmask(bitmask, j)][*k] + calc_distance(&points[*k], &points[*j]));
                        }
                    }
                    a[*bitmask][*j] = min_val;
                }
            }

        }
    }
    finalize(&a, &points, &n)
}

fn initialize(n: &usize) -> Vec<Vec<f64>> {
    let two_to_n = 2usize.pow(*n as u32);
    let mut a = vec![vec![f64::MAX; *n]; two_to_n];
    a[1][0] = 0.0;
    a
}

fn finalize(a: &Vec<Vec<f64>>, points: &Points, n: &usize) -> f64 {
    let mut min_val = f64::MAX;
    for j in 1..*n {
        min_val = min_val.min(a[2usize.pow(*n as u32) - 1][j] + calc_distance(&points[j], &points[0]))
    }
    min_val
}

fn calc_distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn subtract_bitmask(bitmask: &usize, j: &usize) -> usize {
    bitmask ^ (1 << j)
}

fn create_bitmasks_and_subsets(n: &usize, m: &usize) -> (Vec<usize>, Vec<Vec<usize>>, usize) {
    let subsets = (1..*n).combinations(*m);
    let mut v = vec![];
    let mut subset_v = vec![];
    for mut subset in subsets {
        subset.push(0);
        subset_v.push(subset.clone());
        let mut bitmask = 0usize;
        for val in subset {
            bitmask = 1 << val | bitmask;
        }
        v.push(bitmask);
    }
    let length = v.len();
    (v, subset_v, length)
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
        let x = f64::from_str(split_line[0]).unwrap();
        let y = f64::from_str(split_line[1]).unwrap();
        points.push(Point{x, y})
    }

    Ok((points, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(solve("Testcase1.txt").round(), 10.0);
        assert_eq!(solve("Testcase2.txt").round(), 12.0);
        assert_eq!(solve("Testcase3.txt").round(), 14.0);
        assert_eq!(solve("Testcase4.txt").round(), 2.0);
        assert_eq!(solve("Testcase5.txt").round(), 4.0);
        assert_eq!(solve("Testcase6.txt").round(), 4.0);
    }

    #[test]
    fn test_subtract_bitmask() {
        assert_eq!(subtract_bitmask(&1, &0), 0);
        assert_eq!(subtract_bitmask(&3, &0), 2);
    }

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(&Point{x: 10.0, y: 20.0}, &Point{x: 50.0, y: -10.0}), 50.0);
        assert_eq!(calc_distance(&Point{x: 0.0, y: 0.0}, &Point{x: 0.0, y: 0.0}), 0.0);
    }

}