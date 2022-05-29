// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;
use snarkvm_circuit_algorithms::{Hash, Poseidon, BHP};
use snarkvm_circuit_network::Aleo;

/// A trait for a Merkle leaf hash function.
pub trait LeafHash<A: Aleo> {
    type Leaf;

    /// Returns the hash of the given leaf node.
    fn hash(&self, leaf: &Self::Leaf) -> Field<A>;
}

impl<A: Aleo, const NUM_WINDOWS: u8, const WINDOW_SIZE: u8> LeafHash<A> for BHP<A, NUM_WINDOWS, WINDOW_SIZE> {
    type Leaf = Vec<Boolean<A>>;

    /// Returns the hash of the given leaf node.
    fn hash(&self, leaf: &Self::Leaf) -> Field<A> {
        // Prepend the leaf with a `false` bit.
        let mut input = vec![Boolean::constant(false)];
        input.extend_from_slice(leaf);
        // Hash the input.
        Hash::hash(self, &input)
    }
}

impl<A: Aleo, const RATE: usize> LeafHash<A> for Poseidon<A, RATE> {
    type Leaf = Vec<Field<A>>;

    /// Returns the hash of the given leaf node.
    fn hash(&self, leaf: &Self::Leaf) -> Field<A> {
        // Prepend the leaf with a `0field` element.
        let mut input = vec![Field::zero()];
        input.extend_from_slice(leaf);
        // Hash the input.
        Hash::hash(self, &input)
    }
}

#[cfg(all(test, console))]
mod tests {
    use super::*;
    use snarkvm_circuit_algorithms::{Poseidon4, BHP1024};
    use snarkvm_circuit_network::{Aleo, AleoV0 as Circuit};
    use snarkvm_utilities::{test_rng, UniformRand};

    use anyhow::Result;

    const ITERATIONS: u64 = 10;
    const DOMAIN: &str = "MerkleTreeCircuit0";

    macro_rules! check_hash {
        ($hash:ident, $form:ident, $mode:ident, $input_type:ty, $num_inputs:expr, ($num_constants:expr, $num_public:expr, $num_private:expr, $num_constraints:expr)) => {{
            // Initialize the hash.
            let native = snarkvm_console_algorithms::$hash::<<Circuit as Environment>::$form>::setup(DOMAIN)?;
            let circuit = $hash::<Circuit>::constant(native.clone());

            for i in 0..ITERATIONS {
                // Sample a random input.
                let input = (0..$num_inputs).map(|_| <$input_type>::rand(&mut test_rng())).collect::<Vec<_>>();

                // Compute the expected hash.
                let expected: <Circuit as Environment>::BaseField =
                    console::merkle_tree::LeafHash::<<Circuit as Aleo>::Network>::hash(&native, &input)
                        .expect("Failed to hash native input");

                // Prepare the circuit input.
                let circuit_input: Vec<_> = Inject::new(Mode::$mode, input);

                Circuit::scope(format!("LeafHash {i}"), || {
                    // Perform the hash operation.
                    let candidate = LeafHash::hash(&circuit, &circuit_input);
                    assert_scope!($num_constants, $num_public, $num_private, $num_constraints);
                    assert_eq!(expected, candidate.eject_value());
                });
                Circuit::reset();
            }
            Ok::<_, anyhow::Error>(())
        }};
    }

    #[test]
    fn test_hash_bhp1024_constant() -> Result<()> {
        check_hash!(BHP1024, Affine, Constant, bool, 1024, (1807, 0, 0, 0))
    }

    #[test]
    fn test_hash_bhp1024_public() -> Result<()> {
        check_hash!(BHP1024, Affine, Public, bool, 1024, (429, 0, 1758, 1758))
    }

    #[test]
    fn test_hash_bhp1024_private() -> Result<()> {
        check_hash!(BHP1024, Affine, Private, bool, 1024, (429, 0, 1758, 1758))
    }

    #[test]
    fn test_hash_poseidon4_constant() -> Result<()> {
        check_hash!(Poseidon4, BaseField, Constant, <Circuit as Environment>::BaseField, 4, (1, 0, 0, 0))
    }

    #[test]
    fn test_hash_poseidon4_public() -> Result<()> {
        check_hash!(Poseidon4, BaseField, Public, <Circuit as Environment>::BaseField, 4, (1, 0, 700, 700))
    }

    #[test]
    fn test_hash_poseidon4_private() -> Result<()> {
        check_hash!(Poseidon4, BaseField, Private, <Circuit as Environment>::BaseField, 4, (1, 0, 700, 700))
    }
}
