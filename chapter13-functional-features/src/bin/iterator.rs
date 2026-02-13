use chapter12_io_project::Config;
use std::env;

fn main() {
    println!("iterator is lazy");
    println!("iterator impl Iterator trait, next() should end with returning None");
    let mut v1 = ["a".to_string(), "b".to_string()];
    let v1_iter = v1.iter(); // &
    let v1_iter = v1.iter_mut(); // &mut
    let v1_iter = v1.into_iter(); // move
    for val in v1_iter {
        println!("Got: {val}");
    }

    println!("consuming adaptors, calling them will use up the iterator");
    let mut v1 = vec![1, 2, 3];
    let count = v1.iter().count();
    let total: i32 = v1.iter().sum();
    let collect: Vec<i32> = v1.iter().copied().collect();
    println!("iterator adaptors, change some aspect of the original iterator");
    let _ = v1.iter().rev().map(|x| x + 1).filter(|x| x % total == 0);

    println!("iterator is one of zero-cost abstractions");

    // enhance the chapter12-io-project
    let _ = build(env::args());
}

pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();
    let query = args.next().ok_or("Expected a query string")?;
    let file_path = args.next().ok_or("Expected a file_path")?;
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Ok(Config {
        query,
        file_path,
        ignore_case,
    })
}
