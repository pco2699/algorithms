use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    // let vec = read_file()?;

    println!("{}", count(&vec![1,6,3,2,4,5]));
    Ok(())
}

fn count(arr: &[i32]) -> (i64, Vec<i32>) {
    let n = arr.len();
    if n == 1 {
        return 0
    }
    let (x, sorted_x) = count(&arr[0..n/2]);
    let (y, sorted_y) = count(&arr[n/2..]);
    let (z, merged) = count_split_inv(&arr);

    (x + y + z, merged)
} 

fn count_split_inv(arr: &[i32]) -> (i64, Vec<i32>) {
    let n = arr.len();
    let b = &arr[0..n/2];
    let c = &arr[n/2..];
    let mut d = vec![];
    
    let mut i = 0; let mut j = 0;
    let mut res = 0;

    while i < b.len() && j < c.len() {
        if b[i] < c[j] {
            d.push(b[i]);
            i += 1;
        } else if b[i] > c[j] {
            res += n / 2 - i;
            d.push(c[j]);
            j += 1;
        }
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
    }

}