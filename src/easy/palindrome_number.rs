/*
    Given an integer x, return true if x is a , and false otherwise.
*/

/*
    Naive implementation
    Basically use i32 to reverse the number and check if it is equal to the original number.
*/
pub fn is_int_palindrome(x: i32) -> bool {

    /* 
        Negative numbers are not palindromes 
        because they have a negative sign
    */
    if x < 0 {
        return false;
    }

    let mut reversed: i32 = 0;
    let mut original: i32 = x;
    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }
    reversed == x
}

/*
    String implementation
    Convert the number to a string, reverse it, and check if it is equal to the original string.
*/
pub fn us_palindrome_int_str(x: i32) -> bool {
    // Handle the edge case of negative numbers
    if x < 0 {
        return false;
    }

    // Convert the integer to a string
    let original_str: String = x.to_string();

    // Loop the characters and collect them in reverse order
    let reversed_str: String = original_str.chars().rev().collect();

    // Compare the original string with the reversed string
    original_str == reversed_str
}

/*
    Attempt at a more efficient solution
    comparing the first and last digits in an iterative manner.
    until the middle. If one of the digits does not match, return false.
*/
pub fn is_palindrome_iterative(x: i32) -> bool {
    // Handle the edge case of negative numbers
    if x < 0 {
        return false;
    }

    // Convert the integer to a string
    let original_str: String = x.to_string();
    let len = original_str.len();

    // Loop until the middle of the string
    for i in 0..len / 2 {
        // if the mirrored characters do not match, return false
        if original_str.chars().nth(i) != original_str.chars().nth(len - 1 - i) {
            return false;
        }
    }
    true
}

