use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Graph = HashMap<i32, Vec<i32>>;

fn main() {
    dbg!(read_file("Testcase1.txt".to_owned()).unwrap());
}

fn dfs_loop(graph: &Graph, node_num: &i32) {
    let mut t = 0;
    let mut s = 0;
    let mut finishing_times = vec![0; *node_num as usize];
    let mut leaders = vec![0; *node_num as usize];
    let mut explored: HashSet<i32> = HashSet::new();

    for i in *node_num..0 {
        if !explored.contains(&i) {
            s = i;
            dfs(&graph, &i, &mut explored, &mut t, &mut s);
        }
    }
}

fn dfs(
    graph: &Graph,
    node: &i32,
    explored: &mut HashSet<i32>,
    leaders: &mut Vec<i32>,
    finishing_times: &mut Vec<i32>,
    t: &mut i32,
    s: &mut i32,
) {
    explored.insert(*node);
}

fn read_file(file_name: String) -> Result<(Graph, Graph, i32), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut graph: Graph = HashMap::new();
    let mut reversed_graph: Graph = HashMap::new();
    let mut max = i32::MIN;

    for line in buffered.lines() {
        let unwraped_line = line.unwrap();
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let tail: i32 = FromStr::from_str(split_line[0]).unwrap();
        let head: i32 = FromStr::from_str(split_line[1]).unwrap();
        max = cmp::max(tail, max);
        if !graph.contains_key(&tail) {
            graph.insert(tail, vec![]);
        }
        let g = graph.get_mut(&tail).unwrap();
        g.push(head);

        if !reversed_graph.contains_key(&head) {
            reversed_graph.insert(head, vec![]);
        }
        let r_g = reversed_graph.get_mut(&head).unwrap();
        r_g.push(tail);
    }

    Ok((graph, reversed_graph, max))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {}
}
