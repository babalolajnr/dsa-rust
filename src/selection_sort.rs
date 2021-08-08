/**
    Selection sort is a sorting algorithm that selects the smallest element from
    an unsorted list in each iteration and places that element at the beginning
    of the unsorted list.

    Working of Selection Sort

    1. Set the first element as minimum. 
    2. Compare minimum with the second element. If the second element is smaller than minimum,
       assign the second element as minimum.

       Compare minimum with the third element. Again, if the third element is smaller,
       then assign minimum to the third element otherwise do nothing. The process goes
       on until the last element. 
    3. After each iteration, minimum is placed in the front of the unsorted list. 
    4. For each iteration, indexing starts from the first unsorted element.
       Step 1 to 3 are repeated until all the elements are placed at their correct positions.
*/
pub fn selection_sort(arr: &mut [isize]) -> Vec<isize> {
    for step in 0..arr.len() {
        let mut min_idx = step;
        for i in step + 1..arr.len() {
            //Select the minimum element in the loop
            if arr[i] < arr[min_idx] {
                min_idx = i;
            }
        }

        let temp = arr[step];
        arr[step] = arr[min_idx];
        arr[min_idx] = temp;
    }
    arr.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = selection_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn test_array_is_sorted() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let original = arr.to_owned();
        let sorted_arr = selection_sort(&mut arr);
        assert_ne!(original, sorted_arr);
    }
}
