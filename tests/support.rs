macro_rules! iter_works_for {
    ($subject:ident) => {{
        let taps = vec![2, 3];
        let mut lfsr = $subject::from_iter(taps.iter());

        let iter = lfsr.iter();

        let output: Vec<bool> = iter.take(8).collect();

        let expected = vec![true, false, false, true, false, true, true, true];

        assert!($crate::support::eq_mod_rotation(&output, &expected));
    }}
}

macro_rules! primitive_connection_polynomial_yields_a_maximum_sequence {
    ($subject:ident) => {{
        let taps = vec![4, 5, 6, 8];

        let mut lfsr = $subject::from_iter(taps.iter());

        let output: Vec<bool> = lfsr
            .iter()
            .take(255)
            .collect();

        for i in 1 .. 255 {
            assert!(!$crate::support::has_period(&output, i));
        }
    }}
}

pub fn eq_mod_rotation<T: PartialEq>(left: &Vec<T>, right: &Vec<T>) -> bool {
    if left.len() != right.len() { return false }

    let len = left.len();

    for offset in 0 .. len {
        let cond = |i: usize| left[i] == right[(i + offset) % len];
        let equal = (0 .. len).all(cond);

        if equal { return true }
    }

    false
}

pub fn has_period<T: PartialEq>(seq: &Vec<T>, n: usize) -> bool {
    let m = seq.len();

    (0 .. m).all(|i| seq[i] == seq[(i + n) % m])
}

#[cfg(test)]
mod test {
    use super::{eq_mod_rotation, has_period};

    #[test]
    fn eq_mod_rotation_works() {
        let v0: Vec<usize> = vec![1, 2, 3, 4];
        let v1: Vec<usize> = vec![2, 3, 4, 1];
        let v2: Vec<usize> = vec![3, 4, 1, 2];
        let v3: Vec<usize> = vec![4, 1, 2, 3];

        let w: Vec<usize> = vec![4, 3, 2, 1];

        assert!(eq_mod_rotation(&v0, &v0));
        assert!(eq_mod_rotation(&v0, &v1));
        assert!(eq_mod_rotation(&v0, &v2));
        assert!(eq_mod_rotation(&v0, &v3));

        assert!(eq_mod_rotation(&v1, &v1));
        assert!(eq_mod_rotation(&v1, &v2));
        assert!(eq_mod_rotation(&v1, &v3));

        assert!(eq_mod_rotation(&v2, &v2));
        assert!(eq_mod_rotation(&v2, &v3));

        assert!(eq_mod_rotation(&v3, &v3));

        assert!(!eq_mod_rotation(&v0, &w));
        assert!(!eq_mod_rotation(&v1, &w));
        assert!(!eq_mod_rotation(&v2, &w));
        assert!(!eq_mod_rotation(&v3, &w));
    }

    #[test]
    fn has_period_works() {
        let v_period_2 = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let v_period_3 = vec![0, 1, 2, 0, 1, 2];

        let v_no_period = vec![0, 1, 0, 1, 0, 0];

        assert!(has_period(&v_period_2, 2));
        assert!(has_period(&v_period_3, 3));

        for i in 1 .. v_no_period.len() {
            assert!(!has_period(&v_no_period, i));
        }
    }
}
