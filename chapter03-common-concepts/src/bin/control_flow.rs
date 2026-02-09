fn main() {
    println!("[if-else]");
    let number = 3;
    if number > 5 {
        println!("number is more than five");
    } else if number == 5 {
        println!("number is five");
    } else {
        println!("number is less than five");
    }
    let condition = true;
    let mut number = if condition { 5 } else { 6 }; // must be compatible types

    println!("[loops] loop, for, while");
    let mut count = 0;
    // loop label
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
