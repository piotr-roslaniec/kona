#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anton-rs/kona/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/anton-rs/kona/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod errors;
pub use errors::{ExecutorError, ExecutorResult, TrieDBError, TrieDBResult};

mod executor;
pub use executor::{KonaHandleRegister, StatelessL2BlockExecutor, StatelessL2BlockExecutorBuilder};

mod db;
pub use db::{TrieAccount, TrieDB};

mod constants;
mod syscalls;

#[cfg(test)]
mod test {
    use crate::{executor::test::TestdataTrieProvider, StatelessL2BlockExecutor};
    use alloy_primitives::{hex, Bytes, B256};
    use alloy_rlp::Decodable;
    use kona_mpt::NoopTrieHinter;
    use op_alloy_genesis::OP_MAINNET_CONFIG;
    use std::collections::HashMap;

    #[test]
    fn test_compute_output_root() {
        let mut l2_block_executor = StatelessL2BlockExecutor::builder(
            &OP_MAINNET_CONFIG,
            TestdataTrieProvider::new("block_121184863_exec"),
            NoopTrieHinter,
        )
        .build();

        let expected_output_root =
            hex!("3ea8b0e09b39e9daa1b1520fe59faef02de3656d230d876544952cbc44d6d71f");
        let computed_output_root = l2_block_executor.compute_output_root().unwrap();
        assert_eq!(computed_output_root, expected_output_root);
    }
}
