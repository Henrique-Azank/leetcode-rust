/*
    Given a string s, return the longest palindromic substring in s.

    A string is palindromic if it reads the same forward and backward.
    A substring is a contiguous non-empty sequence of characters within a string.
*/

/*
    Brute force solution:
    1. Generate all possible substrings of the input string.
    2. Check each substring to see if it is a palindrome.
    3. Keep track of the longest palindromic substring found.
*/
pub fn brute_longest_palindrome(s: String) -> String {
    // Helper function to check if a string is a palindrome
    fn is_palindrome(s: &str, start: usize, end: usize) -> bool {
        // Instantiate the mutable start and end indices
        let mut left: usize = start;
        let mut right: usize = end;

        // While the left index is less than the right index
        while left < right {
            // If the characters are not equal, return false
            if s.as_bytes()[left] != s.as_bytes()[right] {
                return false;
            }

            // Update the indices
            left += 1;
            right -= 1;
        }

        // Return true if the loop completes
        true
    }

    // Get the size of the string
    let n: usize = s.len();

    // Start the max size as 1 (every single character is a palindrome)
    let mut max_size: usize = 1;

    // Start index of the longest palindromic substring
    let mut start_index: usize = 0;

    // Start the iteration
    for left in 0..n {
        // Find the longest palindrome
        for right in left..n {
            // Check if the substring is a palindrome
            if is_palindrome(&s, left, right) && (right - left + 1) > max_size {
                // Update the max size and start index
                max_size = right - left + 1;
                start_index = left;
            }
        }
    }

    // Return the longest palindromic substring
    s[start_index..start_index + max_size].to_string()
}

/*
    Dynamic Programming Approach:

    Memoization of the results of checking for substrings.

    If we know that a substring s[i..j] is a palindrome, we
    can check if s[i-1] == s[j+1] to determine if s[i-1..j+1]
    is also a palindrome.
*/
pub fn dynamic_longest_palindrome(s: String) -> String {
    println!("{s}");
    return "".to_string();
}
