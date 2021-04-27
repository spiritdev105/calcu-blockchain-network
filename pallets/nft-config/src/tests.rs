#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};

#[test]
fn test_whitelist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(None, NftmartConfig::account_whitelist(ALICE));
		assert_eq!(None, NftmartConfig::account_whitelist(BOB));
		assert_ok!(NftmartConfig::add_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::nftmart_config(crate::Event::AddWhitelist(ALICE)));
		assert_eq!(Some(()), NftmartConfig::account_whitelist(ALICE));
		assert_noop!(
			NftmartConfig::add_whitelist(Origin::signed(BOB), BOB),
			DispatchError::BadOrigin,
		);

		assert_ok!(NftmartConfig::remove_whitelist(Origin::root(), ALICE));
		assert_eq!(last_event(), Event::nftmart_config(crate::Event::RemoveWhitelist(ALICE)));
		assert_eq!(None, NftmartConfig::account_whitelist(ALICE));
		assert_eq!(None, NftmartConfig::account_whitelist(BOB));
	});
}
