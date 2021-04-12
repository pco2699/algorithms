use std::fs::File;
use std::io::{BufReader, BufRead, Error};

enum PivotSelectRule {
    First,
    Last,
    Median
}

fn main() {
    let result = exec(Some("QuickSort.txt".to_owned()), &PivotSelectRule::First);
    println!("{}", result);

    let result2 = exec(Some("QuickSort.txt".to_owned()), &PivotSelectRule::Last);
    println!("{}", result2);

    let result3 = exec(Some("QuickSort.txt".to_owned()), &PivotSelectRule::Median);
    println!("{}", result3);
}

fn exec(file_name: Option<String>, rule: &PivotSelectRule) -> i32 {
    let mut vec = match file_name {
        Some(val) => read_file(val).unwrap(),
        None => vec![3, 2, 1, 4, 5]
    };
    let mut sum = 0;
    quick_sort(&mut vec, rule, &mut sum);
    sum
}

fn quick_sort(arr: &mut[i32], rule: &PivotSelectRule, sum: &mut i32){
    if arr.len() <= 1 {
        return;
    }
    let index = partition(arr, rule);
    quick_sort(&mut arr[0..index], rule, sum);
    *sum += index as i32;
    quick_sort(&mut arr[index+1..], rule, sum);
    *sum += (arr.len() - 1 - index) as i32;
}

fn swap(arr: &mut[i32], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn partition(arr: &mut[i32], rule: &PivotSelectRule) -> usize {
    let p_index = choose_pivot(arr, &rule);
    let p = arr[p_index];
    swap(arr, 0, p_index);

    let mut i = 1;
    for j in 1..arr.len() {
        if arr[j] < p {
            swap(arr, j, i);
            i += 1;
        }
    }
    swap(arr, 0, i-1);
    i-1
}

fn choose_pivot(arr: &[i32], rule: &PivotSelectRule) -> usize {
    match rule {
        PivotSelectRule::First => 0,
        PivotSelectRule::Last => arr.len()-1,
        PivotSelectRule::Median => find_median(arr, 0, arr.len()-1, (arr.len()-1)/2)
    }
}

fn find_median(arr: &[i32], i: usize, j: usize, k: usize) -> usize {
    let x = arr[i] - arr[j];
    let y = arr[j] - arr[k];
    let z = arr[i] - arr[k];
    if x * y > 0 {
        return j;
    }
    if x * z > 0 {
        return k;
    }
    i
}

fn read_file(file_name: String) -> Result<Vec<i32>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec = vec![];

    for line in buffered.lines() {
        vec.push(line.unwrap().parse::<i32>().unwrap());
    }

    Ok(vec)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec() {
        assert_eq!(exec(Some("Case1.txt".to_owned()), &PivotSelectRule::First), 6);
        assert_eq!(exec(Some("Case1.txt".to_owned()), &PivotSelectRule::Last), 10);
        assert_eq!(exec(Some("Case1.txt".to_owned()), &PivotSelectRule::Median), 6);

        assert_eq!(exec(Some("Case2.txt".to_owned()), &PivotSelectRule::First), 7);
        assert_eq!(exec(Some("Case2.txt".to_owned()), &PivotSelectRule::Last), 8);
        assert_eq!(exec(Some("Case2.txt".to_owned()), &PivotSelectRule::Median), 6);

        assert_eq!(exec(Some("Case3.txt".to_owned()), &PivotSelectRule::First), 71);
        assert_eq!(exec(Some("Case3.txt".to_owned()), &PivotSelectRule::Last), 73);
        assert_eq!(exec(Some("Case3.txt".to_owned()), &PivotSelectRule::Median), 56);

    }
}