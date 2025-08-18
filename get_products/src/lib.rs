pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }

    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];
    let mut result = vec![1; n];

    // Build prefix product array
    for i in 1..n {
        prefix[i] = prefix[i - 1] * arr[i - 1];
    }

    // Build suffix product array
    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1] * arr[i + 1];
    }

    // Multiply prefix and suffix products
    for i in 0..n {
        result[i] = prefix[i] * suffix[i];
    }

    result
}
