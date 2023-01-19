fn mean(v: &Vec<i32>) -> f64 {
    let sum: i32 = v.iter().sum();
    sum as f64 / v.len() as f64
}

fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mx = 0;
    for (_, count) in &map {
        if *count > mx {
            mx = *count;
        }
    }

    let mut ret = Vec::new();
    for (key, count) in map {
        if count == mx {
            ret.push(*key);
        }
    }

    ret.sort();
    ret
}

pub fn stats(v: &Vec<i32>){
    println!("Mean: {}", mean(v));
    println!("Median: {}", median(v));
    println!("Modes: {:?}", mode(v));
}