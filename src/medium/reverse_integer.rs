/*
    Given a signed 32-bit integer x, return x with its digits reversed.

    If reversing x causes the value to go outside the signed 32-bit
    integer range [-231, 231 - 1], then return 0.

    **Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
*/

/*
    Naive approach for the reverse integer problem.
    1. Convert the integer to a string.
    2. Reverse the string.
    3. Convert the reversed string back to an integer.
    4. Handle the sign of the integer.
    5. If the reversed integer is out of bounds, return 0.
*/
pub fn naive_reverse_integer(x: i32) -> i32 {
    // Convert the i32 to a string
    let mut x_str: String = x.to_string();

    // Check if the number is negative
    let is_negative: bool = x < 0;

    // If the number is negative, remove the negative sign
    if is_negative {
        x_str.remove(0);
    }

    // Invert the string
    let reversed_str: String = x_str.chars().rev().collect();

    // If the number is negative, add the negative sign back
    let reversed_str: String = if is_negative {
        format!("-{}", reversed_str)
    } else {
        reversed_str
    };

    /*
        Try to convert the reversed string back to i32,
        if it panics, return 0
    */
    match reversed_str.parse::<i32>() {
        Ok(reversed_int) => reversed_int,
        Err(_) => 0,
    }
}

// TODO: Implement these test cases in the project itself
// // Test the naive reverse integer solution
// let test_cases: Vec<i32> = vec![123, -123, 120, 0, 1534236469];
// for &test_case in &test_cases {
//     let result = naive_reverse_integer(test_case);
//     println!("Input: {}, Reversed: {}", test_case, result);
// }
