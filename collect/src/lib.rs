pub fn bubble_sort(arr: &mut [i32]) {
    for _ in 0..arr.len() {
        for i in 1..arr.len() {
            if arr[i] > arr[i - 1] {
                arr.swap(i, i - 1);
            }
        }
    }
}
