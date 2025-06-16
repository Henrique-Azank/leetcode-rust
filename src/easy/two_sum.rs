// Require
use std::collections::HashMap;

// Solution function
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    /*
     Instantiate the map between the required value
     to reach the target and the index of the complement
    */
    let mut solve_hash: HashMap<i32, i32> = HashMap::new();

    // Loop over the provided array and search for the solution
    for (index, value) in nums.iter().enumerate() {
        // Convert the index as i32 using idiomatic "as"
        let index_32: i32 = index as i32;

        // Check if the solve_hash already has the complement
        let has_compl: bool = solve_hash.contains_key(value);

        // If the value has a complement, return the values indexes
        if has_compl {
            // Get the compement value
            let compl_index = solve_hash.get(value).unwrap();
            return vec![*compl_index, index_32];
        }

        // Else, add the key as the complement
        let complement = target - value;
        solve_hash.insert(complement, index_32);
    }

    // Base return
    return Vec::new();
}
