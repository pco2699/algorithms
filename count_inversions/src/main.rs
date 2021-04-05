use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let vec = read_file()?;

    println!("{}", count(&vec));
    Ok(())
}

fn count(arr: &[i32]) -> i64 {
    let (res, _) = count_and_merge(arr);
    res
}

fn count_and_merge(arr: &[i32]) -> (i64, Vec<i32>) {
    let n = arr.len();
    if n == 1 {
        return (0, vec![arr[0]])
    }
    let (x, sorted_x) = count_and_merge(&arr[0..n/2]);
    let (y, sorted_y) = count_and_merge(&arr[n/2..]);
    let (z, merged) = count_split_inv(sorted_x, sorted_y);

    (x + y + z, merged)
} 

fn count_split_inv(b: Vec<i32>, c: Vec<i32>) -> (i64, Vec<i32>) {
    let mut d = vec![];
    
    let mut i = 0; let mut j = 0;
    let mut res = 0;

    while i < b.len() && j < c.len() {
        if b[i] < c[j] {
            d.push(b[i]);
            i += 1;
        } else if b[i] > c[j] {
            res += b.len() - i;
            d.push(c[j]);
            j += 1;
        }
    }
    if i < b.len() {
        d.extend_from_slice(&b[i..]);
    }
    if j < c.len() {
        d.extend_from_slice(&c[j..]);
    }

    (res as i64, d)
}

fn read_file() -> Result<Vec<i32>, Error> {
    let input = File::open("./IntegerArray.txt")?;
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
    fn test_count() {

        assert_eq!(count(&vec![1,3,5,2,4,6]), 3);
        assert_eq!(count(&vec![1,5,3,2,4]), 4);
        assert_eq!(count(&vec![5,4,3,2,1]), 10);
        assert_eq!(count(&vec![1,6,3,2,4,5]), 5);
        assert_eq!(count(&vec![9, 12, 3, 1, 6, 8, 2, 5, 14, 13, 11, 7, 10, 4, 0]), 56);
        assert_eq!(count(&vec![37, 7, 2, 14, 35, 47, 10, 24, 44, 17, 34, 11, 16, 48, 1, 39, 6, 33, 43, 26, 40, 4, 28, 5, 38, 41, 42, 12, 13, 21, 29, 18, 3, 19, 0, 32, 46, 27, 31, 25, 15, 36, 20, 8, 9, 49, 22, 23, 30, 45]), 590);
        assert_eq!(count(&vec![4, 80, 70, 23, 9, 60, 68, 27, 66, 78, 12, 40, 52, 53, 44, 8, 49, 28, 18, 46, 21, 39, 51, 7, 87, 99, 69, 62, 84, 6, 79, 67, 14, 98, 83, 0, 96, 5, 82, 10, 26, 48, 3, 2, 15, 92, 11, 55, 63, 97, 43, 45, 81, 42, 95, 20, 25, 74, 24, 72, 91, 35, 86, 19, 75, 58, 71, 47, 76, 59, 64, 93, 17, 50, 56, 94, 90, 89, 32, 37, 34, 65, 1, 73, 41, 36, 57, 77, 30, 22, 13, 29, 38, 16, 88, 61, 31, 85, 33, 54]), 2372);
    }

}