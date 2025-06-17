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
    // Instantiate the index and lenght of the longest palindromic substring
    let mut start_index: usize = 0;
    let mut max_length: usize = 1;

    /*
        Instantiate the 2D vector to store the results of the palindromic checks.
        Since the max value of the string length is 1000, we can use a 2D vector
        of size 1000x1000 to store the results.
    */
    let mut memoized_subs: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

    // Size of the string
    let string_len: usize = s.len();

    /*
        Mark all the single characters substrings as palindromes
        (i.e max_lenght = 1)
    */
    for i in 0..string_len {
        memoized_subs[i][i] = true;
    }

    /*
        Check for the substrings with length 2
        If the characters are equal, mark the substring as a palindrome
    */
    for i in 0..string_len - 1 {
        // If the current character is equal to the next character
        if s.as_bytes()[i] == s.as_bytes()[i + 1] {
            // Mark the substring as a palindrome
            memoized_subs[i][i + 1] = true;

            // Update the start index and max length
            start_index = i;
            max_length = 2;
        }
    }

    /*
        Check for the substrings with length greater than 2
        If the characters at the start and end of the substring are equal,
        and the substring between them is a palindrome, mark the substring as a palindrome.
    */
    for length in 3..=string_len {
        // Limit for the search
        let search_limit: usize = string_len - length + 1;

        for i in 0..search_limit {
            // Calculate the end index of the substring
            let j: usize = i + length - 1;

            // Check if the characters at the start and end of the substring are equal
            // and if the substring between them is a palindrome
            if s.as_bytes()[i] == s.as_bytes()[j] && memoized_subs[i + 1][j - 1] {
                // Mark the substring as a palindrome
                memoized_subs[i][j] = true;

                // Update the start index and max length
                start_index = i;
                max_length = length;
            }
        }
    }

    // Return the longest palindromic substring
    return s[start_index..start_index + max_length].to_string();
}
