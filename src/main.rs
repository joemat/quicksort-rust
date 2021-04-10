use std::env;

fn as_ints(strings: Vec<String>) -> Vec<i32> {
    strings.into_iter().map(|string_val| {
        string_val.parse::<i32>().expect("Numbers expected")
    }).collect()
}

fn split_by_value(numbers: Vec<i32>, pivot: i32) -> (Vec<i32>, Vec<i32>) {
    let less = numbers.clone().into_iter().filter(|&num| num < pivot).collect();
    let greater_eq = numbers.clone().into_iter().filter(|&num| num >= pivot).collect();
    return (less, greater_eq)
}

fn q_sort(unsorted: Vec<i32>) -> Vec<i32> {

    if let Some((pivot, remaining)) = unsorted.split_first() {
        let (lower, greater_eq) =  split_by_value(remaining.to_vec(), *pivot);

        // join lists: lower + pivot + greater_eq
        return vec![q_sort(lower), q_sort(greater_eq)].join(pivot);
    }
    // unsorted contains only one element => this is already sorted
    return unsorted;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some((_argv0, args_)) = args.split_first() {
        let numbers = as_ints(args_.to_vec());
        println!("Sorting: {:?}", numbers);
        println!("Sorting: {:?}", q_sort(numbers));
    } else {
        println!("Nothing to sort");
    }
}
