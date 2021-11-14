/**
    Bubble sort is a sorting algorithm that compares two adjacent elements and
    swaps them if they are not in the intended order.

    Working of Bubble Sort

    Suppose we are trying to sort the elements in ascending order.

    1. First Iteration (Compare and Swap)

        1. Starting from the first index, compare the first and the second elements.
        2. If the first element is greater than the second element, they are swapped.
        3. Now, compare the second and the third elements. Swap them if they are not in order.
        4. The above process goes on until the last element.

    2. Remaining Iteration

    The same process goes on for the remaining iterations.

    After each iteration, the largest element among the unsorted elements is placed at the end.



*/
pub mod bubble_sort {

    /** Time Complexity
     Best    O(n),
     Worst   O(n2),
     Average O(n2)

    Space Complexity O(1)
    */
    pub fn bubble_sort<T: Ord + Clone>(arr: &mut [T]) -> Vec<T> {
        for i in 0..arr.len() {
            for j in 0..arr.len() - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
        arr.to_owned()
    }

    /**
    Doesn't sort if it is already sorted

    Time Complexity
     Best    O(n)
     Worst   O(n2)
     Average O(n2)

    Space Complexity O(1)
    */
    pub fn optimized_bubble_sort<T: Ord + Clone>(arr: &mut [T]) -> Vec<T> {
        for i in 0..arr.len() {
            // Check if swapping occurs
            let mut swapped = false;
            for j in 0..arr.len() - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    swapped = true;
                }
            }
            // No swapping means elements are already sorted
            if !swapped {
                break;
            }
        }
        arr.to_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = bubble_sort::bubble_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn test_optimized_bubble_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = bubble_sort::optimized_bubble_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort::bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort::bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
