use bin_poly::BinPoly;

#[test]
fn test_from_coeff() {
    let coeff = vec![false, true, true, false, false, true, false, false];

    let poly = BinPoly::from_coeff(coeff.iter());

    assert!(poly._coeff.eq_vec(&coeff));
}

#[test]
fn test_degree() {
    let coeff = vec![false, true, true, false, false, true, false, false];
    let poly = BinPoly::from_coeff(coeff.iter());

    assert_eq!(poly.degree(), 5);
}

#[test]
fn test_coeff() {
    let coeff = vec![false, true, true, false, false, true, false, false];
    let poly = BinPoly::from_coeff(coeff.iter());

    assert_eq!(poly.coeff(0), false);
    assert_eq!(poly.coeff(1), true);
    assert_eq!(poly.coeff(2), true);
    assert_eq!(poly.coeff(3), false);
    assert_eq!(poly.coeff(4), false);
    assert_eq!(poly.coeff(5), true);
    assert_eq!(poly.coeff(6), false);
    assert_eq!(poly.coeff(7), false);
}

#[test]
fn test_coeff_bounds() {
    let coeff = vec![true, true];
    let poly = BinPoly::from_coeff(coeff.iter());

    assert_eq!(poly.coeff(0), true);
    assert_eq!(poly.coeff(1), true);

    assert_eq!(poly.coeff(2), false);
    assert_eq!(poly.coeff(3), false);
}

#[test]
fn test_add_same_degree() {
    let coeff_f = vec![true, true, false, false, true, true, false, true];
    let coeff_g = vec![true, false, true, false, true, false, true, true];

    let mut f = BinPoly::from_coeff(coeff_f.iter());
    let f_copy = f.clone();

    let g = BinPoly::from_coeff(coeff_g.iter());
    let g_copy = g.clone();

    assert!(f == f_copy);

    f.add(&g);

    assert!(f != f_copy);
    assert!(g == g_copy);

    let expected_coeff = vec![false, true, true, false, false, true, true, false];

    assert!(f._coeff.eq_vec(&expected_coeff))
}

#[test]
fn test_add_smaller_degree() {
    let coeff_f = vec![true, true, false, false, true, true, false, true];
    let coeff_g = vec![true, false, true, false, true, false, true];

    let mut f = BinPoly::from_coeff(coeff_f.iter());
    let f_copy = f.clone();

    let g = BinPoly::from_coeff(coeff_g.iter());
    let g_copy = g.clone();

    assert!(f == f_copy);

    f.add(&g);

    assert!(f != f_copy);
    assert!(g == g_copy);

    let expected_coeff = vec![false, true, true, false, false, true, true, true];

    assert!(f._coeff.eq_vec(&expected_coeff))
}

#[test]
fn test_add_larger_degree() {
    let coeff_f = vec![true, true, false, false, true, true, false, true];
    let coeff_g = vec![true, false, true, false, true, false, true, true, true];

    let mut f = BinPoly::from_coeff(coeff_f.iter());
    let f_copy = f.clone();

    let g = BinPoly::from_coeff(coeff_g.iter());
    let g_copy = g.clone();

    assert!(f == f_copy);

    f.add(&g);

    assert!(f != f_copy);
    assert!(g == g_copy);

    let expected_coeff = vec![false, true, true, false, false, true, true, false, true];

    assert!(f._coeff.eq_vec(&expected_coeff))
}

#[test]
fn test_shift() {
    const SHIFT: usize = 4;
    let coeff = vec![true, true, false, false, true, true, false, true];

    let mut f = BinPoly::from_coeff(coeff.iter());

    let previous_degree = f.degree();

    f.shift(SHIFT);

    let expected_coeff = vec![false, false, false, false, true, true,
                              false, false, true, true, false, true];

    assert!(f._coeff.eq_vec(&expected_coeff));
    assert_eq!(f.degree(), previous_degree + SHIFT);
}

#[test]
fn test_into_taps() {
    let coeff = vec![true, true, false, false, true, true, false, true, false, false];

    let f = BinPoly::from_coeff(coeff.iter());

    let taps = f.into_taps();

    assert_eq!(taps, vec![1, 4, 5, 7]);
}
