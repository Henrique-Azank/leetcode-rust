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
}
