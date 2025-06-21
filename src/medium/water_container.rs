/*
    You are given an integer array height of length n. 
    There are n vertical lines drawn such that the two endpoints
    of the ith line are (i, 0) and (i, height[i]).

    Find two lines that together with the x-axis form a container, 
    such that the container contains the most water.
    Return the maximum amount of water a container can store.
*/

/*
    Initial naive approach
*/
pub fn max_area(heights: Vec<i32>) -> i32 {
    // Instantiate the loop variables
    let mut left = 0;
    let mut right = heights.len() - 1;
    let mut max_area = 0;

    // While the indexes do not cross
    while left < right {

        // Calculate the current area
        let width: i32 = (right - left) as i32;
        let h: i32 = heights[left].min(heights[right]);

        // Update the maximum area if the current area is larger
        max_area = max_area.max(width * h);

        // Move the pointer of the shorter line inward
        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    // Return the max area
    max_area
}



