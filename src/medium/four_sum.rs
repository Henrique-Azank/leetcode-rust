/*
    Given an integer array nums of n integers, return an array of 
    all the unique quadruplets: 

    [nums[a], nums[b], nums[c], nums[d]], such that: 

    0 <= a, b, c, d < n
    a, b, c, and d are distinct.
    nums[a] + nums[b] + nums[c] + nums[d] == target.
*/

/* 
    Initial naive approach (Brute Force)

    Loop over the array and check every possible quadruplet.
    Essentially a O(n^4) approach.
*/
pub fn four_sum_loop(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    // Instantiate the result vector
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Loop over the provided array
    for (i, &num_i) in nums.iter().enumerate() {

        // If we are out of bounds, continue
        if i >= nums.len() - 3 {
            continue;
        };

        // Loop over the rest of the array
        for (j, &num_j) in nums.iter().enumerate().skip(i + 1) {

            // Enumerate the rest of the array
            for (k, &num_k) in nums.iter().enumerate().skip(j + 1) {

                // Enumerate the rest of the array
                for (_l, &num_l) in nums.iter().enumerate().skip(k + 1) {

                    // Check for the overflow trap
                    if let Some(sum) = num_i
                        .checked_add(num_j)
                        .and_then(|s| s.checked_add(num_k))
                        .and_then(|s| s.checked_add(num_l))
                    {
                        // Check if the quadruplet sums to the target
                        if sum == target {
                            // Create a quadruplet vector
                            let mut quadruplet = vec![num_i, num_j, num_k, num_l];
                            quadruplet.sort();
    
                            // Check if the quadruplet is already in the result
                            if !result.contains(&quadruplet) {
                                result.push(quadruplet);
                            }
                        }
                    }
                }
            };
        };
    }

    // Return the result vector
    result
}


