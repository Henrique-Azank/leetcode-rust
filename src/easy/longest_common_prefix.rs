/*
    Write a function to find the longest common prefix string amongst an array of strings.
    If there is no common prefix, return an empty string "".
*/

/*
    Initial iterative solution
    1. Find the shortest string in the array.
    2. Iterate through each character of the shortest string.
    3. For each character, check if it is the same in all strings at that index.
    4. If it is, append it to the common prefix.
    5. If not, break the loop as we found a mismatch.
*/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // Instantiate the empty string to hold the common prefix
    let mut common_prefix: String = String::new();

    // Find the shortest string in the array
    let shortest_str = strs.iter().min_by_key(|s| s.len()).unwrap();

    // Iterate through each character of the shortest string
    for (i, c) in shortest_str.chars().enumerate() {
        // Check if the character at the current index is the same in all strings
        if strs.iter().all(|s| s.chars().nth(i) == Some(c)) {
            // If it is, append it to the common prefix
            common_prefix.push(c);
        } else {
            // If not, break the loop as we found a mismatch
            break;
        }
    }

    // Return the common prefix
    common_prefix
}
    


