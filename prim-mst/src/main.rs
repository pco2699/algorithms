use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;
use std::str::FromStr;

use std::cmp::Ordering;
// use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    cost: i32,
}

type AdjList = Vec<Vec<Edge>>;

fn main() {
    println!("{}", run("Testcase2.txt"))
}

fn run(filename: &str) -> i32 {
    let (adj_list, node_num) = read_file(filename).expect("Couldn't find a file");
    prim(&adj_list, node_num)
}

fn prim(adj_list: &AdjList, node_num: usize) -> i32 {
    let mut explored = HashSet::new();
    let mut sum = 0;

    let mut current_node = 0;
    explored.insert(current_node);
    while explored.len() < node_num {
        let mut current_min = i32::MAX;
        let mut current_node_candidate = 0;
        for i in explored.iter() {
            let neighbors = &adj_list[*i];
            for neighbor in neighbors {
                if !explored.contains(&neighbor.node) {
                    let neighbor_node = neighbor.node;
                    let neighbor_cost = neighbor.cost;

                    let cost = neighbor_cost;
                    if current_min > cost {
                        current_node_candidate = neighbor_node;
                        current_min = cost;
                    }
                }
            }
        }
        sum += current_min;
        current_node = current_node_candidate;
        explored.insert(current_node);
    }

    sum
}

fn read_file(file_name: &str) -> Result<(AdjList, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut graph: AdjList = vec![];
    let mut num = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            num = usize::from_str(first_line[0]).unwrap();
            graph = vec![vec![]; num];
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let src = usize::from_str(split_line[0]).unwrap() - 1;
        let dst = usize::from_str(split_line[1]).unwrap() - 1;
        let cost = i32::from_str(split_line[2]).unwrap();
        graph[src].push(Edge {node: dst, cost});
        graph[dst].push(Edge {node: src, cost});
    }

    Ok((graph, num))
}
