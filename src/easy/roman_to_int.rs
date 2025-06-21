/*
    Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

    Roman numerals are usually written largest to smallest from left to right. 
    However, the numeral for four is not IIII. Instead, the number four is written as IV. 
    Because the one is before the five we subtract it making four. The same principle applies to the number nine,
    which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9. 
    X can be placed before L (50) and C (100) to make 40 and 90. 
    C can be placed before D (500) and M (1000) to make 400 and 900.

    Given a roman numeral, convert it to an integer.
*/

/*
    Initial iterative solution
*/
pub fn roman_to_int(s: String) -> i32 {
    // Define a mapping of Roman numeral symbols to their integer values
    let roman_map: std::collections::HashMap<char, i32> = [
        ('I', 1), ('V', 5), ('X', 10), ('L', 50),
        ('C', 100), ('D', 500), ('M', 1000)
    ].iter().cloned().collect();

    // Instantiate a variable to hold the total value
    let mut total = 0;

    // Instantiate a variable to hold the previous value
    let mut prev_value = 0;

    // Iterate through each character in the string
    for c in s.chars() {
        // Get the integer value of the current Roman numeral character
        let value = roman_map.get(&c).unwrap_or(&0);

        // If the previous value is less than the current value, it means we have a subtractive combination
        if prev_value < *value {
            total += value - 2 * prev_value; // Adjust total by subtracting twice the previous value
        } else {
            total += value; // Otherwise, just add the current value
        }

        // Update the previous value to the current one for the next iteration
        prev_value = *value;
    }

    // Return the total value
    total

}




