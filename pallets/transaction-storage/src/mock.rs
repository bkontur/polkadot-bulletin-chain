// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test environment for transaction-storage pallet.

use crate::{
	self as pallet_transaction_storage, TransactionStorageProof, DEFAULT_MAX_BLOCK_TRANSACTIONS,
	DEFAULT_MAX_TRANSACTION_SIZE,
};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, OnFinalize, OnInitialize},
};
use frame_system::{pallet_prelude::BlockNumberFor, EnsureRoot};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

pub type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		TransactionStorage: pallet_transaction_storage,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const TransactionStorageStoragePeriod: BlockNumberFor<Test> = 10;
	pub const TransactionStorageAuthorizationPeriod: BlockNumberFor<Test> = 10;
}

impl pallet_transaction_storage::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
	type MaxBlockTransactions = ConstU32<{ DEFAULT_MAX_BLOCK_TRANSACTIONS }>;
	type MaxTransactionSize = ConstU32<{ DEFAULT_MAX_TRANSACTION_SIZE }>;
	type StoragePeriod = TransactionStorageStoragePeriod;
	type MaxBlockAuthorizationExpiries = ConstU32<{ DEFAULT_MAX_BLOCK_TRANSACTIONS }>;
	type AuthorizationPeriod = TransactionStorageAuthorizationPeriod;
	type Authorizer = EnsureRoot<Self::AccountId>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = RuntimeGenesisConfig { system: Default::default() }.build_storage().unwrap();
	t.into()
}

pub fn run_to_block(n: u64, f: impl Fn() -> Option<TransactionStorageProof>) {
	while System::block_number() < n {
		if let Some(proof) = f() {
			TransactionStorage::check_proof(RuntimeOrigin::none(), proof).unwrap();
		}
		TransactionStorage::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		TransactionStorage::on_initialize(System::block_number());
	}
}
