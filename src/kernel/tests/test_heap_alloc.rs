use _alloc::{boxed::Box, string::String, vec::Vec};
use println;

pub fn heap_test_string() {
    let s = String::from("heap-string");
    assert_eq!(s.as_str(), "heap-string");
    println!("string heap test pass: {}", s);
}

pub fn heap_test_vec() {
    let mut v: Vec<u32> = Vec::new();
    for i in 1..=4 {
        v.push(i);
    }
    assert_eq!(&v[..], &[1, 2, 3, 4]);
    println!("vec heap test pass: {:?}", v);
}

pub fn heap_test_box() {
    let boxed = Box::new(0x1234_u32);
    assert_eq!(*boxed, 0x1234_u32);
    println!("box heap test pass: {}", *boxed);
}

pub fn run_heap_tests() {
    heap_test_string();
    heap_test_vec();
    heap_test_box();
    println!("heap allocation works surprisingly");
}