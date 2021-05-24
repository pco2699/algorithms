use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;

fn main() {
    println!("{}", exec("Twosum.txt", -10000, 10000));
}

fn exec(file_name: &str, min: i64, max: i64) -> u64 {
    let input = read_file(file_name).expect("not be able to find the file");
    calc(&input, min, max)
}


fn calc(input: &Vec<i64>, min: i64, max: i64) -> u64 {
    let mut map = HashSet::new();
    for n in input {
        map.insert(n);
    }
    let mut count = 0;
    for t in min..=max {
        for n in input {
            if map.contains(&(t-n)) && t-n != *n {
                count += 1;
                break;
            }
        }
    }

    count
}

fn read_file(file_name: &str) -> Result<Vec<i64>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec = vec![];

    for line in buffered.lines() {
        vec.push(line.unwrap().parse::<i64>().unwrap());
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(exec("Testcase1.txt", 3, 10), 8);
        assert_eq!(exec("Testcase2.txt", -10000, 10000), 11);
        assert_eq!(exec("Testcase3.txt", -10000, 10000), 6);
        assert_eq!(exec("Testcase4.txt", 0, 4), 2);
    }
}