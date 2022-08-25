extern crate bincode;
extern crate dusk_plonk;
extern crate merlin;

// use dusk_bls12_381::BlsScalar;
// use dusk_plonk::commitment_scheme::kzg10::PublicParameters;
// use dusk_plonk::constraint_system::StandardComposer;
// use dusk_plonk::commitment_scheme::PublicParameters;
use dusk_plonk::constraint_system::TurboComposer;
// use dusk_plonk::fft::EvaluationDomain;
// use merlin::Transcript;
// use std::fs;
use dusk_plonk::{prelude::*, proof_system};
use rand_core::OsRng;
use dusk_jubjub;


// #[derive(Debug, Default)]
// pub struct TestCircuit {
//     a: BlsScalar,
//     b: BlsScalar,
//     c: BlsScalar,
//     d: BlsScalar,
//     e: JubJubScalar,
//     f: JubJubAffine,
// }


// impl Circuit for TestCircuit {
//     const CIRCUIT_ID: [u8; 32] = [0xff; 32];

//     fn gadget(&mut self, composer: &mut TurboComposer) -> Result<(), Error> {
//         let a = composer.append_witness(self.a);
//         let b = composer.append_witness(self.b);

//         //  a + b = c where C is a PI
//         let constraint = Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);
//         composer.append_gate(constraint);

//         // a <= 2^6
//         composer.component_range(a, 1 << 6 );

//         // b <= 2^5
//         composer.component_range(a, 1 << 5 );


//         // a * b = d where D is a PI
//         let constraint = Constraint::new().mult(1).public(-self.d).a(a).b(b);
//         composer.append_gate(constraint);

//         // JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input
//         let e = composer.append_witness(self.e);
//         let mul_result = composer.component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED);
//         composer.assert_equal_public_point(mul_result, self.f);

//         Ok(())
//     }

//     fn public_inputs(&self) -> Vec<PublicInputValue> {
//         vec![self.c.into()]
//     }

//     fn padded_gates(&self) -> usize {
//         1 << 11
//     }
// }



// fn main() {
//     println!("Hello, world! {}", 1 << 4 );

//     let public_param = PublicParameters::setup(1 << 12, &mut OsRng).unwrap();
    
//     let mut c = TestCircuit::default();

//     let (pk, vk) = c.compile(&public_param).unwrap();

//     let proof = {
//         let mut circuit = TestCircuit{
//             a: BlsScalar::from(20u64),
//             b: BlsScalar::from(5u64),
//             c: BlsScalar::from(25u64),
//             d: BlsScalar::from(100u64),
//             e: JubJubScalar::from(2u64),
//             f: JubJubAffine::from(
//                 dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
//             ),
//         };

//         circuit.prove(&public_param, &pk, b"test", &mut OsRng).unwrap()
//     };

//         let public_inputs: Vec<PublicInputValue> = vec![BlsScalar::from(25u64).into(), BlsScalar::from(100u64).into(), BlsScalar::from(2u64).into()];
    
//         TestCircuit::verify(&public_param, &vk, &proof, &public_inputs, b"Test").unwrap();


//     }


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
            composer: &mut TurboComposer,
        ) -> Result<(), Error> {
            let a = composer.append_witness(self.a);
            let b = composer.append_witness(self.b);
    
            // Make first constraint a + b = c
            let constraint = Constraint::new()
                .left(1)
                .right(1)
                .public(-self.c)
                .a(a)
                .b(b);
    
            composer.append_gate(constraint);
    
            // Check that a and b are in range
            composer.component_range(a, 1 << 6);
            composer.component_range(b, 1 << 5);
    
            // Make second constraint a * b = d
            let constraint = Constraint::new()
                .mult(1)
                .public(-self.d)
                .a(a)
                .b(b);
    
            composer.append_gate(constraint);
    
            let e = composer.append_witness(self.e);
            let scalar_mul_result = composer
                .component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED);
    
            // Apply the constraint
            composer.assert_equal_public_point(scalar_mul_result, self.f);
            Ok(())
        }
    
        fn public_inputs(&self) -> Vec<PublicInputValue> {
            vec![self.c.into(), self.d.into(), self.f.into()]
        }
    
        fn padded_gates(&self) -> usize {
            1 << 11
        }
    }
    
    // fn main() {
    //     println!("Hello, world! {}", 1 << 4 );
    
    //     let pp = PublicParameters::setup(1 << 12, &mut OsRng).unwrap();
    //     // Initialize the circuit
    //     let mut circuit = TestCircuit::default();
    //     // Compile/preproces the circuit
    //     let (pk, vd) = circuit.compile(&pp).unwrap();
        
    //     // Prover POV
    //     let proof = {
    //         let mut circuit = TestCircuit {
    //             a: BlsScalar::from(20u64),
    //             b: BlsScalar::from(5u64),
    //             c: BlsScalar::from(25u64),
    //             d: BlsScalar::from(100u64),
    //             e: JubJubScalar::from(2u64),
    //             f: JubJubAffine::from(
    //                 dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
    //             ),
    //         };
    //         circuit.prove(&pp, &pk, b"Test", &mut OsRng).unwrap()
    //     };
        
    //     // Verifier POV
    //     let public_inputs: Vec<PublicInputValue> = vec![
    //         BlsScalar::from(25u64).into(),
    //         BlsScalar::from(100u64).into(),
    //         JubJubAffine::from(
    //             dusk_jubjub::GENERATOR_EXTENDED * JubJubScalar::from(2u64),
    //         )
    //         .into(),
    //     ];
    //     TestCircuit::verify(
    //         &pp,
    //         &vd,
    //         &proof,
    //         &public_inputs,
    //         b"Test",
    //     ).unwrap();
    
    //     println!("Hello, world! {}", 1 << 8 );
    
    
    //     }
    
    



////
/// 
/// 
/// 
///             12, h
///            /      \
///          8, h     4, h
///          /\       /\
///       7,h 1,h   2,h 2,h


//// What we need to prove
/// Leaf-31: {amount:7 , h(email,7)}
/// Leaf-32: {amount:1 , h(email,1)}
/// Leaf-33: {amount:2 , h(email,2)}
/// Leaf-34: {amount:2 , h(email,2)}
/// Leaf-21: {amount:8 , h(L31.hash+amount | L32.hash+amount)}
/// Leaf-22: {amount:4 , h(L33.hash+amount | L34.hash+amount)}
/// Leaf-11: {amount:12 , h(L21.hash+amount | L22.hash+amount)}
/// 
/// Proof leaf level => amount is positive => inputs: amount 
/// Proof intermedian level => amount is positive, summation, proof-of-primage => inputs: amountLeft, hashLeft, amountRight, hashRight, sum, hashSum
/// 
/// 



    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// 
    /// Leaf Circuit


    #[derive(Debug, Default)]
    pub struct LeafCircuit {
        a: BlsScalar,
    }
    
    
    impl Circuit for LeafCircuit {
        const CIRCUIT_ID: [u8; 32] = [0xff; 32];
        fn gadget(
            &mut self,
            composer: &mut TurboComposer,
        ) -> Result<(), Error> {
            let a = composer.append_witness(self.a);

            // composer.component_range(a, 1 << 2);
            composer.component_range(a, 64);

            Ok(())
        }
    
        fn public_inputs(&self) -> Vec<PublicInputValue> {
            vec![]
        }
    
        fn padded_gates(&self) -> usize {
            1 << 11
        }
    }
    
    fn main() {
        println!("Hello, world! {}", 1 << 4 );
    
        let pp = PublicParameters::setup(1 << 12, &mut OsRng).unwrap();
        // Initialize the circuit
        let mut circuit = LeafCircuit::default();
        // Compile/preproces the circuit
        let (pk, vd) = circuit.compile(&pp).unwrap();
        
        // Prover POV
        let proof = {
            let mut circuit = LeafCircuit {
                // a: BlsScalar::from(200u64),
                a: BlsScalar::from(4_294_967_297u64),
            };
            circuit.prove(&pp, &pk, b"Test", &mut OsRng).unwrap()
        };
        
        // Verifier POV
        let public_inputs: Vec<PublicInputValue> = vec![];

        TestCircuit::verify(
            &pp,
            &vd,
            &proof,
            &public_inputs,
            b"Test",
        ).unwrap();
    
        println!("Hello, world! {}", 1 << 8 );
    
    
        }



