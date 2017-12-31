// Copyright © 2017 Bart Massey
// From O'Reilly Rust book, but completed.

//! Quicksort with Hoare partitioning.

#[cfg(test)]
extern crate rand;

/// Rearrange the elements of `slice`. Returns a "pivot"
/// index into the slice.  On return, all elements at
/// indices less than or equal to the pivot index will be
/// less than or equal to the value at the pivot index, and
/// all elements at indices greater than or equal to the
/// pivot index will be greater than or equal to the value
/// at the pivot index.
///
/// Partitioning is done using a custom variant of Hoare's method
/// designed to put the pivot reasonably close to the middle.
///
/// # Examples
///
/// ```
/// let mut a = [5,1,0,2,2,4,3,2];
/// let pivot = quicksort::partition(&mut a);
/// for (i, v) in a.into_iter().enumerate() {
///     if i <= pivot {
///         assert!(*v <= a[pivot])
///     } else {
///         assert!(*v > a[pivot])
///     }
/// }
/// ```
pub fn partition<T: Ord>(slice: &mut [T]) -> usize {
    // Set up the length.
    let n = slice.len();
    if n < 2 {
        panic!("partition of short slice")
    }

    // Things are easier if we order the first considered
    // elements.
    if slice[0] > slice[n-1] {
        slice.swap(0, n-1)
    }

    // Set up the state.
    let mut low = 0;
    let mut high = n - 1;
    let mut low_max = low;
    let mut high_min = high;
    let mut nlow = 1;
    let mut nhigh = 1;

    // Partition the rest of the values.
    loop {
        // Check invariants.
        assert!(slice[low_max] <= slice[high_min]);
        for i in 0..low+1 {
            assert!(slice[i] <= slice[low_max])
        }
        for i in high..n {
            assert!(slice[i] >= slice[high_min])
        }

        // Get some target values to look at.
        low += 1;
        if low < high {
            high -= 1;
        }

        // If we're (almost) done, clean up and return.
        if low >= high {
            // Make sure nothing has been skipped.
            assert!(low == high);

            // Note that high and low are now equal:
            // use both names for "clarity".

            // Make sure the hypothetical last value
            // isn't special.
            if slice[low] < slice[low_max] {
                slice.swap(low, low_max);
            }
            if slice[high] > slice[high_min] {
                slice.swap(high, high_min);
            }

            // Now we are at the pivot. Let's make
            // a third name for this index.
            let pivot = low;

            // Check the invariants one last time.
            for (i, v) in slice.iter().enumerate() {
                if i <= pivot {
                    assert!(*v <= slice[pivot])
                } else {
                    assert!(*v >= slice[pivot])
                }
            }

            // We're done.
            return pivot
        }

        // Put both outstanding values in the low partition.
        let mut place_low = move |slice: &mut[T]| {
            // Move the high value to the low end if needed.
            if low + 1 < high {
                slice.swap(low + 1, high)
            }
            // Update low_max as needed.
            if slice[low] > slice[low_max] {
                low_max = low
            }
            if slice[low + 1] > slice[low_max] {
                low_max = low + 1
            }
            // Adjust the indices to reflect what happpened.
            low += 1;
            high += 1;
            nlow += 2
        };

        let mut place_high = move |slice: &mut [T]| {
            // Move the low value to the high end if needed.
            if low + 1 < high {
                slice.swap(low, high - 1)
            }
            // Update high_min as needed.
            if slice[high] < slice[high_min] {
                high_min = high
            }
            if slice[high - 1] < slice[high] {
                high_min = high - 1
            }
            // Adjust the indices to reflect what happpened.
            low -= 1;
            high -= 1;
            nhigh += 2
        };

        // Ok, now re-establish the invariants. This is a
        // long walk.

        // Case: We are forced to place both values low.
        if slice[low] < slice[low_max] && slice[low+1] < slice[low_max] {
            place_low(slice);
            continue
        }

        // Case: We are forced to place both values high.
        if slice[low] > slice[high_min] && slice[high - 1] > slice[high_min] {
            place_high(slice);
            continue
        }

        // Case: We are out of balance high, so place both values low.
        if nlow + 1 < nhigh {
            place_low(slice);
            continue
        }

        // Case: We are out of balance low, so place both values high.
        if nlow + 1 < nhigh {
            place_high(slice);
            continue
        }

        // Case: we are in-balance and have the option, so split
        // the values.

        // Need the low value first.
        if slice[low] > slice[high] {
            slice.swap(low, high)
        }
        // Adjust the counts.
        nlow += 1;
        nhigh += 1
    }
}


#[test]
fn partition_random() {
    use rand::Rng;
    let n = rand::thread_rng().gen_range(100, 1000);
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(rand::thread_rng().gen_range(-50, 50))
    }
    let pivot = partition(&mut a);
    let pivot_val = a[pivot];
    for (i, v) in a.into_iter().enumerate() {
        if i <= pivot {
            assert!(v <= pivot_val)
        } else {
            assert!(v >= pivot_val)
        }
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
