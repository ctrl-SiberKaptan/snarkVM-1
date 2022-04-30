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

/// Performs a Pedersen commitment taking a 1024-bit value as input.
pub struct PedComm1024<P: Program> {
    operation: BinaryOperation<P>,
}

impl_instruction_boilerplate!(PedComm1024, BinaryOperation, "commit.ped1024");

impl_commit_instruction!(PedComm1024);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_instruction_halts, test_modes, Process};

    type P = Process;

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<P>::parse("commit.ped1024 r0 r1 into r2;").unwrap();
        assert!(matches!(instruction, Instruction::PedComm1024(_)));
    }

    test_modes!(
        address,
        PedComm1024,
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah",
        "1scalar",
        "889102317888271826718559972138868820466563749149942194168269228701119910350group"
    );
    test_modes!(
        bool,
        PedComm1024,
        "true",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        field,
        PedComm1024,
        "1field",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        group,
        PedComm1024,
        "2group",
        "1scalar",
        "2664340318215809634698318956510253812463234504768303019123996597123255397816group"
    );
    test_modes!(
        i8,
        PedComm1024,
        "1i8",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        i16,
        PedComm1024,
        "1i16",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        i32,
        PedComm1024,
        "1i32",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        i64,
        PedComm1024,
        "1i64",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        i128,
        PedComm1024,
        "1i128",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        u8,
        PedComm1024,
        "1u8",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        u16,
        PedComm1024,
        "1u16",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        u32,
        PedComm1024,
        "1u32",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        u64,
        PedComm1024,
        "1u64",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        u128,
        PedComm1024,
        "1u128",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );
    test_modes!(
        string,
        PedComm1024,
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"",
        "1scalar",
        "6258277521376623366396744618094324140083583859539902194155615459223370474204group"
    );
    test_modes!(
        scalar,
        PedComm1024,
        "1scalar",
        "1scalar",
        "7143232585354596727088537818886269936493413322580429357859918031397884359807group"
    );

    test_instruction_halts!(
        string_halts,
        PedComm1024,
        "The Pedersen hash input cannot exceed 1024 bits.",
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"",
        "1scalar"
    );
}
