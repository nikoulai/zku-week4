// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
use ark_bls12_381::{Fq};
use ark_ff::biginteger::{ BigInteger384};
use ark_std::rand;
use ndarray::{Array2, Array1, Array, Axis, arr2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::rand_distr::num_traits::{Pow, PrimInt};
use ark_std::rand::Rng;

struct Freivald {
    x:  Array1<Fq>// Array/Vec of Fq,
}
pub fn fqEl(val: u64)->Fq {
   Fq::new(BigInteger384::from(val))
}

impl Freivald {
    // TODO: Create constructor for object
    fn new(array_size: usize) -> Self {
        // todo!();
    let mut rng = ark_std::test_rng();
    let r: Fq = rng.gen();

    // Populate vector with values r^i for i=0..matrix_size
    //overflows :/
    // let vec: Vec<Fq> = (1..((array_size+1) as u64)).map(|x| fqEl(x.pow(r.into() ))).collect();
    let mut vec  = Vec::with_capacity(array_size);
    let mut temp: Fq = fqEl(1);
    for _ in 1..=array_size {
        temp *= r;
        vec.push(temp);
    }
        // Return freivald value with this vector as its x value
        let arr: Array1<Fq> = Array::from(vec);
        return Freivald{
            x: arr
        }
    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));
        // todo!()
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
        matrix_a.dot(&matrix_b.dot(&self.x)) == supposed_ab.dot(&self.x)

        
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool { 
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    // todo!()
    let r: u8 = rand::random();
    let vec: Vec<Fq> = (1..((200+1) as u64)).map(|x| fqEl(x.pow(r.into() ))).collect();

}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    // todo!();
    //We allow only nxn array so we will also check that arrays are square + m.n x n.k
     matrix_a.len_of(Axis(0)) == matrix_a.len_of(Axis(1)) &&
        matrix_b.len_of(Axis(0)) == matrix_b.len_of(Axis(1)) &&
        supposed_ab.len_of(Axis(0)) == supposed_ab.len_of(Axis(1)) &&
        matrix_a.len_of(Axis(1)) == matrix_b.len_of(Axis(0))
}


#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        // todo!("add matrices types and values")
        static ref MATRIX_A: Array2<Fq>/* Type of matrix. Values should be fq */ = Array::random((200,200), Uniform::new(0u64, 100)).mapv(|x| fqEl(x));
        // arr2(&[[fqEl(1), fqEl(2)],
            // [fqEl(0), fqEl(1)]]);
             /* arbitrary matrix */
        static ref MATRIX_A_DOT_A: Array2<Fq>/* Type of matrix. Values should be fq */ = /* Correct result of A * A */
        (MATRIX_A).dot(&*MATRIX_A); 

        static ref MATRIX_B: Array2<Fq> /* Type of matrix. Values should be fq */ = /* arbitrary matrix */
        Array::random((200,200), Uniform::new(0u64, 100)).mapv(|x| fqEl(x)); 

        static ref MATRIX_B_DOT_B: Array2<Fq> /* Type of matrix. Values should be fq */ = /* Correct result of B * B */
        (MATRIX_B).dot(&*MATRIX_B); 
        static ref MATRIX_C: Array2<Fq> /* Type of matrix. Values should be fq */ = /* arbitrary LARGE matrix (at least 200, 200)*/
      Array::random((200,200), Uniform::new(0u64, 100)).mapv(|x| fqEl(x)); 
        static ref MATRIX_C_DOT_C: Array2<Fq>/* Type of matrix. Values should be fq */ = /* Correct result of C * C */
        (MATRIX_C).dot(&*MATRIX_C); 
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq> /* Type of matrix. Values should be fq */,
        #[case] matrix_b: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] supposed_ab: &Array2<Fq> /* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq> /* Type of matrix. Values should be fq */,
        #[case] b: &Array2<Fq>/* Type of matrix. Values should be fq */,
        #[case] c: &Array2<Fq>/* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
