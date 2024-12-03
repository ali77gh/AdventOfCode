use std::{collections::HashMap, fs};

fn main() {
    part_2();
}

fn part_1() {
    let lines = fs::read_to_string("./input.txt").unwrap();

    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();
    for line in lines.lines() {
        let sp: Vec<&str> = line.split("   ").collect();
        list1.push(sp[0].parse().unwrap());
        list2.push(sp[1].parse().unwrap());
    }

    list1.sort();
    list2.sort();
    let mut l1 = list1.iter();
    let mut l2 = list2.iter();

    let mut sum_diff: u32 = 0;
    while let (Some(i1), Some(i2)) = (l1.next(), l2.next()) {
        let diff = i1.abs_diff(*i2);
        sum_diff += diff;
    }
    println!("sum_diff: {}", sum_diff);
}

fn part_2() {
    let lines = fs::read_to_string("./input.txt").unwrap();

    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();
    for line in lines.lines() {
        let sp: Vec<&str> = line.split("   ").collect();
        list1.push(sp[0].parse().unwrap());
        list2.push(sp[1].parse().unwrap());
    }

    let mut map = HashMap::<u32, u32>::new();

    for i in list2 {
        match map.get_mut(&i) {
            Some(x) => {
                *x += 1;
            }
            None => {
                map.insert(i, 1);
            }
        }
    }

    let mut sum: u32 = 0;
    for i in list1 {
        sum += i * map.get(&i).unwrap_or(&0);
    }
    println!("sum: {}", sum)
}
