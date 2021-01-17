use halo09::commitment_scheme::kzg10::PublicParameters;
use dusk_bls12_381::BlsScalar;
use halo09::fft::Polynomial;
use halo09::lagrange_interpolation;


fn main() {
    // let max_degree = 25 as usize;
    // let srs = PublicParameters::setup(max_degree, &mut rand::thread_rng()).unwrap();
    // let (proving_key, opening_key) = srs.trim(max_degree).unwrap();
    // 
    // let s = vec![BlsScalar::from(2u64), BlsScalar::one(), BlsScalar::from(3u64)];
    // let s = Polynomial::from_coefficients_vec(s);
    // let dual_s = lagrange_interpolation::poly_interpolation(s);
    // 
    // let point = BlsScalar::from(10u64);
    // let value = dual_s.evaluate(&point);
    // 
    // let proof_dual_s = proving_key.open_single(&dual_s, &value, &point).unwrap();
    // 
    // let ok = opening_key.check(point, proof_dual_s);
    // assert!(ok);


}
