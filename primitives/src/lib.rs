#![cfg_attr(not(feature = "std"), no_std)]

pub mod p_resource_order;
pub mod p_provider;
pub mod constants;

use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Counter for the number of eras that have passed.
pub type EraIndex = u32;
