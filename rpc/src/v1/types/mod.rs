// Copyright 2018-2020 Kodebox, Inc.
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

mod action;
mod block;
mod mem_pool;
mod transaction;
mod unsigned_transaction;
mod work;

pub use self::action::{Action, ActionWithTracker};
pub use self::block::Block;
pub use self::block::BlockNumberAndHash;
pub use self::mem_pool::MemPoolMinFees;
pub use self::transaction::{PendingTransactions, Transaction};
pub use self::unsigned_transaction::UnsignedTransaction;
pub use self::work::Work;

use ctypes::TxHash;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterStatus {
    pub list: Vec<(::cidr::IpCidr, String)>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendTransactionResult {
    pub hash: TxHash,
    pub seq: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TPSTestSetting {
    pub count: u64,
    pub seed: u64,
}
