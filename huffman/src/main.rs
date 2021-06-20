mod symbol;
use symbol::Symbol;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use std::collections::VecDeque;

use std::cmp;

type SymbolList = Vec<Symbol>;


fn main() {
    let (max, min) = run("huffman.txt");
    println!("{} {}", max, min);
}

fn run(filename: &str) -> (usize, usize) {
    let mut weight_list = read_file(filename).expect("cannot read the file");
    weight_list.sort();

    let mut queue1: VecDeque<Symbol> = weight_list.into_iter().collect();
    let mut queue2: VecDeque<Symbol> = VecDeque::new();
    let mut tmp: Vec<Symbol> = vec![];

    while queue1.len() > 0 || queue2.len() > 1 {
        let queue1_front = queue1.front();
        let queue2_front = queue2.front();

        if queue1_front == None {
            tmp.push(queue2.pop_front().unwrap());
        } else if queue2_front == None {
            tmp.push(queue1.pop_front().unwrap());
        } else {
            let queue1_front_u = queue1_front.unwrap();
            let queue2_front_u = queue2_front.unwrap();
            if queue1_front_u >= queue2_front_u {
                tmp.push(queue2.pop_front().unwrap());
            } else {
                tmp.push(queue1.pop_front().unwrap());
            }
            
        }
        if tmp.len() == 2 {
            let elem1 = &tmp[0];
            let elem2 = &tmp[1];
            queue2.push_back(merge(&elem1, &elem2));
            tmp.clear();
        }
    }
    if tmp.len() > 0 {
        let elem = &tmp[0];
        let elem2 = queue2.pop_front().unwrap();
        queue2.push_back(merge(&elem, &elem2));
    }

    let res = queue2.pop_front().unwrap();
    
    (res.max_merged_count, res.min_merged_count)
}

fn merge(elem1: &Symbol, elem2: &Symbol) -> Symbol {
    Symbol {
        weight: elem1.weight + elem2.weight,
        max_merged_count: cmp::max(elem1.max_merged_count, elem2.max_merged_count) + 1,
        min_merged_count: cmp::min(elem1.min_merged_count, elem2.min_merged_count) + 1,
    }
}

fn read_file(file_name: &str) -> Result<SymbolList, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut weight_list: SymbolList = vec![];

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let weight = usize::from_str(split_line[0]).unwrap();
        weight_list.push(Symbol {
            weight,
            max_merged_count: 0,
            min_merged_count: 0,
        });
    }

    Ok(weight_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(run("Testcase1.txt"), (4, 2));
        assert_eq!(run("Testcase2.txt"), (5, 2));
        assert_eq!(run("Testcase3.txt"), (6, 3));
    }
}