fn main() {}

fn bubble_sort(arr: &mut [isize]) -> Vec<isize> {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    arr.to_owned()
}

fn optimized_bubble_sort(arr: &mut [isize]) -> Vec<isize> {
    for i in 0..arr.len() {
        // Check if swapping occurs
        let mut swapped = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = bubble_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn test_optimized_bubble_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = optimized_bubble_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }
}
