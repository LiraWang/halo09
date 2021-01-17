use halo09::commitment_scheme::kzg10::PublicParameters;
use dusk_bls12_381::BlsScalar;
use halo09::fft::Polynomial;
use halo09::lagrange_interpolation;


fn main() {
    let a = 1;
    let result = a / 2 % 2;
    println!("{}", result);
}
