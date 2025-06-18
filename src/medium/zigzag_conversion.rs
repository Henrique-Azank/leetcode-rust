/*

    The string "PAYPALISHIRING" is written in a zigzag pattern on a
    given number of rows like this (you may want to display this pattern
    in a fixed font for better legibility)

    P   A   H   N
    A P L S I I G
    Y   I   R

    And then read line by line: "PAHNAPLSIIGYIR"

    Write the code that will take a string and make this conversion given a number of rows:
    string convert(string s, int numRows)
*/

// Use the necessary collection
use std::collections::HashMap;

/*
    Initial naive solution
    0. Consider the edge case of only one row.
    1. Create a HashMap to store the rows.
    2. Use a boolean to check if we are going down or up the zigzag
       (we start going down).
    3. Loop over the characters in the string (with the index).
    4. Append the current character to the current row.
    5. If we are at the next switch index, we need to change the direction
       (i.e. if we are going down, we switch to going up and vice versa).
    6. Go to the next row.
*/
pub fn naive_zigzag_conversion(s: String, num_rows: i32) -> String {
    // Instantiate a HashMap to store the rows
    let mut rows: HashMap<i32, String> = HashMap::new();

    // Boolean to check if we area going down or up the zigzag (we start going down)
    let mut going_down: bool = true;

    // Current row being populated
    let mut current_row: i32 = 0;

    // Next index to switch the direction (initial is num_rows - 1)
    let mut next_switch_index: usize = num_rows as usize - 1;

    // If only one row, return the string as is
    if num_rows == 1 {
        return s;
    }

    // Loop over the characters in the string (with the index)
    for (index, ch) in s.char_indices() {
        // Append the current character to the current row
        rows.entry(current_row).or_insert_with(String::new).push(ch);

        // If we are at the next switch index, we need to change the direction
        if index == next_switch_index as usize {
            // Switch the direction
            going_down = !going_down;
            // Update the next switch index for the string
            next_switch_index = index + num_rows as usize - 1;
        }

        // Go to the next row
        current_row += if going_down { 1 } else { -1 };
    }

    // Collect the rows and join them into a single string
    let mut result_string: String = String::new();
    for row in 0..num_rows {
        if let Some(row_string) = rows.get(&row) {
            result_string.push_str(row_string);
        }
    }

    println!("Result string: {:?}", rows);

    return result_string;
}

/*
    Optmized solution v1
*/
pub fn optimized_zigzag_conversion(s: String, num_rows: i32) -> String {
    // Handle the edge case of only one row
    if (num_rows == 1) || (num_rows >= s.len() as i32) {
        return s;
    }

    // Instantiate a vector to store the rows
    let mut rows: Vec<String> = vec![String::new(); num_rows as usize];

    // Instantiate the initial index
    let mut row_index: usize = 0;

    // Keep track of the direction (down or up)
    let mut going_down: bool = true;

    // Keep track of the current character index
    for ch in s.chars() {
        // Append the current character to the current row
        rows[row_index].push(ch);

        // If we reached the bottom row, switch direction to up
        if row_index == num_rows as usize - 1 {
            going_down = false;
        }

        // If we are at the top row, switch direction to down
        if row_index == 0 {
            going_down = true;
        }

        // Based on the direction, update the row index
        if going_down {
            row_index += 1;
        } else {
            row_index -= 1;
        }
    }

    return rows.join("");
}

/*
    Unsafe optmizations solution. The following optimizations were performed:
    (Opt 1.) Pre-allocate capacity for String rows to avoid reallocations.
    (Opt 2.) Use a single Vec<u8> instead of Vec<String> for better performance.
    (Opt 3.) Avoid bounds checks by using unchecked indexing where safe.
    (Opt 4.) Minimize the usize and i32 conversions by using usize directly.
    (Opt 5.) Return the final string with from_utf8_unchecked

*/
pub fn unsafe_optimized_zigzag_conversion(s: String, num_rows: i32) -> String {
    // Convert the num_rows to usize for indexing ans store the length of the string
    //Minimize the usize and i32 conversions by using usize directly (Opt 4.)
    let num_rows = num_rows as usize;
    let len = s.len();

    // Handle edge cases
    if num_rows == 1 || num_rows >= len {
        return s;
    }

    // Pre-allocate rows with exact capacity (Opt 1.)
    let mut rows: Vec<Vec<u8>> = vec![Vec::with_capacity((len / num_rows) + 1); num_rows];

    // Initialize the row index and direction
    let mut row_index: usize = 0;
    let mut going_down: bool = true;

    // Process the string character by character as u8 bytes instead of chars
    for ch in s.into_bytes() {
        // Avoid bounds checks by using unchecked indexing (Opt 3.)
        unsafe {
            rows.get_unchecked_mut(row_index).push(ch);
        }

        // Update direction and row index
        if row_index == num_rows - 1 {
            going_down = false;
        } else if row_index == 0 {
            going_down = true;
        }

        // Update the row index based on the direction
        if going_down {
            row_index += 1;
        } else {
            row_index -= 1;
        }
    }

    // Convert Vec<u8> to String
    let mut result = String::with_capacity(len);
    for row in rows {
        // SAFETY: We know all bytes are valid UTF-8 since they came from a String
        unsafe {
            // Return the final string with from_utf8_unchecked (Opt 5.)
            result.push_str(std::str::from_utf8_unchecked(&row));
        }
    }

    result
}
