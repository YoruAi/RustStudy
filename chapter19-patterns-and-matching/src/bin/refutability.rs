fn main() {
    println!("patterns that will match for any possible value passed are irrefutable");
    println!("patterns that can fail to match for some possible value are refutable");
    println!("'function parameters', 'let' and 'for' can only accept irrefutable patterns");
    println!(
        "'if-let', 'let-else' and 'while-let' accept refutable and irrefutable patterns,\
        but compiler warns against irrefutable patterns"
    );
}
