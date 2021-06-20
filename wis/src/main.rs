use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::str::FromStr;
use std::cmp;
use std::collections::HashSet;

type WeightList = Vec<usize>;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}


fn main() {
    let (s, _) = exec("mwis.txt");
    println!("{}", aggregate(&selected_order(), &s));
}

fn exec(file_name: &str) -> (HashSet<usize>, usize) {
    let (weight_list, num) = read_file(file_name).expect("cannnot read the file");
    let mut dp = vec![0; num+1];
    dp[0] = 0;
    dp[1] = weight_list[0];

    for i in 2..num+1 {
        dp[i] = cmp::max(dp[i-1], dp[i-2]+weight_list[i-1]);
    }
    
    let mut s = HashSet::new();
    let mut i = num;
    while i >= 1 {
        if i > 2 && dp[i-1] >= dp[i-2] + weight_list[i-1] {
            i -= 1;
        } else {
            s.insert(i-1);
            if i > 2 {
                i -= 2;
            } else {
                break;
            }
        }
    }
    (s, *dp.last().unwrap())
}

fn aggregate(selected_order: &Vec<usize>, s: &HashSet<usize>) -> String {
    selected_order
        .iter()
        .map(|v| if s.contains(v) {"1"} else {"0"})
        .collect()
}

fn selected_order() -> Vec<usize> {
    return [1, 2, 3, 4, 17, 117, 517, 997]
        .iter()
        .map(|v| v - 1)
        .collect();
}

fn read_file(file_name: &str) -> Result<(WeightList, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut weight_list: WeightList = vec![];
    let mut num = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            let first_line = unwraped_line.split(" ").collect::<Vec<&str>>();
            num = usize::from_str(first_line[0]).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let weight = usize::from_str(split_line[0]).unwrap();
        weight_list.push(weight);
    }

    Ok((weight_list, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec("Testcase1.txt"), (set![1, 3], 8));
        assert_eq!(exec("Testcase2.txt"), (set![1, 3, 5, 7, 9], 2616));
        assert_eq!(exec("Testcase3.txt"), (set![0, 2, 5, 8], 2533));
    }
}