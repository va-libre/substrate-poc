use super::*;

pub const KREIVO_PARA_ID: u32 = 1_000;

pub(crate) fn new_ext(para_id: u32) -> sp_io::TestExternalities {
	use kreivo_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	// set parachain id
	let parachain_info_config = parachain_info::GenesisConfig {
		parachain_id: para_id.into(),
	};
	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(&parachain_info_config, &mut t)
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_sudo::GenesisConfig::<Runtime> { key: Some(ALICE) }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
