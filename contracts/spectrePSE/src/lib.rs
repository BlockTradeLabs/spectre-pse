#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

use alloc::vec::Vec;
use ink::storage::traits::StorageLayout;
use ink::storage::Mapping;
use pink_extension as pink;
use pink::{AccountId, Balance, BlockNumber};
use scale::{Decode, Encode};
use scale_info::TypeInfo;

/// A struct for onchain trading accounts ( Sk, Pk )
#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout, TypeInfo))]
pub struct OnchainTradingAccounts {
    pub substrate: (Vec<u8>, AccountId),
    pub ethereum: (Vec<u8>, AccountId),
    pub solana: (Vec<u8>, AccountId),
}

/// A struct that will be sent to the spectre parachain to register onchain trading account
#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout, TypeInfo))]
pub struct OnchainTradingPublicKeys {
    pub substrate: AccountId,
    pub ethereum: AccountId,
    pub solana: AccountId,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Network {
    Substrate,
    Ethereum,
    Solana,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Error {
    /// If the trader account is not registered
    UnregisteredTraderAccount,
    /// When the specified network private key is already registered
    PrivateKeyOfThatNetworkAlreadyRegistered,

    KeysUnavailable,

    FailedToConvertPubKey,
}

type SigningResult<T> = Result<T, Error>;

#[macro_export]
macro_rules! fail {
    ( $y:expr ) => {{
        return Err($y.into());
    }};
}
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            $crate::fail!($y);
        }
    }};
}

#[pink::contract(env=PinkEnvironment)]
mod spectre_pse {
    use super::*;
    use pink::chain_extension::signing as sig;
    use pink::PinkEnvironment;
    use sig::SigType;

    #[ink(storage)]
    pub struct SpectrePse {
        // mapping of trader public key to on chain trading account
        onchain_trading_account: Mapping<AccountId, OnchainTradingAccounts>,

        relayer_account: Vec<u8>, // relayer private account for sponsoring some accounts txn fees ( spectre account )
    }

    impl SpectrePse {
        /// Constructor to initializes your contract
        #[ink(constructor)]
        pub fn seeding(key: Vec<u8>) -> Self {
            Self {
                onchain_trading_account: Mapping::default(),
                relayer_account: key,
            }
        }

        #[ink(message)]
        pub fn generate_onchain_trading_account(&mut self) -> SigningResult<()> {
            let caller = Self::env().caller();
            let caller_public_key: &[u8; 32] = caller.as_ref();

            ensure!(
                self.onchain_trading_account.contains(&caller),
                Error::UnregisteredTraderAccount
            );

            // substrate based secret key
            let sub_privkey = sig::derive_sr25519_key(caller_public_key);
            // substrate based sr25519 public key
            let sub_pub: [u8; 32] = sig::get_public_key(&sub_privkey, SigType::Sr25519)
                .try_into()
                .map_err(|_| Error::FailedToConvertPubKey)?;
            let sub_pubkey = AccountId::from(sub_pub);

            // ethereum based public key
            let compressed_ecdsa = Self::get_ecdsa_account_id(&sig::get_public_key(&sub_privkey, SigType::Ecdsa));
            let eth_pubkey = AccountId::from(compressed_ecdsa);

            // solana based public key
            let sol_pub: [u8; 32] = sig::get_public_key(&sub_privkey, SigType::Ed25519)
                .try_into()
                .map_err(|_| Error::FailedToConvertPubKey)?;
            let sol_pubkey = AccountId::from(sol_pub);

            let keys = OnchainTradingAccounts {
                substrate: (sub_privkey.clone(), sub_pubkey),
                ethereum: (sub_privkey.clone(), eth_pubkey),
                solana: (sub_privkey.clone(), sol_pubkey),
            };

            self.onchain_trading_account.insert(caller, &keys);
            // let gen_pubkey = sig::get_public_key(&sub_privkey, SigType::Sr25519);
            Ok(())
        }

        #[ink(message)]
        pub fn sign(&self, network: Network, message: Vec<u8>) -> SigningResult<Vec<u8>> {
            let caller = Self::env().caller();

            ensure!(
                self.onchain_trading_account.contains(&caller),
                Error::UnregisteredTraderAccount
            );

            match network {
                Network::Substrate => {
                    let key = &self
                        .onchain_trading_account
                        .get(caller)
                        .ok_or(Error::UnregisteredTraderAccount)?
                        .substrate;

                    let signature = sig::sign(&message, &key.0, SigType::Sr25519);
                    Ok(signature)
                }
                Network::Ethereum => {
                    let key = &self
                        .onchain_trading_account
                        .get(caller)
                        .ok_or(Error::UnregisteredTraderAccount)?
                        .ethereum;

                    let signature = sig::sign(&message, &key.0, SigType::Ecdsa);
                    Ok(signature)
                }
                Network::Solana => {
                    let key = &self
                        .onchain_trading_account
                        .get(caller)
                        .ok_or(Error::UnregisteredTraderAccount)?
                        .solana;

                    let signature = sig::sign(&message, &key.0, SigType::Ed25519);
                    Ok(signature)
                }
            }
        }

        #[ink(message)]
        pub fn register_trading_account_to_spectre(&self, message: Vec<u8>) -> SigningResult<Vec<u8>> {
            let trader_account = Self::env().caller();
            // checking if the trader public key has the generated onchain trading account
            ensure!(
                self.onchain_trading_account.contains(&trader_account),
                Error::UnregisteredTraderAccount
            );

            let key = self.relayer_account.clone();
            let signature = sig::sign(&message, &key, SigType::Ecdsa);
            
            Ok(signature)
        }

        #[ink(message)]
        pub fn get_public_keys(&self) -> SigningResult<OnchainTradingPublicKeys> {
            let caller = Self::env().caller();

            ensure!(
                self.onchain_trading_account.contains(&caller),
                Error::UnregisteredTraderAccount
            );

            let onchain_trading_account_id = self
                .onchain_trading_account
                .get(caller)
                .ok_or(Error::UnregisteredTraderAccount)?;

            let trading_accounts_ids = OnchainTradingPublicKeys {
                substrate: onchain_trading_account_id.substrate.1,
                ethereum: onchain_trading_account_id.ethereum.1,
                solana: onchain_trading_account_id.solana.1,
            };

            Ok(trading_accounts_ids)
        }

        fn get_ecdsa_account_id(input: &[u8]) -> [u8; 32] {
            use ink::env::hash;
            let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Blake2x256>(input, &mut output);
            output
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn generate_onchain_trader_keys_works() {
            pink_extension_runtime::mock_ext::mock_all_ext();

            // let contract = SpectrePse::seeding();
            // let message = String::from("hello world");
            // let sign_message = contract.sign(message.clone());
            // let verify_signature = contract.verify(message.clone(), sign_message);
            // assert!(verify_signature);
            // contract.test();
        }

        #[ink::test]
        fn sign_works() {
            pink_extension_runtime::mock_ext::mock_all_ext();
        }
    }
}
