mod union_find_list;
use union_find_list::UnionFindList;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use std::cmp::Ordering;
use std::cmp::Ord;

#[derive(Debug, Clone, Eq)]
struct Edge {
    node1: usize,
    node2: usize,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

type EdgeList = Vec<Edge>;


fn main() {
    println!("{}", exec("clustering1.txt", 4));
}

fn exec(file_name: &str, k: usize) -> usize {
    let (mut edge_list, node_num) = read_file(file_name).expect("Couldn't find a file");
    clustering(&mut edge_list, node_num, k)
}

fn clustering(edge_list: &mut EdgeList, node_num: usize, k: usize) -> usize {
    edge_list.sort();
    edge_list.reverse();
    let mut union_find_list = UnionFindList::new(node_num);
    while union_find_list.len() > k {
        let shortest_edge = edge_list.pop().unwrap();
        if !union_find_list.is_same(shortest_edge.node1, shortest_edge.node2) {
            union_find_list.merge(shortest_edge.node1, shortest_edge.node2);
        }
    }
    for edge in edge_list.iter_mut() {
        if union_find_list.is_same(edge.node1, edge.node2) {
            edge.cost = usize::MAX;
        }
    }
    edge_list.iter().min().unwrap().cost
}

fn read_file(file_name: &str) -> Result<(EdgeList, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut edge_list: EdgeList = vec![];
    let mut num = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            num = usize::from_str(first_line[0]).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let src = usize::from_str(split_line[0]).unwrap() - 1;
        let dst = usize::from_str(split_line[1]).unwrap() - 1;
        let cost = usize::from_str(split_line[2]).unwrap();
        edge_list.push(
            Edge {
                node1: src,
                node2: dst,
                cost
            }
        )
    }

    Ok((edge_list, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt", 4), 1);
        assert_eq!(exec("Testcase1.txt", 3), 2);
        assert_eq!(exec("Testcase1.txt", 2), 5);
        assert_eq!(exec("Testcase2.txt", 4), 1);
        assert_eq!(exec("Testcase2.txt", 3), 4);
        assert_eq!(exec("Testcase2.txt", 2), 8);
    }
}