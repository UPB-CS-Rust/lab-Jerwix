fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut largest = 0;
    let mut smallest = 999;
    for x in input {
        if x > largest {
            largest = x
        }
        if x < smallest {
            smallest = x
        }
    }

    println!("{} is largest and {} is smallest", largest, smallest);
}
