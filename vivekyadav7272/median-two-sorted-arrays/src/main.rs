pub fn median_two_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> f64 {
    let smaller = if arr1.len() < arr2.len() {arr1} else {arr2};
    let bigger = if std::ptr::eq(smaller, arr1) {arr2} else {arr1};
    
    let mut start = 0usize;
    let mut end = smaller.len();

    loop {
        let smallerPartition = (start + end)/2;
        let biggerPartition = (arr1.len() + arr2.len() + 1)/2 - smallerPartition;
        
        let smallerLeft = if smallerPartition as i32 - 1 < 0 {f64::NEG_INFINITY} else {smaller[smallerPartition-1] as f64};
        let biggerLeft = if biggerPartition as i32 - 1 < 0 {f64::NEG_INFINITY} else {bigger[biggerPartition-1] as f64};

        let smallerRight = if smallerPartition >= smaller.len() {f64::INFINITY} else {smaller[smallerPartition] as f64};
        let biggerRight = if biggerPartition >= bigger.len() {f64::INFINITY} else {bigger[biggerPartition] as f64};

        if smallerLeft <= biggerRight && biggerLeft <= smallerRight {
            // Found correct partition.
            if (arr1.len() + arr2.len()) % 2 == 0 {
                return (smallerLeft.max(biggerLeft) + smallerRight.max(biggerRight))/2.0f64;
            }

            return smallerLeft.max(biggerLeft);  // The left partition is always the bigger one, meaning it contains the median.
        } else if smallerLeft > biggerRight {
            end = smallerPartition;
        } else {
            start = smallerPartition + 1;
        }
    }

    f64::INFINITY
}


fn main() {
    dbg!(median_two_sorted_arrays(&[1,3], &[2]));
    dbg!(median_two_sorted_arrays(&[1,2], &[3,4]));
}
