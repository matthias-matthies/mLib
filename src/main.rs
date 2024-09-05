fn binary_search<T: std::cmp::PartialOrd + std::fmt::Debug>(needle: T, haystack: &Vec<T>) -> Option<usize> {
    return binary_search_help(needle, haystack, 0, haystack.len());
}

fn binary_search_help<T: std::cmp::PartialOrd + std::fmt::Debug>(needle: T, haystack: &Vec<T>, left_bound: usize, right_bound: usize) -> Option<usize> {
    let middle = ((right_bound - left_bound) / 2) + left_bound;
    if right_bound - left_bound <= 2 {
        return if haystack[middle] == needle {
            Some(middle)
        } else {
            None
        }
    }
    return if haystack[middle] > needle {
        binary_search_help(needle, haystack, left_bound, middle)
    } else {
        binary_search_help(needle, haystack, middle, right_bound)
    }
}

fn main() {
    let mut ordered: Vec<i32> = Vec::new();
    ordered.push(1);
    ordered.push(2);
    ordered.push(3);
    ordered.push(3);
    ordered.push(3);
    ordered.push(3);
    ordered.push(4);
    ordered.push(5);
    ordered.push(6);
    ordered.push(7);
    ordered.push(8);
    let needle = 3;
    let bin = binary_search(needle, &ordered);
    println!("{:#?}", bin);
    println!("Hello World");
}
