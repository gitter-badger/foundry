// Copyright 2018-2019 Kodebox, Inc.
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

use crate::uint::Uint;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NullEngineParams {
    /// Block reward.
    pub block_reward: Option<Uint>,
}

/// Null engine descriptor
#[derive(Debug, PartialEq, Deserialize)]
pub struct NullEngine {
    pub params: NullEngineParams,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn null_engine_deserialization() {
        let s = r#"{
            "params": {
                "blockReward": "0x0d"
            }
        }"#;

        let deserialized: NullEngine = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized.params.block_reward, Some(0x0d.into()));
    }
}
