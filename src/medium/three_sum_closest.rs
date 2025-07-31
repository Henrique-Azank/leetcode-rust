
/*
    Given an integer array nums of length n and an integer target, 
    find three integers in nums such that the sum is closest to target.

    Return the sum of the three integers.
    You may assume that each input would have exactly one solution.
*/

/*
    Initial naive implementation: 
    Brute force approach to find the closest sum of the three integers
*/
pub fn three_sum_closest_naive(nums: Vec<i32>, target: i32) -> i32 {

    // Instantiate the closest sum to a large value
    let mut closest_sum = i32::MAX;

    // Get the length of the input array
    let n = nums.len();

    // Iterate through all combinations of three numbers
    for i in 0..n-2 {
        for j in (i + 1)..n-1 {
            for k in (j + 1)..n {

                // Calculate the sum of the three numbers
                let sum = nums[i] + nums[j] + nums[k];

                // If the absolute difference between the sum and target 
                // is smaller than the current closest sum, update closest sum
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }
            }
        }
    }

    // Return the closest sum found
    closest_sum
}

/*
    Two pointer approach to find the closest sum of three integers:
    Based on the solution to the 3Sum problem, we can adapt the two-pointer technique
    to find the closest sum to the target.
*/
pub fn three_sum_closest_two_pointer(nums: Vec<i32>, target: i32) -> i32 {

    // Sort the initial array
    let mut nums = nums;
    nums.sort();

    // Initialize the closest sum to a large value
    let mut closest_sum = i32::MAX;

    // Loop over the nums array 
    for i in 0..nums.len() {

        // Initialize two pointers 

        // (immediately after the current index) 
        let mut left: usize = i + 1;
        
        // (end of the array)
        let mut right: usize = nums.len() - 1;

        // Transverse the array with the two pointers
        while left < right {

            // Calculate the sum of the three numbers
            let sum: i32 = nums[i] + nums[left] + nums[right];

            // If the sum is equal to the target, return it immediately
            if sum == target {
                return sum;
            }

            // If the absolute difference between the sum and target 
            // is smaller than the current closest sum, update closest sum
            if (sum - target).abs() < (closest_sum - target).abs() {
                closest_sum = sum;
            }

            // If the sum is less than the target, move the left pointer to the right
            if sum < target {
                left += 1;
            // If the sum is greater than the target, move the right pointer to the left
            } else {
                right -= 1;
            }

        }

    }

    // Return the closest sum found
    closest_sum

}

