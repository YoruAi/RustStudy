fn main() {
    println!("String is a special vector");
    println!("String is UTF-8 encoded");

    let mut s = String::new();
    let mut s1 = "hello world".to_string();
    let mut s2 = String::from("hello world! ");

    s1.push_str("hello");
    s1.push(',');
    let s3 = s1 + &s2; // s1 is moved here
    let s = format!("{s2}-{s3}");

    println!("String doesn't support index");
    println!("Bytes, Scalar Values, and Grapheme Clusters");
    let s = String::from("你好世界");
    let s1 = &s[0..3]; // unsafe, if 0..2 it will panic
    for c in s.chars() {
        // scalar values
        println!("{c}");
    }
    for b in s.bytes() {
        // bytes
        println!("{b}");
    }

    let _ = s.contains("你好");
    let _ = s.starts_with("你好");
    let _ = s.ends_with("世界");
    let _ = s.find("你");
    let _ = s.replace("你", "我");
    let _ = s.trim();
    let _ = s.split_whitespace();
    let _ = s.split('_');
    let _ = s.to_lowercase();
    let _ = s.to_uppercase();
}
