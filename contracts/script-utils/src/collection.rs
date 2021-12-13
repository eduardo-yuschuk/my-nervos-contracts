use crate::error::Error;
use crate::helper::{parse_dyn_vec_len, u64_from_slice, DYN_MIN_LEN};
use alloc::vec::Vec;
use core::result::Result;

const FIXED_LEN: usize = 9;

// FIXED_LEN + DYN_MIN_LEN * 2
const COLLECTION_DATA_MIN_LEN: usize = 13;
pub const COLLECTION_TYPE_ARGS_LEN: usize = 24;

/// Collection cell data structure
/// This structure contains the following information:
/// 1) version: u8
/// 2) cost: u64 NFT minting cost in CKB
/// 3) name: <size: u16> + <content>
/// 4) symbol: <size: u16> + <content>
/// 5) token_uri: <size: u16> + <content>
/// The fields of 3), 4) cannot be changed after they are set and they cannot be
/// missing. The fields of 1) and 2) can be changed and it cannot be missing.
/// The filed of 5) can be changed and it also can be missing and it will not be validated.
#[derive(Debug, Clone)]
pub struct Collection {
    pub version: u8,
    pub cost: u64,
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
    pub token_uri: Vec<u8>,
}

impl Collection {
    pub fn from_data(data: &[u8]) -> Result<Self, Error> {
        if data.len() < COLLECTION_DATA_MIN_LEN {
            return Err(Error::CollectionDataInvalid);
        }

        let version: u8 = data[0];
        if version != 0 {
            return Err(Error::VersionInvalid);
        }

        let cost = u64_from_slice(&data[1..9]);

        let name_len = parse_dyn_vec_len(&data[FIXED_LEN..(FIXED_LEN + DYN_MIN_LEN)]);
        // DYN_MIN_LEN: the min length of description
        if data.len() < FIXED_LEN + name_len + DYN_MIN_LEN {
            return Err(Error::CollectionDataInvalid);
        }
        let name = data[FIXED_LEN..(FIXED_LEN + name_len)].to_vec();

        let description_index = FIXED_LEN + name_len;
        let description_len =
            parse_dyn_vec_len(&data[description_index..(description_index + DYN_MIN_LEN)]);
        // DYN_MIN_LEN: the min length of meta_data_cell_type_hash
        if data.len() < description_index + description_len + DYN_MIN_LEN {
            return Err(Error::CollectionDataInvalid);
        }
        let symbol = data[description_index..(description_index + description_len)].to_vec();

        let meta_data_cell_type_hash_index = FIXED_LEN + name_len + description_len;
        let meta_data_cell_type_hash_len = parse_dyn_vec_len(
            &data[meta_data_cell_type_hash_index..(meta_data_cell_type_hash_index + DYN_MIN_LEN)],
        );
        let token_uri = data[meta_data_cell_type_hash_index
            ..(meta_data_cell_type_hash_index + meta_data_cell_type_hash_len)]
            .to_vec();

        Ok(Collection {
            version,
            cost,
            name,
            symbol,
            token_uri,
        })
    }

    pub fn immutable_equal(&self, other: &Collection) -> bool {
        self.name == other.name && self.symbol == other.symbol
    }
}
