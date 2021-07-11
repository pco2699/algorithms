use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;

use std::cmp;

struct Item {
    weight: u32,
    value: u32
}

type ItemList = Vec<Item>;

fn main() {
    println!("{}", solve("knapsack1.txt"));
}

fn solve(file_name: &str) -> u32 {
    let (item_list, knapsack_size) = read_file(file_name).expect("file not found");
    let mut dp_table = vec![vec![0u32; item_list.len()+1]; knapsack_size+1];
    
    let w = knapsack_size;
    let n = item_list.len();

    for i in 1..=n {
        let wi = item_list[i-1].weight as usize;
        let vi = item_list[i-1].value as usize;
        for x in 0..=w {
            dp_table[x][i] = cmp::max(dp_table[x][i-1], if x >= wi { dp_table[x-wi][i-1] + vi as u32} else { 0 })
        }
    }

    dp_table[w][n]
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