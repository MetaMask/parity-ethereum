// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use v1::types::{Bytes, H256};
use std::collections::BTreeMap;

/// Kitsunet Slice representation
#[derive(Debug, Serialize)]
#[serde(rename_all="kebab-case")]
pub struct KitsunetSlice {
	/// Id of the slice <path>-<max_depth>-<trie_root>
	pub slice_id: String,
	/// Metadata of the slice, time and number of nodes
	pub metadata: Option<KitsunetSliceMetadata>,
	/// Trie nodes of the slice
	pub trie_nodes: Option<KitsunetSliceNodes>,
	/// If the slice is a state one (as opposed of storage),
	/// we return the leaves' storage root and EVM code of
	/// the smart contracts
	pub leaves: Option<BTreeMap<H256, KitsunetSmartContract>>
}

/// Kitsunet slice metadata representation.
/// Time in milliseconds and number of trie nodes
#[derive(Debug, Serialize)]
#[serde(rename_all="kebab-case")]
pub struct KitsunetSliceMetadata {
	/// Time of different operations in milliseconds
	pub time_ms: BTreeMap<String, String>,
	/// Number of nodes on each subset: stem and head, trie nodes,
	/// max depth found, leaves and smart contracts (if it is a state slice)
	pub nodes_number: BTreeMap<String, String>,
}

/// Kitsunet slice trie nodes representation.
/// Stem (including the trie root), head of the slice,
/// and the slice nodes
#[derive(Debug, Serialize)]
#[serde(rename_all="kebab-case")]
pub struct KitsunetSliceNodes {
	/// Trie nodes from the root (including it) to the
	/// slice head (excluding it)
	pub stem: BTreeMap<H256, Bytes>,
	/// Head of the slice
	pub head: BTreeMap<H256, Bytes>,
	/// Trie nodes of the slice
	pub slice: Option<BTreeMap<H256, Bytes>>,
}

/// Kitsunet smart contract, if the slice is a state one
/// (as opposed of storage), we return the leaves' storage root
/// and EVM code of the smart contracts
#[derive(Debug, Serialize)]
#[serde(rename_all="kebab-case")]
pub struct KitsunetSmartContract {
	/// Storage root of the smart contract
	pub storage_root: H256,
	/// EVM code of the smart contract
	pub evm_code: Bytes,
}

#[cfg(test)]
mod tests {
	use rustc_hex::FromHex;
	use serde_json;
	use std::collections::BTreeMap;
	use std::str::FromStr;
	use super::{KitsunetSlice, KitsunetSliceMetadata, KitsunetSliceNodes, KitsunetSmartContract};
	use v1::types::{Bytes, H256};

	#[test]
	fn test_serialize_kitsunet_slice() {
		let kitsunet_slice = KitsunetSlice{
			slice_id: "1372-10-8e9e2d4514bee72525133350201ecfc3da3815a4bb6b03ad5ff3bdd555363188".into(),
			metadata: Some(KitsunetSliceMetadata{
				time_ms: map![
					"00 trie-loading".into() => "0.015000".into(),
					"01 fetch-stem-keys".into() => "0.084504".into(),
					"02 fetch-slice-keys".into() => "177.019430".into(),
					"03 fetch-leaves-info".into() => "1104.785629".into()
				],
				nodes_number: map![
					"N00 stem-and-head-nodes".into() => "5".into(),
					"N01 max-depth".into() => "7".into(),
					"N02 total-trie-nodes".into() => "1022".into(),
					"N03 leaves".into() => "739".into(),
					"N04 smart-contacts".into() => "122".into()
				],
			}),
			trie_nodes: Some(KitsunetSliceNodes{
				stem: map![
					H256::from_str("49a2b2245809d0557c13e70b743d77820147ad2806fa96c7ae2442b9ec075fe2").unwrap() =>
						Bytes("f90211a0d3679ab92133318a2448a06929746b777145419b8c861a7e50356a".from_hex().unwrap()),
					H256::from_str("8e9e2d4514bee72525133350201ecfc3da3815a4bb6b03ad5ff3bdd555363188").unwrap() =>
						Bytes("f90211a0ddcc9851a7e66913088afa21a5a6246438616d0d32ff4a5a39542b".from_hex().unwrap()),
					H256::from_str("b40284394e2e20fde4426b1a31efdff3155ba0a30bc4b5767f82ac28c5fcb61d").unwrap() =>
						Bytes("f90211a06cefd44c061cc5347291252df405c98dbf17d3b7c9af92521b9d0c".from_hex().unwrap())
				],
				head: map![
					H256::from_str("7c265eff0de2be90d104198753ebe3591538eaea49bb3ca2005d2de212593dd4").unwrap() =>
						Bytes("f90211a0aec14f94a5b41e071d79bc9d8e652bf6e62f43087e7ceb63e5e29d".from_hex().unwrap())
				],
				slice: Some(map![
					H256::from_str("00320959ed3f2b59e417bc0ee4dd6849dd84e88de42183fd9b73793e2c8dcebb").unwrap() =>
						Bytes("f87180a0a6d162c2da0594b9f08fceabf5f42501a6a12f59e96c1e38332b1a".from_hex().unwrap()),
					H256::from_str("003abd40edace0a07b110cc5920700f9c7a32c5f1c418d89e52e99ef49f4a721").unwrap() =>
						Bytes("f851808080a02367d13dca8c2cd674625b7fa7566e508539eac966ea31d5ae".from_hex().unwrap()),
					H256::from_str("003de42b4903bc596bb9d496520ddb8391c5ede58c6387517107debcdcadf9aa").unwrap() =>
						Bytes("f86c9d3a04863fdd15efc7d1285c4ee41272081141db7ee989039695547058".from_hex().unwrap())
				]),
			}),
			leaves: Some(map![
				H256::from_str("13720e771220eac4201e1ee88c325f335368e2fdbc70e8fbcfca7d3f5658dac6").unwrap() =>
					KitsunetSmartContract{
						storage_root: H256::from_str("8a7e2341281de0a8844a76ae48939449b36cc23f69694f27cc988a1be34efe7c").unwrap(),
						evm_code: Bytes("60606040525b603c5b60006010603e565b90505936810160405236600082376020".from_hex().unwrap()),
					},

				H256::from_str("1372159a8c7294b99cfe7c8ea80c19037f357a064c453636a234f103de73268e").unwrap() =>
					KitsunetSmartContract{
						storage_root: H256::from_str("e7b9307faaf6686a9d9220d832cd4b2bdf1bcf6a1fae4d2603a884e8f3f20713").unwrap(),
						evm_code: Bytes("606060405236156100885763ffffffff60e060020a60003504166302".from_hex().unwrap()),
					},

				H256::from_str("1372162c80de9c254a7a5e934606411789257fb6f1533af414415f1638f850c3").unwrap() =>
					KitsunetSmartContract{
						storage_root: H256::from_str("88f6c90814631203ebcd879b9b2a01f894d317d77528d0a411dcdc9664a288fb").unwrap(),
						evm_code: Bytes("606060405263ffffffff60e060020a6000350416636ea056a9811461".from_hex().unwrap()),
					}
			]),
		};

		let serialized_kitsunet_slice = serde_json::to_string(&kitsunet_slice).unwrap();

		assert_eq!(serialized_kitsunet_slice, r#"{"slice-id":"1372-10-8e9e2d4514bee72525133350201ecfc3da3815a4bb6b03ad5ff3bdd555363188","metadata":{"time-ms":{"00 trie-loading":"0.015000","01 fetch-stem-keys":"0.084504","02 fetch-slice-keys":"177.019430","03 fetch-leaves-info":"1104.785629"},"nodes-number":{"N00 stem-and-head-nodes":"5","N01 max-depth":"7","N02 total-trie-nodes":"1022","N03 leaves":"739","N04 smart-contacts":"122"}},"trie-nodes":{"stem":{"0x49a2b2245809d0557c13e70b743d77820147ad2806fa96c7ae2442b9ec075fe2":"0xf90211a0d3679ab92133318a2448a06929746b777145419b8c861a7e50356a","0x8e9e2d4514bee72525133350201ecfc3da3815a4bb6b03ad5ff3bdd555363188":"0xf90211a0ddcc9851a7e66913088afa21a5a6246438616d0d32ff4a5a39542b","0xb40284394e2e20fde4426b1a31efdff3155ba0a30bc4b5767f82ac28c5fcb61d":"0xf90211a06cefd44c061cc5347291252df405c98dbf17d3b7c9af92521b9d0c"},"head":{"0x7c265eff0de2be90d104198753ebe3591538eaea49bb3ca2005d2de212593dd4":"0xf90211a0aec14f94a5b41e071d79bc9d8e652bf6e62f43087e7ceb63e5e29d"},"slice":{"0x00320959ed3f2b59e417bc0ee4dd6849dd84e88de42183fd9b73793e2c8dcebb":"0xf87180a0a6d162c2da0594b9f08fceabf5f42501a6a12f59e96c1e38332b1a","0x003abd40edace0a07b110cc5920700f9c7a32c5f1c418d89e52e99ef49f4a721":"0xf851808080a02367d13dca8c2cd674625b7fa7566e508539eac966ea31d5ae","0x003de42b4903bc596bb9d496520ddb8391c5ede58c6387517107debcdcadf9aa":"0xf86c9d3a04863fdd15efc7d1285c4ee41272081141db7ee989039695547058"}},"leaves":{"0x13720e771220eac4201e1ee88c325f335368e2fdbc70e8fbcfca7d3f5658dac6":{"storage-root":"0x8a7e2341281de0a8844a76ae48939449b36cc23f69694f27cc988a1be34efe7c","evm-code":"0x60606040525b603c5b60006010603e565b90505936810160405236600082376020"},"0x1372159a8c7294b99cfe7c8ea80c19037f357a064c453636a234f103de73268e":{"storage-root":"0xe7b9307faaf6686a9d9220d832cd4b2bdf1bcf6a1fae4d2603a884e8f3f20713","evm-code":"0x606060405236156100885763ffffffff60e060020a60003504166302"},"0x1372162c80de9c254a7a5e934606411789257fb6f1533af414415f1638f850c3":{"storage-root":"0x88f6c90814631203ebcd879b9b2a01f894d317d77528d0a411dcdc9664a288fb","evm-code":"0x606060405263ffffffff60e060020a6000350416636ea056a9811461"}}}"#);
	}
}
