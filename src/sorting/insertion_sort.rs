fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();

    for j in 1..n {
        let key = arr[j];
        // The definition of `i` is differert from CLRS. Because `i` will be usize, we can't make `i` less than 0 in the `while` loop.
        let mut i = j;

        while i > 0 && arr[i - 1] > key {
            arr[i] = arr[i - 1];
            i -= 1;
        }

        arr[i] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
