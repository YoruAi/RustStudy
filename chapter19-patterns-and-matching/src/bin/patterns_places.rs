fn main() {
    let x = Some(5);
    let v = vec!['a', 'b', 'c'];
    // 1. match
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    // 2. `if`-let/`let`-else
    if let Some(i) = x {
    } else {
    }
    let Some(i) = x else {
        return;
    };
    // 3. while-let
    while let Some(i) = x {
        break;
    }
    // 4. for
    for (index, value) in v.iter().enumerate() {}
    // 5. let
    let (x, y) = (1, 2);
    // 6. function parameters
    fn coordinates(&(x, y): &(i32, i32)) {}
    let point = (3, 5);
    coordinates(&point);
}
