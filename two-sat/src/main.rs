use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use rand::Rng;


#[derive(Clone, Debug)]
struct Clause {
    left: Variable,
    right: Variable
}

#[derive(Clone, Debug)]
struct Variable {
    value: u32,
    sign: bool
}

impl Variable {
    fn new(input: i32) -> Variable {
        Variable {
            value: input.abs() as u32 - 1,
            sign: if input > 0 { true } else { false }
        }
    }
}

type Clauses = Vec<Clause>;
type index = usize;
type Choices = Vec<bool>;

fn main() {
    solve("Testcase4.txt");
    dbg!(solve("2sat1.txt"));
    dbg!(solve("2sat2.txt"));
    dbg!(solve("2sat3.txt"));
    dbg!(solve("2sat4.txt"));
    dbg!(solve("2sat5.txt"));
    dbg!(solve("2sat6.txt"));
}

fn solve(file_name: &str) -> bool {
    let (clauses, num) = read_file(file_name).unwrap();
    let reduced_clauses = preprocess(&clauses);

    papadimitriou(&reduced_clauses, &num)
}

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: usize) -> usize {
    assert!(x > 0);
    num_bits::<usize>() - x.leading_zeros() as usize - 1
}

fn papadimitriou(clauses: &Clauses, num: &usize) -> bool {
    let reduced_num = clauses.len();
    if reduced_num <= 0 {
        return true;
    }

    let log_num = log_2(reduced_num);
    for _ in 0..log_num {
        println!("log_num: {}", log_num);
        println!("reduced_num: {}", reduced_num);

        let mut choices = create_choices(&num);
        let end = 2 * reduced_num.pow(2);

        println!("end: {}", end);
        for _ in 0..end {
            match check_choices(&clauses, &choices) {
                Some(val) => flip_choices(&val, &clauses, &mut choices),
                None => return true
            }
        }
    }

    false
}

fn flip_choices(index: &usize, clauses: &Clauses, choices: &mut Choices) {
    let mut rng = rand::thread_rng();
    let clause = &clauses[*index];
    let chosen_index = if rng.gen::<bool>() {
        clause.left.value
    } else {
        clause.right.value
    } as usize;
    choices[chosen_index] = !choices[chosen_index];
}

fn check_choices(clauses: &Clauses, choices: &Choices) -> Option<usize> {
    for (i, clause) in clauses.iter().enumerate() {
        let left = !(choices[clause.left.value as usize] ^ clause.left.sign);

        let right = !(choices[clause.right.value as usize] ^ clause.right.sign);
        let result = left | right;
        if !result {
            return Some(i)
        } 
    }
    None
}

fn create_choices(num: &usize) -> Choices {
    let mut rng = rand::thread_rng();
    let mut choices = vec![];

    for _ in 0..*num {
        choices.push(rng.gen());
    }
    choices
}

fn preprocess(clauses: &Clauses) -> Clauses {
    let mut clauses_calc = clauses.clone();

    for _ in 0..100 {
        let mut clauses_new = vec![];

        let mut positive_set: HashSet<u32> = HashSet::new();
        let mut negative_set: HashSet<u32> = HashSet::new();
        for clause in &clauses_calc {
            set_to_set(&clause, &mut positive_set, &mut negative_set);
        }

        let mut index_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, clause) in clauses_calc.iter().enumerate() {
            let vec_left = index_map.entry(*&clause.left.value as usize).or_insert(vec![]);
            vec_left.push(i);

            let vec_right = index_map.entry(*&clause.right.value as usize).or_insert(vec![]);
            vec_right.push(i);
        }
        let mut index_to_be_deleted: HashSet<usize> = HashSet::new();
        for diff in positive_set.symmetric_difference(&negative_set) {
            let vec = index_map.get(&(*diff as usize)).unwrap();
            for v in vec {
                index_to_be_deleted.insert(*v);
            }
        }

        for (i, clause) in clauses_calc.iter().enumerate() {
            if index_to_be_deleted.contains(&i) {
                continue;
            } else {
                clauses_new.push(clause.clone());
            }
        }
        
        clauses_calc = clauses_new.clone();
    }
    
    clauses_calc
}

fn set_to_set(clause: &Clause, pos_set: &mut HashSet<u32> , neg_set: &mut HashSet<u32>) {
    if clause.left.sign {
        pos_set.insert(clause.left.value);
    } else {
        neg_set.insert(clause.left.value);
    }

    if clause.right.sign {
        pos_set.insert(clause.right.value);
    } else {
        neg_set.insert(clause.right.value);
    }
}

fn read_file(file_name: &str) -> Result<(Clauses, usize), Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);
    let mut caluses: Clauses = vec![];
    let mut num = 0;

    for(index, line) in buffered.lines().enumerate() {
        let unwraped_line = line.unwrap();
        if index == 0 {
            num = usize::from_str(&unwraped_line).unwrap();
            continue;
        }
        let split_line = unwraped_line.split(" ").collect::<Vec<&str>>();
        let left = i32::from_str(split_line[0]).unwrap();
        let right = i32::from_str(split_line[1]).unwrap();

        let var_left = Variable::new(left);
        let var_right = Variable::new(right);

        caluses.push(Clause{left: var_left, right: var_right});
    }

    Ok((caluses, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        assert_eq!(solve("Testcase1.txt"), true);
        assert_eq!(solve("Testcase2.txt"), true);
        assert_eq!(solve("Testcase3.txt"), false);
        assert_eq!(solve("Testcase4.txt"), true);
        assert_eq!(solve("Testcase5.txt"), false);
    }
}