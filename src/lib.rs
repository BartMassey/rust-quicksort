// Copyright © 2017 Bart Massey
// From O'Reilly Rust book, but completed.

//! Quicksort with Hoare partitioning.

/// Rearrange the elements of `slice`. Returns a "pivot"
/// index into the slice.  On return, all elements at indices less than or
/// equal to the pivot index will be less than or equal to
/// the value at the pivot index, and all elements at
/// indices greater than the pivot index will be greater
/// than the value at the pivot index.
///
/// Partitioning is done using Hoare's Method 
/// <http://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme>.
pub fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let n = slice.len();
    if n < 2 {
        panic!("partition of short slice")
    }

    // Median-of-three pivot.
    let mut pivot = 0;
    if slice[pivot] > slice[n / 2] {
        pivot = n / 2;
    }
    if slice[pivot] < slice[n - 1] {
        pivot = n - 1;
    }
    // Put the pivot in the middle.
    let p = n / 2;
    slice.swap(pivot, p);

    // Pointers for lesser stack (grows up) and greater
    // stack (grows down).
    let mut lesser = 1;
    let mut greater = n - 1;

    // Exchange as needed to ensure crossover.
    loop {
        while lesser <= greater && slice[lesser] <= slice[0] {
            lesser += 1
        }
        while lesser <= greater && slice[greater] > slice[0] {
            greater -= 1
        }
        if lesser >= greater {
            slice.swap(greater, 0);
            return greater
        }
        slice.swap(lesser, greater)
    }
}

/// Sorts the elements of the slice using Quicksort with
/// Hoare Partitioning.
///
/// # Examples
///
/// ```
/// let mut a = [5,1,0,4,3,2];
/// quicksort::quicksort(&mut a);
/// for (i, v) in a.into_iter().enumerate() {
///     assert_eq!(i, *v)
/// }
/// ```
pub fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;  // Nothing to sort.
    }

    // Partition the slice into two parts, front and back.
    let pivot_index = partition(slice);

    // Recursively sort the front half of `slice`.
    quicksort(&mut slice[.. pivot_index]);

    // And the back half.
    quicksort(&mut slice[pivot_index + 1 ..]);
}

#[test]
fn quicksort_string() {
    let mut a: Vec<char> = "heabfdcg".chars().collect();
    quicksort(&mut a);
    assert_eq!(a.into_iter().collect::<String>(), "abcdefgh")
}

#[cfg(test)]
extern crate rand;

#[test]
fn quicksort_random() {
    use rand::Rng;
    let n = rand::thread_rng().gen_range(100, 1000);
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(rand::thread_rng().gen_range(0, 1000))
    }
    quicksort(&mut a);
    for i in 1..n {
        assert!(a[i-1] <= a[i])
    }
}
