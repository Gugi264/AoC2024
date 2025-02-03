use day11::blink_n_times;

fn main() {
    let input = "0 37551 469 63 1 791606 2065 9983586";

    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let res = blink_n_times(&stones, 36);
    println!("stones: {}", res);
}
