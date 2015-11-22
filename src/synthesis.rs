use std::slice::Iter;
use bit_vec::BitVec;
use bin_poly::BinPoly;

pub fn berlekamp_massey(iter: Iter<bool>) -> Vec<usize> {
    let seq: BitVec = iter.cloned().collect();

    let mut conn = BinPoly::from_coeff([true].iter());
    let mut prev = BinPoly::from_coeff([true].iter());
    let mut m = 0;
    let mut lc = 0;

    for n in 0 .. seq.len() {
        let d = discrepancy(n, lc, &conn, &seq);

        if d {
            let temp = conn.clone();

            let mut prev0 = prev.clone();
            prev0.shift((n - m) + 1);

            conn.add(&prev0);

            if lc <= n / 2 {
                lc = n + 1 - lc;
                m = n + 1;
                prev = temp.clone();
            }
        }
    }

    conn.into_taps()
}

fn discrepancy(n: usize, lc: usize, conn: &BinPoly, seq: &BitVec) -> bool {
    (0 .. lc + 1)
        .map(|i| conn.coeff(i) && seq[n - i])
        .fold(false, |acc, x| acc ^ x)
}
