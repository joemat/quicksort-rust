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


#[cfg(test)]
mod qsort {
    use crate::q_sort;

    #[test]
    fn empty_vector() {
        let empty_vector: Vec<i32> = Vec::new();
        assert_eq!(empty_vector.clone(), q_sort(empty_vector.clone()));
    }

    #[test]
    fn single_value() {
        assert_eq!(vec![1], q_sort(vec![1]));
    }

    #[test]
    fn simple_sort() {
        assert_eq!(vec![1, 2, 3], q_sort(vec![3,2,1]));
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(vec![1,1,1,3], q_sort(vec![1,1,3,1]));
    }
}


mod args {
    use crate::as_ints;

    #[test]
    fn all_args_are_numbers() {
        assert_eq!(vec![1, 2, 4, 3], as_ints(vec!["1".to_string(), "2".to_string(), "4".to_string(), "3".to_string()]));
    }

    #[test]
    #[should_panic]
    fn panic_when_non_number_is_passed() {
        as_ints(vec!["non-number".to_string()]);
    }
}