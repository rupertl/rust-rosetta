// Implements http://rosettacode.org/wiki/Range_expansion

#![feature(phase)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use std::iter::range_inclusive;

#[cfg(not(test))]
fn main() {
    let range = "-6,-3-1,3-5,7-11,14,15,17-20";
    println!("Expanded range: {}", expand_range(range));
}

// Expand a string containing numbers and ranges, into a vector of numbers
fn expand_range(range: &str) -> Vec<int> {
    let mut result = vec![];

    for item in range.split(',') {
        result.extend(expand_item(item).into_iter());
    }

    result
}

// Expand a single element, which can be a number or a range.
fn expand_item(item: &str) -> Vec<int> {
    // Handle the case of a single number
    for cap in regex!(r"^(-?\d+)$").captures_iter(item) {
        return vec![from_str(cap.at(0)).unwrap()]
    }

    // Handle the case of a range
    for cap in regex!(r"^(-?\d+)-(-?\d+)$").captures_iter(item) {
        let left: int = from_str(cap.at(1)).unwrap();
        let right = from_str(cap.at(2)).unwrap();

        // Generate and collect a range between them
        return range_inclusive(left, right).collect()
    }

    panic!("The item `{}` is not a number or a range!", item);
}

#[test]
fn test_basic() {
    let range = "1-5,6";
    assert!(expand_range(range).as_slice() == [1, 2, 3, 4, 5, 6]);

    let range = "-6,-3-1,3-5,7-11,14,15,17-20";
    assert!(expand_range(range).as_slice() ==
        [-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
}

#[test]
#[should_fail]
fn test_wrong() {
    let range = "one-five,six";
    assert!(expand_range(range).as_slice() == [1, 2, 3, 4, 5, 6]);
}
