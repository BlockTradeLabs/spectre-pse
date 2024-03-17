#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

use pink_extension as pink;
use scale::{Decode, Encode};
use scale_info::TypeInfo;
use ink::storage::traits::StorageLayout;
use pink::{Balance, BlockNumber};
use ink::storage::Mapping;
use alloc::vec::Vec;

#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout, TypeInfo))]
pub struct OnchainTradingAccounts {
    pub substrate: Vec<u8>,
    pub ethereum: Vec<u8>,
    pub solana: Vec<u8>
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Network {
    Substrate,
    Ethereum,
    Solana
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Error {
    /// If the trader account is not registered
    UnregisteredTraderAccount,
    /// When the specified network private key is already registered
    PrivateKeyOfThatNetworkAlreadyRegistered,

    KeysUnavailable  
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
    use sig::SigType;
    use pink::PinkEnvironment;


    #[ink(storage)]
    pub struct SpectrePse {
        onchain_trading_account: Mapping<AccountId,OnchainTradingAccounts>,
        relayer_account: Vec<u8> // relayer private account for sponsoring some accounts txn fees
    }

    impl SpectrePse {
        /// Constructor to initializes your contract
        #[ink(constructor)]
        pub fn seeding(key: Vec<u8> ) -> Self {
            Self {
                onchain_trading_account: Mapping::default(),
                relayer_account: key,
            }
        }

        #[ink(message)]
        pub fn generate_onchain_trader_keys(&mut self) -> SigningResult<()> {
            let caller = Self::env().caller();
            let caller_public_key:&[u8;32] = caller.as_ref();

            ensure!(self.onchain_trading_account.contains(&caller),Error::UnregisteredTraderAccount);
            let sub_privkey = sig::derive_sr25519_key(caller_public_key);
            let keys = OnchainTradingAccounts {
                substrate: sub_privkey.clone(),
                ethereum: sub_privkey.clone(),
                solana: sub_privkey.clone()
            };

            self.onchain_trading_account.insert(caller,&keys);
           // let gen_pubkey = sig::get_public_key(&sub_privkey, SigType::Sr25519);
            Ok(())
        }

        #[ink(message)]
        pub fn sign(&self, network: Network, message: Vec<u8>) -> SigningResult<Vec<u8>> {
            let caller = Self::env().caller();

            ensure!(self.onchain_trading_account.contains(&caller),Error::UnregisteredTraderAccount);

            match network {
                Network::Substrate => {
                    let key = &self.onchain_trading_account.get(caller).ok_or(Error::UnregisteredTraderAccount)?.substrate;

                    let signature = sig::sign(&message, key, SigType::Sr25519);
                    Ok(signature)

                },
                Network::Ethereum => {
                    let key = &self.onchain_trading_account.get(caller).ok_or(Error::UnregisteredTraderAccount)?.ethereum;

                    let signature = sig::sign(&message, key, SigType::Ecdsa);
                    Ok(signature)
                },
                Network::Solana => {
                    let key = &self.onchain_trading_account.get(caller).ok_or(Error::UnregisteredTraderAccount)?.solana;

                    let signature = sig::sign(&message, key, SigType::Ed25519);
                    Ok(signature)
                }

            }
            
        }

        #[ink(message, selector = 0xCODE0003)]
        pub fn get_public_keys(&self) -> SigningResult<OnchainTradingAccounts>{
            let caller = Self::env().caller();

            ensure!(self.onchain_trading_account.contains(&caller),Error::UnregisteredTraderAccount);

            let keys = self.onchain_trading_account.get(caller).ok_or(Error::UnregisteredTraderAccount)?;

            Ok(keys) 
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
        fn sign_works(){
            pink_extension_runtime::mock_ext::mock_all_ext();
 
        }
    }
}
