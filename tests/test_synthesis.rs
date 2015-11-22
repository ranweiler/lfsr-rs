use lfsr::synthesis::berlekamp_massey;
use lfsr::lfsr::galois::LFSR;
use support::eq_mod_rotation;

#[test]
fn berlekamp_massey_kat() {
    let seq = vec![true, false, false, true, false, true, true, true];
    let expected_taps = vec![2, 3];

    let result = berlekamp_massey(seq.iter());

    assert_eq!(result, expected_taps);

    let mut result_lfsr = LFSR::from_iter(result.iter());

    let result_output: Vec<bool> = result_lfsr.iter().take(8).collect();

    assert!(eq_mod_rotation(&seq, &result_output));
}
