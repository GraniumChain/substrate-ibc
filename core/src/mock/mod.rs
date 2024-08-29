use super::*;

use crate as pallet_ibc;

use sp_runtime::{traits::IdentityLookup, BuildStorage};
pub use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		ConstU128, ConstU16, ConstU32, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
		IdentityFee, Weight,
	},
	StorageValue,
};
use frame_system as system;
use pallet_ibc_utils::module::DefaultRouter;
use sp_runtime::{
	generic,
	traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};

pub type Balance = u128;
pub type Moment = u64;
pub type Signature = MultiSignature;
pub(crate) type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type AccountData = pallet_balances::AccountData<Balance>;
	type Block = Block;
}

pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxAuthorities: u32 = 100;
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

parameter_types! {
	pub const ExpectedBlockTime: u64 = 6;
	pub const ChainVersion: u64 = 0;
}

impl pallet_ibc_utils::module::AddModule for Test {
	fn add_module(router: pallet_ibc_utils::module::Router) -> pallet_ibc_utils::module::Router {
		router
	}
}

impl pallet::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TimeProvider = pallet_timestamp::Pallet<Test>;
	type ExpectedBlockTime = ExpectedBlockTime;
	const IBC_COMMITMENT_PREFIX: &'static [u8] = b"Ibc";
	type ChainVersion = ChainVersion;
	type IbcModule = DefaultRouter;
	type WeightInfo = ();
}

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Timestamp: pallet_timestamp,
		Ibc: pallet_ibc,
	}
);

#[allow(dead_code)]
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
