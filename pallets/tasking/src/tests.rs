use crate::{self as pallet_tasking, Config};
use frame_support::{assert_ok, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TaskingPallet: pallet_tasking::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for TestRuntime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl Config for TestRuntime {
	type Event = Event;
}

struct ExternalityBuilder;

// Build genesis storage according to the mock runtime.
impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

// Refer to following issue for FRAME V2 sample code
// https://github.com/substrate-developer-hub/recipes/issues/458

#[test]
fn task_actions() {
	ExternalityBuilder::build().execute_with(|| {
		// Create a task.
		assert_ok!(TaskingPallet::create_task(Origin::signed(1), 1));
		println!("{:?}", System::events());

		// Get the list of all tasks
		let tasks = vec![1];
		let expected_event = Event::TaskingPallet(pallet_tasking::Event::GetTask(tasks));
		assert_ok!(TaskingPallet::get_task(Origin::none()));

		assert_eq!(System::events()[1].event, expected_event);
	});
}

// #[test]
// fn check_errors() {
// 	// TODO
// 	assert!(true);
// }
