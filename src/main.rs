use leetcode_solutions::medium::zigzag_conversion::optimized_zigzag_conversion;

fn main() {
    // Get the string to convert
    let s: String = "ABC".to_string();

    // Get the number of rows
    let num_rows: i32 = 1;

    // Call the naive zigzag conversion function
    let result: String = optimized_zigzag_conversion(s, num_rows);

    // Example usage of the brute_longest_palindrome function
    println!("{:?}", result);
}
