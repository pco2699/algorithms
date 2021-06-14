use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

mod union_find_list;
use union_find_list::UnionFindList;

type NodeList = Vec<usize>;

fn main() {
    println!("{}", exec("clustering_big.txt"));
}

fn exec(file_name: &str) -> usize {
    let (node_list, num, num_of_bits) = read_file(file_name).unwrap();
    let map = create_map(&node_list);
    let bitmasks = create_bitmasks(num_of_bits);
    let mut union_find = UnionFindList::new(num);

    for k in map.keys() {
        for (_, bitmask) in bitmasks.iter().enumerate() {
            let res = bitmask ^ k;
            if map.contains_key(&res) {
                let node_ids1 = map.get(&res).unwrap();
                let node_ids2 = map.get(k).unwrap();
                for node_id1 in node_ids1 {
                    for node_id2 in node_ids2 {
                        union_find.merge(*node_id1, *node_id2);
                    }
                }
            }
        }
    }
    union_find.len()
}

fn create_map(list: &NodeList) -> HashMap<usize, Vec<usize>> {
    let mut number_map = HashMap::new();
    for (index, node) in list.iter().enumerate() {
        let vec = number_map.entry(*node).or_insert(vec![]);
        vec.push(index)
    }
    number_map
}

fn create_bitmasks(num_of_bits: usize) -> Vec<usize> {
    let mut masks = vec![];
    // create 0bit masks
    masks.push(0);
    // create 1bit mask
    for i in 0..num_of_bits {
        masks.push(1 << i);
    }
    // create 2bit mask
    let comb = (0..num_of_bits).combinations();
    for (n1, n2) in comb {
        masks.push(1 << n1 ^ 1 << n2)
    }

    masks
}

fn read_file(file_name: &str) -> Result<(NodeList, usize, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut node_list: NodeList = vec![];
    let mut num = 0;
    let mut num_of_bits = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            num = usize::from_str(first_line[0]).unwrap();
            num_of_bits = usize::from_str(first_line[1]).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let mut sum = 0;
        for i in 0..num_of_bits {
            let cur = usize::from_str(split_line[i]).unwrap();
            sum += cur * 2usize.pow(i as u32);
        }
        node_list.push(sum);
    }
    Ok((node_list, num, num_of_bits))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt"), 2);
        assert_eq!(exec("Testcase2.txt"), 6);
        assert_eq!(exec("Testcase3.txt"), 15);
        assert_eq!(exec("Testcase4.txt"), 15);
        assert_eq!(exec("Testcase5.txt"), 14);
        assert_eq!(exec("Testcase6.txt"), 3);
        assert_eq!(exec("Testcase7.txt"), 716);

    }
}