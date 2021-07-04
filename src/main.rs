use rand::Rng;
use std::{io, cmp::Ordering, collections::HashMap};

fn receive_input_integer(message: &str) -> i32 {
    loop {
        // receive # of integers to generate
        println!("{}", message);

        //check input error
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break input;
    }
}

fn mean(list: &[i32]) -> Option<f32> {
    let sum = list.iter().sum::<i32>() as f32;
    let count = list.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None
    };

    mean
}

fn partition(list: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match list.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = list.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter()
                .fold((vec![], vec![]), |mut splits, next| {
                    {
                        let (ref mut left, ref mut right) = &mut splits;
                        if next < &pivot {
                            left.push(*next);
                        } else {
                            right.push(*next);
                        }
                    }
                    splits
                });

            Some((left, pivot, right))
        }
    }
}

fn select(list: &[i32], k: usize) -> Option<i32> {
    let part = partition(list);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        },
    }
}


fn median(list: &[i32]) -> Option<f32> {
    let size = list.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(list, (even / 2) -1);
            let snd_med = select(list, even / 2);
            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None
            }
        },
        odd => select(list, odd / 2).map(|x| x as f32)
    }
}

fn mode(list: &[i32]) -> Option<i32> {
    let frequencies = list.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_,count)| count)
        .map(|(value, _)| *value);

    mode
}

fn main() {
    // generate an empty vector
    let mut v: Vec<i32> = Vec::new();

    // receive # of integers to generate
    let message = "Please input the number of integers (0-10) to generate.";
    let num_int = receive_input_integer(&message);

    // generate random integers into vector
    for _n in 0..num_int {
        let rand_int = rand::thread_rng().gen_range(1..11);
        v.push(rand_int)
    }

    // print out the list generated
    println!("List of integers is {:?}", &v);

    // calculate mean
    let mean = mean(&v);

    // calculate median
    let median = median(&v);

    // calculate mode
    let mode = mode(&v);

    // print result
    println!("Mean: {:?}, Median: {:?}, Mode: {:?}", mean.unwrap(), median.unwrap(), mode.unwrap());

}
