
/*
    Given a string containing digits from 2-9 inclusive,
    return all possible letter combinations that the number could represent. 
    Return the answer in any order.
*/

// Base dependencies
use std::collections::HashMap;

/*
    Initial naive approach (Brute Force):

    Loop over the digits and map them to their corresponding letters,
    then generate all combinations of these letters.
*/
pub fn phone_letter(digits: String) -> Vec<String> {

    // Instantiate a hashmap to map digits to letters
    let letter_map: HashMap<char, Vec<char>> = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);


    // Instantiate a vector that holds the result
    let mut result: Vec<String> = Vec::new();

    // Loop over the digits as characters
    for digit in digits.chars() {

        // Get the corresponding letters for the digit
        let digit_letters: Vec<char> = letter_map.get(&digit).unwrap().clone();

        // If the result is empty, initialize it with the letters of the first digit
        if result.is_empty() {
            // Get the corresponding letters for the first digit
            result = digit_letters.iter().map(|&c| c.to_string()).collect();
            continue;
        }

        // Instantiate a temporary vector to hold the new combinations
        let mut new_combinations: Vec<String> = Vec::new();

        // Loop over the current result
        for combination in &result {
            // Loop over the letters for the current digit
            for letter in &digit_letters {

                // Create a new combination by appending the letter to the current combination
                new_combinations.push(format!("{}{}", combination, letter));

            }            
        }

        // Update the result with the new combinations
        result = new_combinations;
    }

    // Return the result vector 
    result
}




