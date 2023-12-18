use rand::{Rng, rngs::ThreadRng};

pub fn sort<T>(elements: &mut Vec<T>, cmp: &dyn Fn(&T, &T) -> bool) {
    let mut rng = ThreadRng::default();
    let size = elements.len();
    qsort(elements, 0, size, cmp, &mut rng);
}

// Result:
// | x_0 < p | x_1 < p | ... | p | p | ... | p | x_k > p | ...
//                             ^                    ^
//                            left                right

fn partition<T>(elements: &mut [T], lo: usize, hi: usize, pivot_index: usize, cmp: &dyn Fn(&T, &T) -> bool) -> (usize, usize) {
    let num_less_than_pivot = elements[lo..hi].iter().filter(|x| cmp(*x, &elements[pivot_index])).count();
    let pivot_position = lo + num_less_than_pivot;
    elements.swap(pivot_position, pivot_index);
    let mut less_id = lo;
    let mut greater_id = pivot_position + 1;

    for i in lo..hi {
        if cmp(&elements[i], &elements[pivot_position]) {
            elements.swap(less_id, i);
            less_id += 1;
        }
    }

    for i in pivot_position + 1..hi {
        if !cmp(&elements[i], &elements[pivot_position]) && !cmp(&elements[pivot_position], &elements[i]) {
            elements.swap(greater_id, i);
            greater_id += 1;
        }
    }
    (less_id, greater_id)
}

fn qsort<T>(elements: &mut [T], lo: usize, hi: usize, cmp: &dyn Fn(&T, &T) -> bool, rng: &mut ThreadRng) {
    if hi < lo + 2 {
        return;
    }
    let mid = rng.gen_range(lo..hi);
    let (left, right) = partition(elements, lo, hi, mid, cmp);
    qsort(elements, lo, left, cmp, rng);
    qsort(elements, right, hi, cmp, rng);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp_i32(lhs: &i32, rhs: &i32) -> bool {
        lhs < rhs
    }

    #[test]
    fn check_ints() {
        let tests = &[
            ("already_sorted", vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            ("reversed", vec![4, 3, 2, 1], vec![1, 2, 3, 4]),
            ("equal", vec![4, 4, 4, 4], vec![4, 4, 4, 4]),
        ];

        for (name, original, expected) in tests {
            let mut result = original.clone();
            sort(&mut result, &cmp_i32);
            assert!(
                result.eq(expected),
                "Testcase {} failed: {:?} != {:?}",
                name,
                expected,
                result
            );
        }
    }

    #[test]
    fn big_vec() {
        for _ in 0..100 {
            let mut rng = ThreadRng::default();
            let origin: &mut Vec<i32> =
                &mut (0..1000).map(|_| rng.gen_range(0..100)).collect();
            let mut sorted = origin.clone();
            sorted.sort();
            sort(origin, &cmp_i32);
            assert!(
                sorted.eq(origin),
                "Big vec failed: {:?} != {:?}",
                sorted,
                origin
            );
        }
    }

    #[test]
    fn empty_vec() {
        let mut empty: Vec<i32> = vec![];
        sort(&mut empty, &cmp_i32);
        assert_eq!(empty, vec![]);
    }

    #[test]
    fn single_element() {
        let mut single = vec![42];
        sort(&mut single, &cmp_i32);
        assert_eq!(single, vec![42]);
    }

    #[test]
    fn already_sorted_chars() {
        let mut chars = vec!['a', 'b', 'c', 'd', 'e'];
        sort(&mut chars, &|a, b| a < b);
        assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn random_strings() {
        let mut rng = ThreadRng::default();
        let mut strings: Vec<String> = (0..100).map(|_| rng.gen::<char>().to_string()).collect();
        sort(&mut strings, &|a, b| a < b);
        assert!(strings.windows(2).all(|w| w[0] <= w[1]));
    }
}
