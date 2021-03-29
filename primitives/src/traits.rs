// Copyright (C) 2019-2021 Calcu Network Technologies Ltd.
// This file is part of Calcu.

use frame_support::traits::LockableCurrency;
use crate::{TarsAnchor, MerkleRoot, BlockNumber};
use sp_std::collections::btree_set::BTreeSet;

/// A currency whose accounts can have liquidity restrictions.
pub trait UsableCurrency<AccountId>: LockableCurrency<AccountId> {
	fn usable_balance(who: &AccountId) -> Self::Balance;
}

/// Means for interacting with a specialized version of the `swork` trait.
pub trait TarsInterface<AccountId> {
	// Check whether work report was reported in the last report slot according to given block number
	fn is_wr_reported(anchor: &TarsAnchor, bn: BlockNumber) -> bool;
	// Update the used value in anchor's work report
	fn update_used(anchor: &TarsAnchor, decreased_used: u64, increased_used: u64);
    // Check whether the who and anchor is consistent with current status
	fn check_anchor(who: &AccountId, anchor: &TarsAnchor) -> bool;
	// Get total used and free space
	fn get_total_capacity() -> u128;
}

/// Means for interacting with a specialized version of the `market` trait.
pub trait MarketInterface<AccountId, Balance> {
	// used for `added_files`
	// return real used size of this file
	fn upsert_replica(who: &AccountId, cid: &MerkleRoot, reported_file_size: u64, anchor: &TarsAnchor, valid_at: BlockNumber, members: &Option<BTreeSet<AccountId>>) -> u64;
	// used for `delete_files`
	// return real used size of this file
	fn delete_replica(who: &AccountId, cid: &MerkleRoot, anchor: &TarsAnchor) -> u64;
	// used for distribute market staking payout
	fn withdraw_staking_pot() -> Balance;
}
