#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};

#[test]
fn test_whitelist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(None, NFTConfig::account_whitelist(ALICE));
		assert_eq!(None, NFTConfig::account_whitelist(BOB));
		assert_ok!(NFTConfig::add_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::nftconf(crate::Event::AddWhitelist(ALICE)));
		assert_eq!(Some(()), NFTConfig::account_whitelist(ALICE));
		assert_noop!(
			NFTConfig::add_whitelist(Origin::signed(BOB), BOB),
			DispatchError::BadOrigin,
		);

		assert_ok!(NFTConfig::remove_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::nftconf(crate::Event::RemoveWhitelist(ALICE)));
		assert_eq!(None, NFTConfig::account_whitelist(ALICE));
		assert_eq!(None, NFTConfig::account_whitelist(BOB));
	});
}
