use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::cmp::Reverse;

fn main() {
    println!("{}", exec("Median.txt"));
}

fn exec(file_name: &str) -> usize {
    let input = read_file(file_name).expect("not be able to find the file");
    calc(&input)
}

fn calc(input: &Vec<usize>) -> usize {
    if input.len() <= 0 {
        return 0;
    }
    let mut h_low = BinaryHeap::<usize>::new();
    let mut h_high = BinaryHeap::<Reverse<usize>>::new();

    h_low.push(input[0]);
    let mut sum = input[0];

    for i in input.iter().skip(1) {
        // push the element
        let median_low = h_low.peek().unwrap();
        if i < median_low {
            h_low.push(*i);
        } else {
            h_high.push(Reverse(*i));
        }

        // balance the heap
        let k = h_low.len() + h_high.len();
        let thres = if k % 2 == 0 { k / 2 } else { k / 2 + 1 };
        if h_low.len() > thres {
            let val = h_low.pop().unwrap();
            h_high.push(Reverse(val));
        } else if h_high.len() > thres {
            let Reverse(val) = h_high.pop().unwrap();
            h_low.push(val);
        }

        // calc the median
        // k is even
        if k % 2 == 0 {
            sum += h_low.peek().unwrap();
        // k is odd
        } else {
            if h_high.len() > h_low.len() {
                let Reverse(val) = h_high.peek().unwrap();
                sum += val;
            } else {
                let val = h_low.peek().unwrap();
                sum += val;
            }
        }
    }

    sum % 10000
}

fn read_file(file_name: &str) -> Result<Vec<usize>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec = vec![];

    for line in buffered.lines() {
        vec.push(line.unwrap().parse::<usize>().unwrap());
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(exec("Testcase1.txt"), 142);
        assert_eq!(exec("Testcase2.txt"), 9335);
    }
}
