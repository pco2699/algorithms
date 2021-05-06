use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use counter::Counter;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use std::thread;

type Graph = HashMap<i32, Vec<i32>>;

const STACK_SIZE: usize = 200 * 1024 * 1024;

fn main() {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    child.join().unwrap();
}

fn run() {
    println!("{}", exec("SCC.txt"));
}

fn exec(file_name: &str) -> String {
    let (graph, r_graph, num) = read_file(file_name.to_owned()).unwrap();

    let mut order: HashMap<i32, i32> = HashMap::new();
    for i in 0..num {
        order.insert(i, i);
    }
    let (finishing_times, _) = dfs_loop(&r_graph, &num, &order);
    order.clear();
    for (i, elem) in finishing_times.iter().enumerate() {
        order.insert(*elem, i as i32);
    }
    let (_, leaders) = dfs_loop(&graph, &num, &order);
    aggregate(&leaders)
}

fn aggregate(leaders: &Vec<i32>) -> String {
    let score = leaders
        .into_iter()
        .collect::<Counter<_>>()
        .most_common_ordered();
    if score.len() > 5 {
        return score
            .iter()
            .take(5)
            .map(|&elem| elem.1.to_string())
            .collect::<Vec<_>>()
            .join(",");
    }

    let mut res = score
        .iter()
        .map(|&elem| elem.1.to_string())
        .collect::<Vec<_>>();
    let rest = vec!["0".to_owned(); 5 - score.len()];
    res.append(&mut rest.to_owned());

    res.join(",")
}

fn dfs_loop(graph: &Graph, node_num: &i32, order: &HashMap<i32, i32>) -> (Vec<i32>, Vec<i32>) {
    let mut t = 0;
    let mut finishing_times = vec![0; *node_num as usize];
    let mut leaders = vec![0; *node_num as usize];
    let mut explored: HashSet<i32> = HashSet::new();

    for i in (0..*node_num).rev() {
        let node = order.get(&i).unwrap();
        if !explored.contains(&node) {
            let s = *node;
            dfs(
                &graph,
                &node,
                &mut explored,
                &mut leaders,
                &mut finishing_times,
                &mut t,
                &s,
            );
        }
    }

    (finishing_times, leaders)
}

fn dfs(
    graph: &Graph,
    node: &i32,
    explored: &mut HashSet<i32>,
    leaders: &mut Vec<i32>,
    finishing_times: &mut Vec<i32>,
    t: &mut i32,
    s: &i32,
) {
    explored.insert(*node);
    leaders[*node as usize] = *s;
    if let Some(neighbors) = graph.get(&node) {
        for neighbor in neighbors {
            if !explored.contains(neighbor) {
                dfs(graph, neighbor, explored, leaders, finishing_times, t, s);
            }
        }
    }
    finishing_times[*node as usize] = *t;
    *t += 1;
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
        let tail_1indexed: i32 = FromStr::from_str(split_line[0]).unwrap();
        let tail = tail_1indexed - 1;

        let head_1indexed: i32 = FromStr::from_str(split_line[1]).unwrap();
        let head = head_1indexed - 1;

        max = cmp::max(tail, max);
        let g = graph.entry(tail).or_insert(vec![]);
        g.push(head);

        let r_g = reversed_graph.entry(head).or_insert(vec![]);
        r_g.push(tail);
    }

    Ok((graph, reversed_graph, max + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt"), "3,3,3,0,0");
        assert_eq!(exec("Testcase2.txt"), "3,3,2,0,0");
        assert_eq!(exec("Testcase3.txt"), "3,3,1,1,0");
        assert_eq!(exec("Testcase4.txt"), "7,1,0,0,0");
        assert_eq!(exec("Testcase5.txt"), "6,3,2,1,0");
        assert_eq!(exec("Testcase6.txt"), "3,1,1,0,0");
    }
}
