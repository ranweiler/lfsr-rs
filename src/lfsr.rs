use bit_vec::BitVec;

pub fn shift(state: &mut BitVec) {
    let range = 1 .. state.len();

    for i in range.rev() {
        let pred = state[i - 1];
        state.set(i, pred);
    };

    state.set(0, false);
}

#[cfg(test)]
mod test {
    use bit_vec::BitVec;
    use super::shift;

    #[test]
    fn it_works() {
        let ref mut bv = BitVec::from_elem(4, true);

        shift(bv);
        assert!(bv.eq_vec(&[false, true, true, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, true, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, false]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, false]));
    }
}
