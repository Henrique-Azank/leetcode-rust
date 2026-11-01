/*
    Given an integer array nums of n integers, return an array of 
    all the unique quadruplets: 

    [nums[a], nums[b], nums[c], nums[d]], such that: 

    0 <= a, b, c, d < n
    a, b, c, and d are distinct.
    nums[a] + nums[b] + nums[c] + nums[d] == target.
*/

// Standard library imports
use std::collections::{HashMap, HashSet};

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

/*
    Sorting + Two Pointers (O(n^3) Time, O(1) Auxiliary Space)

    Sort the array first, fix the first two elements with nested loops (i and j),
    and use two pointers (left and right) to find the remaining two elements.

    Same approach as the 3 sum but with another loop
*/
pub fn four_sum_two_pointers(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    // Get the size of the array 
    let n = nums.len();

    // Initialize the result vector
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Early return if there are fewer than four elements
    if n < 4 {
        return result;
    }

    // Unstable sorting -> Faster than stable sorting
    nums.sort_unstable();

    // Another way to prevent integer overflow edge cases
    let target = target as i64;

    // Iterate through the array
    for i in 0..n - 3 {

        // Skip duplicate values for the first element
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // Iterate through the array for the second element
        for j in (i + 1)..n - 2 {

            // Skip duplicate values for the second element
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            // Initialize the left and right pointers
            let mut left = j + 1;
            let mut right = n - 1;

            // Iterate while the pointers do not cross
            while left < right {

                // Calculate the sum of the four elements (Refrain from integer overflow)
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                // Compare the sum with the target and move the pointers
                if sum == target {
                    result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                    // Skip duplicates for left and right pointers
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }

    result
}

/*
    Sorting + Two Pointers with Early Pruning (O(n^3) Worst-Case, much faster in practice)

    Uses the sorted property to bound the minimum and maximum possible sums at each
    outer loop iteration, breaking or skipping early when a solution is impossible.
*/
pub fn four_sum_pruned(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    // Get the size of the array
    let n = nums.len();

    // Initialize the result vector
    let mut result: Vec<Vec<i32>> = Vec::new();

    // Early return if there are fewer than four elements
    if n < 4 {
        return result;
    }

    // Unstable sorting -> Faster than stable sorting
    nums.sort_unstable();

    // Convert target to i64 to prevent integer overflow edge cases
    let target = target as i64;

    // Iterate through the array
    for i in 0..n - 3 {

        // Skip duplicates for the first element
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // Smallest possible sum with nums[i] is already greater than target -> no further i can work
        let min_i = nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64;
        if min_i > target {
            break;
        }

        // Largest possible sum with nums[i] is still smaller than target -> skip to next i
        let max_i = nums[i] as i64 + nums[n - 3] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
        if max_i < target {
            continue;
        }

        // Iterate through the array for the second element
        for j in (i + 1)..n - 2 {

            // Skip duplicates for the second element
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            // Smallest possible sum with (nums[i], nums[j]) > target -> break out of j loop
            let min_j = nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64;
            if min_j > target {
                break;
            }

            // Largest possible sum with (nums[i], nums[j]) < target -> continue to next j
            let max_j = nums[i] as i64 + nums[j] as i64 + nums[n - 2] as i64 + nums[n - 1] as i64;
            if max_j < target {
                continue;
            }

            // Initialize the left and right pointers
            let mut left = j + 1;
            let mut right = n - 1;

            while left < right {

                // Calculate the sum of the four elements (Refrain from integer overflow)
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                // Compare the sum with the target and move the pointers
                if sum == target {
                    result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }

    result
}

/*
    Hash Map of Pair Sums (O(n^2) Average Time, O(n^2) Space)

    Rewrites a + b + c + d == target as (a + b) == target - (c + d).
    Caps each unique number to at most 4 occurrences first so duplicate-heavy inputs
    do not degrade pair-bucket traversal to O(n^4).
*/
pub fn four_sum_hash_map(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    
    // Sort the array for efficient duplicate filtering
    nums.sort_unstable();

    // At most 4 identical numbers can ever participate in a quadruplet
    let mut filtered: Vec<i32> = Vec::with_capacity(nums.len());
    for num in nums {
        let len = filtered.len();
        if len < 4 || filtered[len - 4] != num {
            filtered.push(num);
        }
    }

    let n = filtered.len();
    if n < 4 {
        return Vec::new();
    }

    let target = target as i64;
    // Map from pair sum (nums[a] + nums[b]) -> list of index pairs (a, b) with a < b
    let mut pair_sums: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();
    let mut unique_quads: HashSet<[i32; 4]> = HashSet::new();

    // Iterate over the split point: add pairs (a, b) with b = c - 1 before querying pairs (c, d)
    // This guarantees a < b < c < d without needing manual index overlap checks.
    for c in 2..n - 1 {
        let b = c - 1;
        for a in 0..b {
            let sum_ab = filtered[a] as i64 + filtered[b] as i64;
            pair_sums.entry(sum_ab).or_default().push((a, b));
        }

        for d in (c + 1)..n {
            let sum_cd = filtered[c] as i64 + filtered[d] as i64;
            let complement = target - sum_cd;

            if let Some(pairs) = pair_sums.get(&complement) {
                for &(a, b) in pairs {
                    unique_quads.insert([filtered[a], filtered[b], filtered[c], filtered[d]]);
                }
            }
        }
    }

    unique_quads.into_iter().map(|q| q.to_vec()).collect()
}

/*
    Generalized Recursive kSum (O(n^(k-1)) Time, O(k) Stack Space)

    Reduces kSum to (k - 1)Sum recursively until reaching the base case k == 2,
    which is solved using a standard two-pointer scan.
*/
pub fn four_sum_k_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    k_sum(&nums, target as i64, 4)
}




fn k_sum(nums: &[i32], target: i64, k: usize) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut result: Vec<Vec<i32>> = Vec::new();
    if n < k || k < 2 {
        return result;
    }

    // Early pruning using average value bounds:
    // If the smallest element * k > target, or largest element * k < target, no solution exists
    let min_val = nums[0] as i64;
    let max_val = nums[n - 1] as i64;
    if min_val * (k as i64) > target || max_val * (k as i64) < target {
        return result;
    }

    if k == 2 {
        return two_sum(nums, target);
    }

    for i in 0..=n - k {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let current = nums[i];
        for mut sub in k_sum(&nums[i + 1..], target - current as i64, k - 1) {
            let mut quad = Vec::with_capacity(k);
            quad.push(current);
            quad.append(&mut sub);
            result.push(quad);
        }
    }

    result
}

fn two_sum(nums: &[i32], target: i64) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] as i64 + nums[right] as i64;
        if sum == target {
            result.push(vec![nums[left], nums[right]]);

            while left < right && nums[left] == nums[left + 1] {
                left += 1;
            }
            while left < right && nums[right] == nums[right - 1] {
                right -= 1;
            }

            left += 1;
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    result
}



