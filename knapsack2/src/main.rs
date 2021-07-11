use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use std::collections::HashMap;

use std::cmp;
use std::thread;

struct Item {
    weight: u32,
    value: u32
}

type ItemList = Vec<Item>;
type Key = (usize, usize);
type Memo = HashMap<Key, u32>;

const STACK_SIZE: usize = 200 * 1024 * 1024;

fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    child.join().unwrap();
}

fn run() {
    println!("{}", solve("knapsack_big.txt"));
}

fn solve(file_name: &str) -> u32 {
    let (item_list, knapsack_size) = read_file(file_name).expect("file not found");
    let mut memo: Memo = HashMap::new();
    
    let w = knapsack_size;
    let n = item_list.len();

    dp(w, n, &item_list, &mut memo)
}

fn dp(x: usize, i: usize, item_list: &ItemList, memo: &mut Memo) -> u32 {
    if memo.contains_key(&(x, i)){
        return *memo.get(&(x, i)).unwrap();
    }
    if i == 0 {
        return 0;
    }

    let wi = item_list[i-1].weight;
    let vi = item_list[i-1].value;
    let res = cmp::max(dp(x, i-1, &item_list, memo), if x >= wi as usize { dp(x - wi as usize, i-1, &item_list, memo) + vi } else { 0 });
    memo.insert((x, i), res);
    
    res
}

fn read_file(file_name: &str) -> Result<(ItemList, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut item_list: ItemList = vec![];
    let mut knapsack_size: usize = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            knapsack_size = usize::from_str(first_line[0]).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let value = u32::from_str(split_line[0]).unwrap();
        let weight = u32::from_str(split_line[1]).unwrap();
        item_list.push(Item {
            weight,
            value,
        });
    }

    Ok((item_list, knapsack_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(solve("Testcase1.txt"), 150);
        assert_eq!(solve("Testcase2.txt"), 8);
    }
}