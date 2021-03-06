// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    base_dpc::{parameters::SystemParameters, BaseDPCComponents},
    Assignment,
};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    algorithms::{CommitmentScheme, CRH},
    gadgets::{
        algorithms::{CRHGadget, CommitmentGadget},
        r1cs::{ConstraintSynthesizer, ConstraintSystem},
        utilities::{alloc::AllocGadget, uint::UInt8},
    },
};

/// Always-accept program
pub struct NoopCircuit<C: BaseDPCComponents> {
    /// System parameters
    pub system_parameters: Option<SystemParameters<C>>,

    /// Commitment to the program input.
    pub local_data_root: Option<<C::LocalDataCRH as CRH>::Output>,

    /// Record position
    pub position: u8,
}

impl<C: BaseDPCComponents> NoopCircuit<C> {
    pub fn blank(system_parameters: &SystemParameters<C>) -> Self {
        let local_data_root = <C::LocalDataCRH as CRH>::Output::default();

        Self {
            system_parameters: Some(system_parameters.clone()),
            local_data_root: Some(local_data_root),
            position: 0u8,
        }
    }

    pub fn new(
        system_parameters: &SystemParameters<C>,
        local_data_root: &<C::LocalDataCRH as CRH>::Output,
        position: u8,
    ) -> Self {
        Self {
            system_parameters: Some(system_parameters.clone()),
            local_data_root: Some(local_data_root.clone()),
            position,
        }
    }
}

impl<C: BaseDPCComponents> ConstraintSynthesizer<C::InnerField> for NoopCircuit<C> {
    fn generate_constraints<CS: ConstraintSystem<C::InnerField>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        execute_noop_gadget(
            cs,
            self.system_parameters.get()?,
            self.local_data_root.get()?,
            self.position,
        )
    }
}

fn execute_noop_gadget<C: BaseDPCComponents, CS: ConstraintSystem<C::InnerField>>(
    cs: &mut CS,
    system_parameters: &SystemParameters<C>,
    local_data_root: &<C::LocalDataCRH as CRH>::Output,
    position: u8,
) -> Result<(), SynthesisError> {
    let _position = UInt8::alloc_input_vec(cs.ns(|| "Alloc position"), &[position])?;

    let _local_data_commitment_parameters_gadget =
        <C::LocalDataCommitmentGadget as CommitmentGadget<_, _>>::ParametersGadget::alloc_input(
            &mut cs.ns(|| "Declare local data commitment parameters"),
            || Ok(system_parameters.local_data_commitment.parameters().clone()),
        )?;

    let _local_data_root_gadget = <C::LocalDataCRHGadget as CRHGadget<_, _>>::OutputGadget::alloc_input(
        cs.ns(|| "Allocate local data root"),
        || Ok(local_data_root),
    )?;

    Ok(())
}
