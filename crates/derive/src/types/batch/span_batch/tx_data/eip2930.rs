//! This module contains the eip2930 transaction data type for a span batch.

use crate::types::eip2930::AccessList;
use alloy_primitives::U256;
use alloy_rlp::{Bytes, Decodable, Encodable, Header};

/// The transaction data for an EIP-2930 transaction within a span batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanBatchEip2930TransactionData {
    /// The ETH value of the transaction.
    pub value: U256,
    /// The gas price of the transaction.
    pub gas_price: U256,
    /// Transaction calldata.
    pub data: Bytes,
    /// Access list, used to pre-warm storage slots through static declaration.
    pub access_list: AccessList,
}

impl Encodable for SpanBatchEip2930TransactionData {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.value.length()
            + self.gas_price.length()
            + self.data.length()
            + self.access_list.length();
        let header = Header {
            list: true,
            payload_length,
        };

        header.encode(out);
        self.value.encode(out);
        self.gas_price.encode(out);
        self.data.encode(out);
        self.access_list.encode(out);
    }
}

impl Decodable for SpanBatchEip2930TransactionData {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let header = Header::decode(buf)?;
        if !header.list {
            return Err(alloy_rlp::Error::Custom(
                "Expected list data for EIP-2930 transaction",
            ));
        }
        let buf_len_start = buf.len();

        let value = U256::decode(buf)?;
        let gas_price = U256::decode(buf)?;
        let data = Bytes::decode(buf)?;
        let access_list = AccessList::decode(buf)?;

        if buf.len() != buf_len_start - header.payload_length {
            return Err(alloy_rlp::Error::Custom("Invalid EIP-2930 transaction RLP"));
        }

        Ok(Self {
            value,
            gas_price,
            data,
            access_list,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::SpanBatchTransactionData;
    use alloc::vec::Vec;

    #[test]
    fn encode_eip2930_tx_data_roundtrip() {
        let access_list_tx = SpanBatchEip2930TransactionData {
            value: U256::from(0xFF),
            gas_price: U256::from(0xEE),
            data: Bytes::from(alloc::vec![0x01, 0x02, 0x03]),
            access_list: AccessList::default(),
        };
        let mut encoded_buf = Vec::new();
        SpanBatchTransactionData::Eip2930(access_list_tx.clone()).encode(&mut encoded_buf);

        let decoded = SpanBatchTransactionData::decode(&mut encoded_buf.as_slice()).unwrap();
        let SpanBatchTransactionData::Eip2930(access_list_decoded) = decoded else {
            panic!(
                "Expected SpanBatchEip2930TransactionData, got {:?}",
                decoded
            );
        };

        assert_eq!(access_list_tx, access_list_decoded);
    }
}
