#![allow(dead_code)]
pub fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn slice_string_fn() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slices: {hello} -- {world}");
}

pub fn slice_array_fn() {
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
    println!("Finished slice_array_fn call");
}