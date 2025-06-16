/*
    Given two sorted arrays nums1 and nums2
    of size m and n respectively, return the median
    of the two sorted arrays.

    The overall run time complexity should be O(log (m+n))
*/

/*
    Base Idea:
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
