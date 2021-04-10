use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some((_argv0, unsorted)) = args.split_first() {
        run_q_sort(as_ints(unsorted.to_vec()));
    } else {
        // no arguments passed
        println!("Nothing to sort");
    }
}

fn as_ints(strings: Vec<String>) -> Vec<i32> {
    strings.into_iter().map(|string_val| {
        string_val.parse::<i32>().expect("Numbers expected")
    }).collect()
}

fn run_q_sort(unsorted: Vec<i32>) {
    println!("Sorting: {:?}", unsorted);
    let sorted = q_sort(unsorted);
    println!("Sorted:  {:?}", sorted);
}

fn q_sort(unsorted: Vec<i32>) -> Vec<i32> {
    return if let Some((&pivot, remaining)) = unsorted.split_first() {
        sort_with_pivot(pivot, remaining.to_vec())
    } else {
        // unsorted contains only one element => this is already sorted
        unsorted
    };
}

fn sort_with_pivot(pivot: i32, numbers: Vec<i32>) -> Vec<i32> {
    let less = numbers.clone().into_iter().filter(|&num| num < pivot).collect();
    let greater_eq = numbers.clone().into_iter().filter(|&num| num >= pivot).collect();

    concatenate(q_sort(less),
                &pivot,
                q_sort(greater_eq))
}

fn concatenate(less: Vec<i32>, pivot: &i32, greater_eq: Vec<i32>) -> Vec<i32> {
    vec![less, greater_eq].join(pivot)
}


#[cfg(test)]
mod qsort {
    use crate::q_sort;

    #[test]
    fn empty_vector() {
        let empty_vector: Vec<i32> = Vec::new();
        assert_eq!(empty_vector.clone(),
                   q_sort(empty_vector.clone()));
    }

    #[test]
    fn single_value() {
        assert_eq!(vec![1],
                   q_sort(vec![1]));
    }

    #[test]
    fn two_values() {
        assert_eq!(vec![1, 2],
                   q_sort(vec![2, 1]));
    }


    #[test]
    fn simple_sort() {
        assert_eq!(vec![1, 2, 3],
                   q_sort(vec![3, 2, 1]));
    }

    #[test]
    fn simple_sort_more_values() {
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
                   q_sort(vec![3, 2, 1, 14, 13, 12, 11, 10, 4, 5, 6, 9, 8, 7]));
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(vec![1, 1, 1, 2, 2, 3, 3],
                   q_sort(vec![1, 3, 1, 3, 1, 2, 2]));
    }
}

#[cfg(test)]
mod args {
    use crate::as_ints;

    #[test]
    fn all_args_are_numbers() {
        assert_eq!(vec![1, 2, 4, 3],
                   as_ints(vec!["1".to_string(),
                                "2".to_string(),
                                "4".to_string(),
                                "3".to_string()]));
    }

    #[test]
    #[should_panic]
    fn panic_when_non_number_is_passed() {
        as_ints(vec!["non-number".to_string()]);
    }
}