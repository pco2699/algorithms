use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

use std::cmp::Ordering;

fn main() {
    println!("{}", exec("jobs.txt", "subtract"));
    println!("{}", exec("jobs.txt", "ratio"));
}

fn exec(file_name: &str, method: &str) -> u64 {
    let mut input = if method == "subtract" {
        calc_score(read_file(file_name).unwrap(), subtract) 
    } else {
        calc_score(read_file(file_name).unwrap(), ratio)
    };

    input.sort();
    process(&input)
}

fn process(vec: &Vec<Job>) -> u64 {
    let mut sum = 0u64;
    let mut comp_time = 0u64;
    for job in vec {
        comp_time += job.length as u64;
        sum += comp_time * job.weight as u64;
    }

    sum
}

#[derive(Debug)]
struct Job {
    weight: u32,
    length: u32,
    score: f64
}


impl Eq for Job {}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.length == other.length
    }
}


impl Ord for Job  {
    fn cmp(&self, other: &Self) -> Ordering {
        let order_r = self.score.partial_cmp(&other.score);
        if let Some(order) = order_r {
            if order == Ordering::Equal {
                return self.weight.cmp(&other.weight);
            } else {
                return order;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Job  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(&self))
    }
}


fn calc_score(vec: Vec<Job>, predicate: fn(&Job) -> f64) -> Vec<Job> {
    vec.iter().map(|job| {
        Job {
            weight: job.weight,
            length: job.length,
            score: predicate(job)
        }
    }).collect()
}

fn subtract(job: &Job) -> f64 {
    job.weight as f64 - job.length as f64
}

fn ratio(job: &Job) -> f64 {
    job.weight as f64 / job.length as f64
}

fn read_file(file_name: &str) -> Result<Vec<Job>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec = vec![];

    for line in buffered.lines().skip(1) {
        let unwraped_line = line?;
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let weight = FromStr::from_str(split_line[0]).unwrap();
        let length = FromStr::from_str(split_line[1]).unwrap();
        vec.push(Job {length, weight, score: 0.0});
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt", "subtract"), 31);
        assert_eq!(exec("Testcase1.txt", "ratio"), 29);
        assert_eq!(exec("Testcase5.txt", "subtract"), 68615);
        assert_eq!(exec("Testcase5.txt", "ratio"), 67247);
    }
}