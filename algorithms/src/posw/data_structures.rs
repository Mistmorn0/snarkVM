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

use crate::polycommit::{
    kzg10::{Commitment, Proof},
    sonic_pc::{CommitterKey, VerifierKey},
};
use snarkvm_curves::PairingEngine;
use std::marker::PhantomData;

use crate::fft::DensePolynomial;

pub type SRS<E> = PhantomData<E>;
pub type VerifyingKey<E> = crate::polycommit::sonic_pc::VerifierKey<E>;

#[derive(Clone, Debug)]
pub struct ProvingKey<E: PairingEngine> {
    pub ck: CommitterKey<E>,
    pub vk: VerifierKey<E>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpochChallenge<E: PairingEngine> {
    pub epoch_polynomial: DensePolynomial<E::Fr>,
}

impl<E: PairingEngine> EpochChallenge<E> {
    pub fn degree(&self) -> usize {
        self.epoch_polynomial.degree()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProverPuzzleSolution<E: PairingEngine> {
    pub address: Address,
    pub nonce: u64,
    pub commitment: Commitment<E>,
    pub proof: Proof<E>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombinedPuzzleSolution<E: PairingEngine> {
    pub individual_puzzle_solutions: Vec<(Address, u64, Commitment<E>)>,
    pub proof: Proof<E>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EpochInfo {
    epoch_number: u64,
}

impl EpochInfo {
    pub fn to_bytes_le(&self) -> [u8; 8] {
        self.epoch_number.to_le_bytes()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn to_bytes_le(&self) -> [u8; 32] {
        self.0
    }
}
