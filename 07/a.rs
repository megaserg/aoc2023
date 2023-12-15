use std::collections::HashMap;
use std::io;
use Type::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn get_type(s: &str) -> Type {
    let mut cts: HashMap<char, u32> = HashMap::new();

    for c in s.chars() {
        let count = cts.entry(c).or_insert(0);
        *count += 1;
    }

    if cts.len() == 1 {
        return FiveKind;
    }

    assert_eq!(cts.values().sum::<u32>(), 5);
    let mut vals: Vec<u32> = cts.values().cloned().collect();
    vals.sort();

    if cts.len() == 2 {
        if vals == vec![1, 4] {
            return FourKind;
        }
        if vals == vec![2, 3] {
            return FullHouse;
        }
        assert!(false);
    }

    if cts.len() == 3 {
        if vals == vec![1, 1, 3] {
            return ThreeKind;
        }
        if vals == vec![1, 2, 2] {
            return TwoPair;
        }
        assert!(false);
    }

    if cts.len() == 4 {
        assert!(vals == vec![1, 1, 1, 2]);
        return OnePair;
    }

    return HighCard;
}

fn get_num(s: &str) -> [u32; 5] {
    let map: HashMap<char, u32> = vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
    ].into_iter().collect();

    let mut a: [u32; 5] = Default::default();

    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            a[i] = c.to_digit(10).unwrap();
        } else {
            a[i] = map[&c];
        }
    }

    return a
}

fn main() {
    let mut input = String::new();

    let mut cards = Vec::new();

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let (card, bid_str) = input.trim().split_once(" ").unwrap();
        let bid = bid_str.parse::<u32>().unwrap();

        cards.push((get_type(card), get_num(card), bid));

        input.clear();
    }

    cards.sort();

    let mut score = 0;
    for (rank, (_type, _num, bid)) in cards.iter().enumerate() {
        score += ((rank as u32) + 1) * (*bid);
        eprintln!("{} * {}", rank, bid);
    }
    println!("{}", score);
}