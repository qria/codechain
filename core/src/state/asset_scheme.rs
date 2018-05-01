// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use cbytes::Bytes;
use ctypes::{Address, H256};

use super::CacheableItem;

#[derive(Clone, Debug, RlpEncodable, RlpDecodable, Serialize, Deserialize)]
pub struct AssetScheme {
    metadata: String,
    lock_script: H256,
    parameters: Vec<Bytes>,
    remainder: u64,
    registrar: Option<Address>,
}

impl AssetScheme {
    pub fn new(
        metadata: String,
        lock_script: H256,
        parameters: Vec<Bytes>,
        remainder: u64,
        registrar: Option<Address>,
    ) -> Self {
        Self {
            metadata,
            lock_script,
            parameters,
            remainder,
            registrar,
        }
    }

    pub fn metadata(&self) -> &String {
        &self.metadata
    }

    pub fn lock_script(&self) -> &H256 {
        &self.lock_script
    }

    pub fn parameters(&self) -> &Vec<Bytes> {
        &self.parameters
    }

    pub fn remainder(&self) -> &u64 {
        &self.remainder
    }

    pub fn release_remainder(&mut self, amount: &u64) {
        debug_assert!(self.remainder >= *amount);
        self.remainder = self.remainder - *amount;
    }

    pub fn registrar(&self) -> &Option<Address> {
        &self.registrar
    }

    pub fn is_permissioned(&self) -> bool {
        self.registrar.is_some()
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AssetSchemeAddress(H256);

const PREFIX: u8 = 'S' as u8;
impl_address!(AssetSchemeAddress, PREFIX);

impl AssetSchemeAddress {
    pub fn new(transaction_hash: H256) -> Self {
        let index = ::std::u64::MAX;

        Self::from_transaction(transaction_hash, index)
    }
}

impl CacheableItem for AssetScheme {
    type Address = AssetSchemeAddress;
    fn overwrite_with(&mut self, other: Self) {
        self.metadata = other.metadata;
        self.lock_script = other.lock_script;
        self.registrar = other.registrar;
        self.remainder = other.remainder;
    }

    fn is_null(&self) -> bool {
        self.remainder == 0
    }

    fn from_rlp(rlp: &[u8]) -> Self {
        ::rlp::decode(rlp)
    }

    fn rlp(&self) -> Bytes {
        ::rlp::encode(self).into_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetSchemeAddress, H256, PREFIX};

    #[test]
    fn asset_from_address() {
        let origin = {
            let mut address;
            loop {
                address = H256::random();
                if address[0] == 'S' as u8 {
                    continue
                }
                for i in 1..8 {
                    if address[i] == 0 {
                        continue
                    }
                }
                break
            }
            address
        };
        let asset_address = AssetSchemeAddress::new(origin);
        let hash: H256 = asset_address.into();
        assert_ne!(origin, hash);
        assert_eq!(hash[0..4], [PREFIX, 0, 0, 0]);
        assert_eq!(hash[4..8], [0, 0, 0, 0]); // world id
    }
}