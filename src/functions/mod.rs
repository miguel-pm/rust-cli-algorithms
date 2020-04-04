/// ### Arithmetic Functions
/// small module containing functional programming style arithmetic functions
pub mod arithmetic_functions {

    /// Add all the numbers in a list together
    pub fn sum_list (list: &[i32]) -> i32 {
        if list.len() == 1 {
            return list[0]
        }
        list[0] + sum_list(&list[1..])
    }

    /// Calculates the average between the numbers of a list
    pub fn avg (list: &[i32]) -> i32 {
        if list.len() == 1 {
            return list[0]
        }
        sum_list(list) / list.len() as i32
    }
}
