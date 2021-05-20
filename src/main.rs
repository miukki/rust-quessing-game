use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::mem;
use self::dusk::plonk;
use self::something::User; // Ok.

//use library
use guessing_game::GREETING;
use guessing_game::say_hi;

use dusk_plonk::prelude::*;


//plonk usage
mod dusk {
    pub fn plonk() {
        println!("load examples PLONK")
        let composer = &mut StandardComposer
    }


}

  
mod something {
    #[derive(Debug)]//#[derive(Debug, Default)]
    pub struct User {
        pub name: String,
        pub id: i32
    }

    
}
//cargo doc --open
//target/doc/rand/trait.Rng.html#method.gen_range

enum State {
    Locked,
    Failed,
    Unlocked
}


// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input
#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    e: JubJubScalar,
    f: JubJubAffine,
}


impl Circuit for TestCircuit {
    const CIRCUIT_ID: [u8; 32] = [0xff; 32];
    fn gadget(
        &mut self,
        composer: &mut StandardComposer,
    ) -> Result<(), Error> {
        let a = composer.add_input(self.a);
        let b = composer.add_input(self.b);
        // Make first constraint a + b = c
        composer.poly_gate(
            a,
            b,
            composer.zero_var,
            BlsScalar::zero(),
            BlsScalar::one(),
            BlsScalar::one(),
            BlsScalar::zero(),
            BlsScalar::zero(),
            Some(-self.c),
        );
        // Check that a and b are in range
        composer.range_gate(a, 1 << 6);
        composer.range_gate(b, 1 << 5);
        // Make second constraint a * b = d
        composer.poly_gate(
            a,
            b,
            composer.zero_var,
            BlsScalar::one(),
            BlsScalar::zero(),
            BlsScalar::zero(),
            BlsScalar::one(),
            BlsScalar::zero(),
            Some(-self.d),
        );

        let e = composer.add_input(self.e.into());
        let scalar_mul_result = composer
            .fixed_base_scalar_mul(e, dusk_jubjub::GENERATOR_EXTENDED);
        // Apply the constrain
        composer.assert_equal_public_point(scalar_mul_result, self.f);
        Ok(())
    }
    fn padded_circuit_size(&self) -> usize {
        1 << 11
    }
}


fn main() {


    plonk();

    

}


