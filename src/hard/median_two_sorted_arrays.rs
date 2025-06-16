/*
    Given two sorted arrays nums1 and nums2
    of size m and n respectively, return the median
    of the two sorted arrays.

    The overall run time complexity should be O(log (m+n))
*/

/*
    Naive Approach:
    1. Merge the tow arrays into a new array.
    2. Sort the array.
    3. Find the median of the new array.
*/
pub fn naive_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // Merge the two arrays
    let mut merged: Vec<i32> = [nums1, nums2].concat();

    // Sort the merged array
    merged.sort();

    // Get the size of the vector
    let size: i32 = merged.len() as i32;

    // If the size is even, return the average of the two middle elements
    if size % 2 == 0 {
        // Find the two middle elements
        let mid1: i32 = merged[(size / 2 - 1) as usize];
        let mid2: i32 = merged[(size / 2) as usize];

        // Return the average of the two middle elements
        return (mid1 + mid2) as f64 / 2.0;
    }

    // If the size is odd, return the middle element
    return merged[(size / 2) as usize] as f64;
}

/*
    Partial merge Approach:
    1. Calculate the index of the median in the merged array.
    2. Merge the sorted arrays in parts.
    3. Find the median based on the index.
*/
pub fn partial_merge_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // Size of the two arrays
    let n: usize = nums2.len();
    let m: usize = nums1.len();
    let merged_size: usize = nums1.len() + nums2.len();

    // Is odd or even
    let is_odd: bool = merged_size % 2 == 1;

    // Index to end the merge process
    let end_index: usize = merged_size / 2;

    // Instantiate the merged array
    let mut merged: Vec<i32> = vec![];

    // Current index of the arrays
    let mut i: usize = 0;
    let mut j: usize = 0;

    // Perform the merge process
    while merged.len() <= end_index {
        // If both arrays are exhausted, break
        if i >= m && j >= n {
            break;
        }

        // If the first array is exhausted, push the element from the second array
        if i >= m {
            merged.push(nums2[j]);
            j += 1;
            continue;
        }

        // If the second array is exhausted, push the element from the first array
        if j >= n {
            merged.push(nums1[i]);
            i += 1;
            continue;
        }

        // If the current element of the first array is less than the current element of the second array
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    // Return the median based on the index
    if is_odd {
        return merged[end_index] as f64;
    }

    // If the size is even, return the average of the two middle elements
    return (merged[end_index] + merged[end_index - 1]) as f64 / 2.0;
}

/*
    Binary Search Approach:
    1. Use binary search to find the partition point in the smaller array.
    2. Calculate the median based on the partition point.
    3. Return the median.
*/
pub fn binary_search_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // Get the smaller and larger array
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    // Size of the two arrays
    let m: usize = nums1.len();
    let n: usize = nums2.len();
    let total_len: usize = m + n;

    // If one of the arrays is empty, return the median of the other array
    let half_len: usize = (total_len + 1) / 2;
    let mut imin: usize = 0;
    let mut imax: usize = m;

    while imin <= imax {
        // Calculate the partition indices
        let i: usize = (imin + imax) / 2;
        let j: usize = half_len - i;

        // Check if the partition is valid
        if i < m && nums2[j - 1] > nums1[i] {
            // i is too small, increase it
            imin = i + 1;
        } else if i > 0 && nums1[i - 1] > nums2[j] {
            // i is too big, decrease it
            imax = i - 1;
        } else {
            // i is perfect
            let max_left: i32;
            if i == 0 {
                max_left = nums2[j - 1];
            } else if j == 0 {
                max_left = nums1[i - 1];
            } else {
                max_left = std::cmp::max(nums1[i - 1], nums2[j - 1]);
            }

            if total_len % 2 == 1 {
                return max_left as f64;
            }

            let min_right: i32;
            if i == m {
                min_right = nums2[j];
            } else if j == n {
                min_right = nums1[i];
            } else {
                min_right = std::cmp::min(nums1[i], nums2[j]);
            }

            return (max_left + min_right) as f64 / 2.0;
        }
    }

    panic!("Input arrays are not sorted or have invalid lengths");
}
