#[macro_use] extern crate serde_derive;

mod crypto_ctx;
mod hw_client;
mod hw_ctx;
pub mod hw_rpc_task;
mod key_pair_ctx;

pub use crypto_ctx::{CryptoCtx, CryptoInitError, CryptoInitResult};
pub use hw_client::TrezorConnectProcessor;
pub use hw_client::{HwClient, HwError, HwProcessingError, HwResult, HwWalletType};
pub use hw_common::primitives::{Bip32Error, ChildNumber, DerivationPath, EcdsaCurve, ExtendedPublicKey,
                                Secp256k1ExtendedPublicKey};
pub use hw_ctx::HardwareWalletCtx;
pub use key_pair_ctx::KeyPairCtx;
pub use trezor;
