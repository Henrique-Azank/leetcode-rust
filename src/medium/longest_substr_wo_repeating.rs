use std::collections::HashMap;

// Public function solution
pub fn length_of_longest_substring(s: String) -> i32 {
    // Instantiate a HashMap for storing the characters
    let mut char_map: HashMap<char, i32> = HashMap::new();

    // Instantiate the substring sizes and indexes
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut max_size: i32 = 0;

    // Loop through the characters and indexes
    for (index, ch) in s.char_indices() {
        // Index as a i32
        let index_32: i32 = index as i32;

        // Check if the character already exists
        let has_char: Option<&i32> = char_map.get(&ch);

        // Check if the char is already contained or not
        match has_char {
            // Char already in the map
            Some(ch_index) => {
                // If the found index is out of bounds
                if *ch_index < left {
                    // Map the right of the substring
                    right = index_32;
                } else {
                    // Finish the substring
                    max_size = max_size.max(right - left + 1);
                    left = *ch_index + 1;
                    right = index_32;
                }
            }
            // Char not in the map
            None => {
                // Map the right of the substring
                right = index_32;
            }
        }

        // Insert the character in the map
        char_map.insert(ch, index_32);
    }

    // If the max size is zero, return the size of the string
    if max_size == 0 {
        return s.chars().count() as i32;
    }

    // Return the max size
    return max_size.max(right - left + 1);
}
