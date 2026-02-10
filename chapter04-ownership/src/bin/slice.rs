fn main() {
    // A slice is a kind of reference

    // string slice
    let mut s = String::from("hello world");
    let _hello = &s[..5]; // string_slice[0..=4]
    let _world = &s[6..];
    let _slice = "hello world";
    let slice = first_word(&s);
    // s.clear();
    let _ = slice.len();
    s.clear();

    // array slice
    let mut a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    // a[0] = 0;
    assert_eq!(slice, &[2, 3]);
    a[0] = 0;
    let _ = a[0];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
