use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use std::str::FromStr;

fn main() {
    let hoge = read_file("Testcase1.txt".to_owned()).unwrap();
    dbg!(&hoge[1]);
}

fn read_file(file_name: String) -> Result<Vec<Vec<i32>>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut vec: Vec<Vec<i32>> = vec![];

    for line in buffered.lines() {
        let mut neighbor_list = vec![];
        let unwraped_line = line.unwrap();
        for s in unwraped_line.split(" ") {
            neighbor_list.push(FromStr::from_str(s).unwrap())
        }
        vec.push(neighbor_list);
    }

    Ok(vec)
}