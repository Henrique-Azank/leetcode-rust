/*
    Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.

    The algorithm for myAtoi(string s) is as follows:

    1. Whitespace: Ignore any leading whitespace (" ").
    2. Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
    3. Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the 
                   string is reached. If no digits were read, then the result is 0.
    4. Rounding: If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then round the integer to remain
                 in the range. Specifically, integers less than -2^31 should be rounded to -2^31, and integers greater than 2^31 - 1
                 should be rounded to 2^31 - 1.
    5. Return the integer as the final result.
*/


/*
    Initial naive implementation of the myAtoi function.
*/
pub fn  my_atoi(s: String) -> i32 {

    // Initially trim the string
    let s: &str = s.trim();

    // Instantiate a variable to hold the result
    let mut result: String = String::new();

    // Enumeration for the type of string
    #[derive(PartialEq)]
    enum ResultType {
        Undefined,
        Positive,
        Negative,
    }

    // Initialize the string type as ResultType::Undefined
    let mut string_type = ResultType::Undefined;

    // Loop through the characters in the string as u8 bytes
    'main_loop: for (i, ch) in s.char_indices() {

        // Create a match statement to determine the type of string
        match ch {
            ' ' => {
                // If the string type is still undefined, continue to ignore whitespace
                if string_type == ResultType::Undefined {
                    continue;
                } else {
                    break 'main_loop; // If we have already defined a sign, break on whitespace
                }
            }, // Ignore whitespace
            '+' => {
                if string_type == ResultType::Undefined {
                    string_type = ResultType::Positive;
                } else {
                    break 'main_loop;
                }
            },
            '-' => {
                if string_type == ResultType::Undefined {
                    // Define the number as negative
                    string_type = ResultType::Negative;
                } else {
                    break 'main_loop;
                }
            },
            '0' => {
                if string_type == ResultType::Undefined {
                    // Treat leading zeros as positive without sign
                    string_type = ResultType::Positive; 
                }

                // Case where + or - is followed by only leading zeros and a whitespace or non-digit character
                if string_type != ResultType::Undefined && s.chars().nth(i + 1).map(|c| c.is_whitespace()).unwrap_or(false) {
                    break 'main_loop;
                }

                // Check if the result is empty, if so, continue to skip leading zeros
                if result.is_empty() {
                    continue;
                }

                // If we have already read some digits, treat '0' as a valid digit
                result.push(ch);
            },
            '1'..='9' => {
                if string_type == ResultType::Undefined {
                    string_type = ResultType::Positive;
                }

                // Push the digit to the result vector
                result.push(ch);
            },
            // Non-digit character encountered
            _ => break 'main_loop,
            
        }
    };


    // Function to parse the string into an i32, clamping it to the valid range
    fn parse_i32_clamped(s: &str, num_type: ResultType) -> i32 {

        // If the string is empty, return 0
        if s.is_empty() {
            return 0;
        }

        // Try to perform the parsing
        match s.parse::<i32>() {
            Ok(n) => {
                return match num_type {
                    ResultType::Positive => n,
                    ResultType::Negative => -n,
                    _ => 0,
                };
            },  // Happy path - no overflow
            Err(_e) => {
                // Handle overflow based on the type of number
                return match num_type {
                    ResultType::Positive => i32::MAX,  // Return max for positive overflow
                    ResultType::Negative => i32::MIN,  // Return min for negative overflow
                    _ => 0,         // Undefined case, return 0
                };
            },
        }
    }

    // Based on the string type, convert the result to an integer
    let final_result: i32 = parse_i32_clamped(&result, string_type);

    // Clamp the final result to the 32-bit signed integer range
    return final_result;

}
    

// TODO: Implement unit tests for the my_atoi function
// // Examples list
// let examples: Vec<String> = vec![
//     "   -42".to_string(),
//     "4193 with words".to_string(),
//     "words and 987".to_string(),
//     "-91283472332".to_string(),
//     "91283472332".to_string(),
//     "0-1".to_string(),
//     "   +0 123".to_string(),
//     "    -88827   5655  U".to_string(),
// ];

// for input in examples {
//     println!("Input: {}", input);
//     let result = my_atoi(input);
//     println!("Output: {}", result);
// }
