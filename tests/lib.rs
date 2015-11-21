extern crate lfsr;

#[macro_use]
mod support;

#[test]
fn iter_works_for_fibonacci() {
    use lfsr::lfsr::fibonacci::LFSR;

    iter_works_for!(LFSR);
}

#[test]
fn iter_works_for_galois() {
    use lfsr::lfsr::galois::LFSR;

    iter_works_for!(LFSR);
}

#[test]
fn same_connection_polynomial_produce_output_equal_mod_rotation() {
    use ::support::eq_mod_rotation;

    let taps = vec![2, 3];

    let mut galois = lfsr::lfsr::galois::LFSR::from_iter(taps.iter());
    let galois_output: Vec<bool> = galois.iter().take(7).collect();

    let mut fibonacci = lfsr::lfsr::fibonacci::LFSR::from_iter(taps.iter());
    let fibonacci_output: Vec<bool> = fibonacci.iter().take(7).collect();

    assert!(eq_mod_rotation(&fibonacci_output, &galois_output));
}

#[test]
fn primitive_connection_polynomial_yields_a_maximum_sequence_for_fibonacci() {
    use lfsr::lfsr::fibonacci::LFSR;

    primitive_connection_polynomial_yields_a_maximum_sequence!(LFSR);
}

#[test]
fn primitive_connection_polynomial_yields_a_maximum_sequence_for_galois() {
    use lfsr::lfsr::galois::LFSR;

    primitive_connection_polynomial_yields_a_maximum_sequence!(LFSR);
}
