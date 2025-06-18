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
