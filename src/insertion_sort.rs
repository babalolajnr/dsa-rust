pub fn insertion_sort<T: PartialEq + PartialOrd + Clone>(arr: &mut [T]) -> Vec<T> {

    // The resulting vector should contain the same amount of elements as
    // the slice that is being sorted, so enough room is preallocated
    let mut result: Vec<T> = Vec::with_capacity(arr.len());

    // Iterate over the elements to sort and
    // put a clone of the element to insert in elem.
    for elem in arr.iter().cloned() {
        // How many elements have already been inserted?
        let n_inserted = result.len();

        // Loop over the inserted elements and one more index.
        for i in 0..=n_inserted {
            // If at the end or result[i] is larger than the current element,
            // we have found the right spot:
            if i == n_inserted || result[i] > elem {
                // Insert the element at i,
                // move the rest to higher indexes:
                result.insert(i, elem);
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let sorted_arr = insertion_sort(&mut arr);
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }

    #[test]
    fn test_array_is_sorted() {
        let mut arr = vec![8, 1, -8, 5, 0, 7, -2, 2, -6, -1, 3, -3, -4, -7, 6, -5, 4];
        let original = arr.to_owned();
        let sorted_arr = insertion_sort(&mut arr);
        assert_ne!(original, sorted_arr);
    }
}
