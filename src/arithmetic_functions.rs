/// Add all the numbers in a list together
pub fn sum_list (list: &[i128]) -> i128 {
    if list.len() == 1 {
        return list[0]
    }
    list[0] + sum_list(&list[1..])
}

/// Calculates the average between the numbers of a list
pub fn avg (list: &[i128]) -> i128 {
    if list.len() == 1 {
        return list[0]
    }
    sum_list(list) / list.len() as i128
}

// Returns the largest element in a list
pub fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> Option<T> {
    let length = list.len();
    if length == 0 {
        println!("Invalid vector provided");
        return None;
    } else if length == 1 {
        return Some(list[0]);
    }
    let first = list.get(0).unwrap();
    let second = list.get(1).unwrap();

    let largest_num = if first >= second {
        first
    } else {
        second
    };
    if length == 2 { return Some(*largest_num); }
    let mut new_list = Vec::from(&list[2..]);
    new_list.push(*largest_num);
    largest(&new_list)
}
