use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use rand::Rng;

use std::collections::HashSet;
use std::str::FromStr;
use std::cmp;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    println!("{}", exec("kargerMinCut.txt".to_owned()));
}

fn exec(filename: String) -> i32 {
    let (org_list, org_set) = read_file(filename).unwrap();
    let mut res: i32 = i32::MAX;
    let exec_num = org_set.len().pow(2);
    let cpu_num = num_cpus::get();
    println!("{}", cpu_num);
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for _ in 0..cpu_num {
        let moved_list = org_list.to_vec();
        let moved_set = org_set.clone();
        let tx = tx.clone();
        let exec_n = exec_num / cpu_num;
        pool.execute(move || {
            let mut min: i32 = i32::MAX;
            for i in 0..exec_n {
                let mut list = moved_list.to_vec();
                let mut set = moved_set.clone();
                while set.len() > 2 {
                    let (vertex_merged, vertex_merging) = select_random_edge(&list, &mut set);
                    merge(&mut list, vertex_merging, vertex_merged);
                }
                if i % 100 == 0 {
                    println!("{} / {}", i, exec_n);
                }

                min = cmp::min(list[get_element_from_hashset(&set) as usize].len() as i32, min);
            
            }
            tx.send(min).expect("Could not send data!");
        })
    }
    drop(tx);
    for t in rx.iter() {
        res = cmp::min(res, t);
    }
    res
}

fn merge(list: &mut Vec<Vec<i32>>, vertex_merging: i32, vertex_merged:i32 ) {
    let vertex_list_merged = list[vertex_merged as usize].to_vec();
    for v in vertex_list_merged {
        if v != vertex_merging {
            list[vertex_merging as usize].push(v);
            swap_elements(&mut list[v as usize], vertex_merging, vertex_merged);
        }
    }
    delete_elements_from_vec(&mut list[vertex_merging as usize], vertex_merged);
    list[vertex_merged as usize] = vec![];
}

fn swap_elements(list: &mut Vec<i32>, elem: i32, target: i32) {
    let res = list.iter().position(|x| *x == target);
    match res {
        Some(index) => {
            list[index] = elem
        },
        None => {}
    }
}

fn delete_elements_from_vec(list: &mut Vec<i32>, elem: i32) {
    list.retain(|&i| i != elem);
}

fn select_random_edge(list: &Vec<Vec<i32>>, set: &mut HashSet<i32>) -> (i32, i32) {
    let vertex1_index = select_and_delete_from_hashset(set);
    let vertex1_list = &list[vertex1_index as usize];
    let vertex2 = vertex1_list[generate_random_int(vertex1_list.len())];
    
    (vertex1_index as i32, vertex2)
}

fn select_and_delete_from_hashset(set: &mut HashSet<i32>) -> i32 {
    let index = rand::thread_rng().gen_range(0..set.len());
    let element = set.iter().nth(index).unwrap().clone();
    set.remove(&element);

    element
}

fn get_element_from_hashset(set: &HashSet<i32>) -> i32 {
    set.iter().nth(0).unwrap().clone()
}

fn generate_random_int(upper: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..upper) as usize
}

fn read_file(file_name: String) -> Result<(Vec<Vec<i32>>, HashSet<i32>), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec: Vec<Vec<i32>> = vec![];
    let mut set: HashSet<i32> = HashSet::new();

    for line in buffered.lines() {
        let mut neighbor_list = vec![];
        let unwraped_line = line.unwrap();
        let mut i = 0;
        for s in unwraped_line.split(" ") {
            let num: i32  = FromStr::from_str(s).unwrap();
            if i != 0 {
                neighbor_list.push(num - 1);
            } else {
                set.insert(num - 1);
            }
            i += 1;
            
        }
        vec.push(neighbor_list);
    }

    Ok((vec, set))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt".to_owned()), 2);
        assert_eq!(exec("Testcase2.txt".to_owned()), 2);
        assert_eq!(exec("Testcase3.txt".to_owned()), 2);
        assert_eq!(exec("Testcase4.txt".to_owned()), 1);
        assert_eq!(exec("Testcase5.txt".to_owned()), 1);
        assert_eq!(exec("Testcase6.txt".to_owned()), 3);
        assert_eq!(exec("Testcase7.txt".to_owned()), 2);

    }
}