/*
    Seven different symbols represent Roman numerals

    Roman numerals are formed by appending the conversions of decimal place values from highest to lowest.
    Converting a decimal place value into a Roman numeral has the following rules:

    - If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input,
    append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.

    -If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol,
    for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are
    used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).

    -Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5
    (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.

    Given an integer, convert it to a Roman numeral.
*/

/*
    Initial naive approach
    1. Create a mapping of integer values to their corresponding Roman numeral symbols.
    2. Start with the largest value and check how many times it fits into the number.
    3. Append the corresponding Roman numeral symbol to the result string.
*/
pub fn integer_to_roman(num: i32) -> String {

    // Define the mapping of values to Roman numerals
    let values: [(i32, &'static str); 13] = 
        [
            (1000, "M"),(900, "CM"),(500, "D"),(400, "CD"),
            (100, "C"),(90, "XC"),(50, "L"),(40, "XL"),
            (10, "X"),(9, "IX"),(5, "V"),(4, "IV"),(1, "I")
        ];

    // Initialize the result string
    let mut result = String::new();
    let mut remaining = num;

    // Iterate through the values in descending order
    for &(value, symbol) in &values {
        // Determine how many times the current value fits into the remaining number
        while remaining >= value {
            result.push_str(symbol); // Append the Roman numeral symbol
            remaining -= value; // Subtract the value from the remaining number
        }
    }

    result // Return the final Roman numeral string

}
    

