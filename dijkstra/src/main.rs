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

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

type AdjList = Vec<Vec<Edge>>;

fn main() {
    println!("{}", run("dijkstraData.txt"))
}

fn aggregate(result: &Vec<usize>) -> String {
    let selected_order: HashSet<usize> = [7, 37, 59, 82, 99, 115, 133, 165, 188, 197]
        .iter()
        .map(|v| v - 1)
        .collect();

    result
        .iter()
        .enumerate()
        .filter(|(i, _)| selected_order.contains(&i))
        .map(|(_, v)| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn run(filename: &str) -> String {
    let adj_list = read_file(filename).expect("Couldn't find a file");
    aggregate(&dijkstra(&adj_list))
}

fn dijkstra(adj_list: &AdjList) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj_list.len()];
    let node_num = adj_list.len();
    let mut explored = HashSet::new();

    let mut current_node = 0;
    dist[0] = current_node;
    explored.insert(current_node);
    while explored.len() < node_num {
        let mut current_min = usize::MAX;
        let mut current_node_candidate = 0;
        for i in explored.iter() {
            let neighbors = &adj_list[*i];
            for neighbor in neighbors {
                if !explored.contains(&neighbor.node) {
                    let neighbor_node = neighbor.node;
                    let neighbor_cost = neighbor.cost;

                    let cost = dist[*i] + neighbor_cost;
                    if current_min > cost {
                        current_node_candidate = neighbor_node;
                        current_min = cost;
                    }
                }
            }
        }
        current_node = current_node_candidate;
        dist[current_node] = current_min;
        explored.insert(current_node);
    }

    dist
}

fn read_file(file_name: &str) -> Result<AdjList, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut graph: AdjList = vec![];

    for line in buffered.lines() {
        let unwraped_line = line.unwrap();
        let split_line = unwraped_line.split("\t").collect::<Vec<&str>>();
        graph.push(
            split_line
                .iter()
                .skip(1)
                .map(|val| val.split(",").collect::<Vec<_>>())
                .filter(|val| !val[0].is_empty())
                .map(|val| Edge {
                    node: usize::from_str(val[0]).expect("failed to parse number") - 1,
                    cost: usize::from_str(val[1]).expect("failed to parse number"),
                })
                .collect::<Vec<Edge>>(),
        );
    }

    Ok(graph)
}
