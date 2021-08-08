fn main() {
    let mut arr: Vec<i32> = vec![8, 1, 5, 7, 2, 3, 6, 4];
    bubble_sort(&mut arr);
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    println!("{:?}", arr);
}
