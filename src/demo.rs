use crate::commitment_scheme::kzg10::PublicParameters;
use dusk_bls12_381::BlsScalar;
use crate::fft::Polynomial;
use crate::lagrange_interpolation;
use crate::lagrange_interpolation::{compute_f, compute_dual_vec, poly_interpolation};

#[test]
fn test_main() {
    //srs
    let max_degree = 25 as usize;
    let srs = PublicParameters::setup(max_degree, &mut rand::thread_rng()).unwrap();
    let (proving_key, opening_key) = srs.trim(max_degree).unwrap();

    // the random point that is chosen by the Verifier
    let rand_point = BlsScalar::from(100).invert().unwrap();

    //simulate the challenge and compute its invert
    let k = 4;
    let mut powers_of_x = vec![];
    let mut content_vec = vec![];
    let mut locate_vec_ori = vec![];
    //all one, help to save the packed challenges
    let mut cur = BlsScalar::one();
    let mut locate = 0;
    for _ in 0..(1 << k) {
        powers_of_x.push(cur);
        content_vec.push(BlsScalar::one());
        locate_vec_ori.push(locate);
        // println!("the current locate_vec_ori is {}", locate);
        cur = rand_point * cur;
        locate += 1;
    }


    let mut challenges = vec![];
    let mut challenges_inv = vec![];
    let mut packed_challenges = content_vec;

    let mut cur_k = k;
    let mut b = powers_of_x.clone();
    //the current challenge should be a random challenge
    let mut curchallenge = BlsScalar::from(109).invert().unwrap();
    while b.len() > 1 {
        let l = 1 << (cur_k - 1);
        let mut curchallenge_inv = curchallenge.invert().unwrap();
        challenges.push(curchallenge);
        challenges_inv.push(curchallenge_inv);

        for j in 0..l {
            b[j] = (b[j] * curchallenge_inv) + (b[j + l] * curchallenge);
        }
        for i in 0..(1 << k) {
            let tag  = locate_vec_ori[i] / l % 2;
            println!("the current tag is {}", tag);
            if tag == 0 {
                packed_challenges[i] = packed_challenges[i] * curchallenge_inv.clone();
            } else if tag == 1{ packed_challenges[i] = packed_challenges[i] * curchallenge.clone(); }
        }


        /// Shortens the vector, keeping the first `len` elements and dropping
        /// the rest.
        b.truncate(l);

        cur_k -= 1;
        curchallenge = (curchallenge.clone() + BlsScalar::from(129)).square();
    }

    let s_packed_challenge = packed_challenges;
    let s_packed_challenge_poly = Polynomial::from_coefficients_vec(s_packed_challenge.clone());


    let length = s_packed_challenge.len();

    //compute the dual vector and commit to it
    let dual_s = compute_dual_vec(s_packed_challenge_poly.clone(), length.clone());
    let dual_s_2 = Polynomial::from_coefficients_vec(dual_s);

    let commit_dual_s_poly = proving_key.commit(&s_packed_challenge_poly).unwrap();

    let z = s_packed_challenge_poly.evaluate(&rand_point);

    let proof = proving_key.open_single(&s_packed_challenge_poly, &z, &rand_point).unwrap();
    //witness
    // let witness_dual_s = proving_key.compute_single_witness(&s_packed_challenge_poly, &rand_point);
    // let commit_witness_dual_s = proving_key.commit(&witness_dual_s).unwrap();

    let s = challenges;
    let mut s_invert = challenges_inv;

    let z_compute_f = compute_f(rand_point, &s, &s_invert);
    assert_eq!(z_compute_f, z);

    let ok = opening_key.check(rand_point, proof);
    assert!(ok);

}
