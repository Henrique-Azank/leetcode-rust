/*
    Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
    '.' Matches any single character.​​​​
    '*' Matches zero or more of the preceding element.

    The matching should cover the entire input string (not partial).
*/

/*
    Initial inplementation of the regular expression matching function.
*/
pub fn is_match(s: String, p:String) -> bool {

    // Instantiate a 2D vector for dynamic programming
    let mut dp: Vec<Vec<bool>> = vec![vec![false; p.len() + 1]; s.len() + 1];

    // Base case: empty string and empty pattern match
    dp[0][0] = true;

    // Handle patterns with '*' that can match zero characters
    for j in 1..=p.len() {
        if p.chars().nth(j - 1) == Some('*') {
            dp[0][j] = dp[0][j - 2];
        }
    }

    // Fill the dp table looking at each character in the string and pattern
    for i in 1..=s.len() {

        // For every character in the pattern
        for j in 1..=p.len() {
            if p.chars().nth(j - 1) == Some(s.chars().nth(i - 1).unwrap()) || p.chars().nth(j - 1) == Some('.') {
                dp[i][j] = dp[i - 1][j - 1];
            } else if p.chars().nth(j - 1) == Some('*') {
                dp[i][j] = dp[i][j - 2] || (dp[i - 1][j] && (p.chars().nth(j - 2) == Some(s.chars().nth(i - 1).unwrap()) || p.chars().nth(j - 2) == Some('.')));
            }
        }

    }

    // The result is found in the bottom-right corner of the dp table
    dp[s.len()][p.len()]
}


