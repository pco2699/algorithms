use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashMap;
use std::str::FromStr;

type Graph = HashMap<usize, HashMap<usize, i64>>;

fn main() {
    println!("{}", run("g1.txt"));
    println!("{}", run("g2.txt"));
    println!("{}", run("g3.txt"));
}

fn run(filename: &str) -> String {
    let (graph, node_num) = read_file(filename).expect("Couldn't find a file");
    match wf(&graph, node_num) {
        Some(res) => res.to_string(),
        None => "NULL".to_owned() 
    }
}

fn wf(graph: &Graph, node_num: usize) -> Option<i64> {
    let mut dp = initialize(&graph, node_num);

    for k in 1..=node_num {
        for i in 0..node_num {
            for j in 0..node_num {
                dp[k][i][j] = std::cmp::min(dp[k-1][i][j],
                        if dp[k-1][i][k-1] == std::i64::MAX || dp[k-1][k-1][j] == std::i64::MAX { std::i64::MAX }
                        else { dp[k-1][i][k-1] + dp[k-1][k-1][j] }
                    )
            }
        }
    }

    if check_negative_cycles(&dp[node_num]) {
        None
    } else {
        Some(find_shortest(&dp[node_num], node_num))
    }
}

fn find_shortest(result: &Vec<Vec<i64>>, node_num: usize) -> i64 {
    let mut res = std::i64::MAX;
    for i in 0..node_num {
        for j in 0..node_num {
            res = std::cmp::min(result[i][j], res)
        }
    }
    res
}

fn check_negative_cycles(result: &Vec<Vec<i64>>) -> bool {
    for (i, r) in result.iter().enumerate() {
        if r[i] < 0 {
            return true;
        }
    }
    false
}

fn initialize(graph: &Graph, node_num: usize) -> Vec<Vec<Vec<i64>>> {
    let mut dp = vec![vec![vec![std::i64::MAX; node_num]; node_num]; node_num + 1];
    for i in 0..node_num {
        for j in 0..node_num {
            dp[0][i][j] = if i == j {
                    0 
                } else {
                    match get_cost(&graph, i, j) {
                        Some(cost) => cost,
                        None    => std::i64::MAX
                    }
                } 
        }
    }
    dp
}

fn get_cost(graph: &Graph, i: usize, j: usize) -> Option<i64> {
    if !path_exists(&graph, i, j) {
        None
    } else {
        let g = graph.get(&i).unwrap();
        Some(*g.get(&j).unwrap())
    }
}

fn path_exists(graph: &Graph, i: usize, j: usize) -> bool {
    if graph.contains_key(&i) {
        let g = graph.get(&i).unwrap();
        g.contains_key(&j)
    } else {
        false
    }
}

fn read_file(file_name: &str) -> Result<(Graph, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut graph: Graph = HashMap::new();
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
        let cost = i64::from_str(split_line[2]).unwrap();
        let g = graph.entry(src).or_insert(HashMap::new());
        g.insert(dst, cost);
    }

    Ok((graph, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(run("Testcase1.txt",), "NULL");
        assert_eq!(run("Testcase2.txt",), "-2");
        assert_eq!(run("Testcase3.txt",), "-2");
        assert_eq!(run("Testcase4.txt",), "NULL");
    }
}