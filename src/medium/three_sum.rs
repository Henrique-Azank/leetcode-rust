/*
    Given an integer array nums, 
    return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, 
    and nums[i] + nums[j] + nums[k] == 0.

    Notice that the solution set must not contain duplicate triplets.
*/


/*
    Initial naive approach (Brute Force):

    Loop over the array and check for each triplet 
    every time - essentially O(n^3) complexity
*/
pub fn three_sum_loop(nums: Vec<i32>) -> Vec<Vec<i32>> {

    // Instantiate the result vector
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Loop over the provided array
    for (i, &num_i) in nums.iter().enumerate() {

        // If we are out of bounds, continue
        if i >= nums.len() - 2 {
            continue;
        };

        // Loop over the rest of the array
        for (j, &num_j) in nums.iter().enumerate().skip(i + 1) {

            // Enumerate the rest of the array
            for (_k, &num_k) in nums.iter().enumerate().skip(j + 1) {

                // Check if the triplet sums to zero
                if num_i + num_j + num_k == 0 {
                    // Create a triplet vector
                    let mut triplet = vec![num_i, num_j, num_k];
                    triplet.sort();

                    // Check if the triplet is already in the result
                    if !result.contains(&triplet) {
                        result.push(triplet);
                    }
                }


            };
        };
    }

    // Return the result vector
    result
}




