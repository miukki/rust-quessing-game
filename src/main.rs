use self::dusk::plonk;

use dusk_plonk::prelude::*;
use dusk_jubjub::{JubJubAffine, JubJubScalar};

use rand_core::OsRng;

//plonk usage
mod dusk {
    pub fn plonk() {
        println!("Complete examples TestCircuit");
    }
}

//cargo doc --open
//target/doc/rand/trait.Rng.html#method.gen_range

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
        let zero_var = composer.add_witness_to_circuit_description(BlsScalar::zero());
        println!("zero_var {:?}", zero_var); // ok!
        // Make first constraint a + b = c
        composer.poly_gate(
            a,
            b,
            zero_var,
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
            zero_var,
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



fn circuit_verify() -> Result<(), Error> {

    // Now let's use the Circuit we've just implemented!

    let pp = PublicParameters::setup(1 << 12, &mut OsRng)?;
    // println!("pp, {:?}", pp);

    // Initialize the circuit
    //representing  as a circuit with logic gates
    //we build circuit (type default) 
    let mut circuit = TestCircuit::default();

    //how to create custom gate?

    // Compile the circuit
    let (pk, vd) = circuit.compile(&pp)?;


    // Prover POV
    let proof = {
        //for polynomial equations
        //give me a set of values that satisfies a set of math equations
        let mut circuit = TestCircuit {
            a: BlsScalar::from(20u64),
            b: BlsScalar::from(5u64),
            c: BlsScalar::from(25u64),
            d: BlsScalar::from(100u64),
            e: JubJubScalar::from(2u64),
            f: JubJubAffine::from(
                dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
            ),
        };
        circuit.gen_proof(&pp, &pk, b"Test")
    }?;

    // Verifier POV
    // what is vec! ?
    let public_inputs: Vec<PublicInputValue> = vec![
        BlsScalar::from(25u64).into(),
        BlsScalar::from(100u64).into(),
        JubJubAffine::from(
            dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
        )
        .into(),
    ];

    

    circuit::verify_proof(
        &pp,
        &vd.key(),
        &proof,
        &public_inputs,
        &vd.pi_pos(),
        b"Test",
    )
     


}



fn main() {

   
    let result =  circuit_verify();
    println!("result::: {:?}", result);


}


