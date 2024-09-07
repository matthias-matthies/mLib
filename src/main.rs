fn binary_search<T: std::cmp::PartialOrd + std::fmt::Debug>(needle: T, haystack: &Vec<T>) -> Option<usize> {
    return _binary_search(needle, haystack, 0, haystack.len());
}

fn _binary_search<T: std::cmp::PartialOrd + std::fmt::Debug>(needle: T, haystack: &Vec<T>, left_bound: usize, right_bound: usize) -> Option<usize> {
    let middle = ((right_bound - left_bound) / 2) + left_bound;
    if right_bound - left_bound <= 2 {
        return if haystack[middle] == needle {
            Some(middle)
        } else {
            None
        }
    }
    return if haystack[middle] > needle {
        _binary_search(needle, haystack, left_bound, middle)
    } else {
        _binary_search(needle, haystack, middle, right_bound)
    }
}

fn merge_sort<T: std::cmp::PartialOrd + Copy>(arr: &mut Vec<T>) {
    _merge_sort(arr, 0, arr.len() - 1);
}

fn _merge_sort<T: std::cmp::PartialOrd + Copy>(arr: &mut Vec<T>, left_bound: usize, right_bound: usize) {
    if left_bound < right_bound {
        let middle = (left_bound + right_bound) / 2;
        _merge_sort(arr, left_bound, middle);
        _merge_sort(arr, middle + 1, right_bound);
        merge(arr, left_bound, middle, right_bound);
    }
}

fn merge<T: std::cmp::PartialOrd + Copy>(arr: &mut Vec<T>, left_bound: usize, mid: usize, right_bound: usize) {
    let left_arr = &arr[left_bound..=mid].to_owned();
    let right_arr = &arr[mid + 1..=right_bound].to_owned();

    let mut i = 0;
    let mut j = 0;

    for k in left_bound..=right_bound {
        if i < left_arr.len() && j < right_arr.len() {
            if left_arr[i] < right_arr[j] {
                arr[k] = left_arr[i];
                i += 1;
            } else {
                arr[k] = right_arr[j];
                j += 1;
            }
        } else if i >= left_arr.len() {
            arr[k] = right_arr[j];
            j += 1;
        } else {
            arr[k] = left_arr[i];
            i += 1;
        }
    }
}

fn main() {
    let mut arr: Vec<i32> = Vec::new();
    arr.push(3);
    arr.push(2);
    arr.push(3);
    arr.push(3);
    arr.push(3);
    arr.push(7);
    arr.push(4);
    arr.push(5);
    arr.push(6);
    arr.push(1);
    arr.push(8);
    let needle = 3;
    merge_sort(&mut arr);
    let bin = binary_search(needle, &arr);
    println!("{:#?}", bin);
    println!("Hello World");
}
