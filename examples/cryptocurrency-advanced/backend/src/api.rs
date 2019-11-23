// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cryptocurrency API.

use exonum_merkledb::{ListProof, MapProof};

use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{self, BlockProof, TransactionMessage},
    crypto::{Hash, PublicKey},
    explorer::BlockchainExplorer,
    helpers::Height,
};

use crate::{wallet::Wallet, Schema, CRYPTOCURRENCY_SERVICE_ID};

/// Describes the query parameters for the `get_wallet` endpoint.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct WalletQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

/// Proof of existence for specific wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletProof {
    /// Proof of the whole database table.
    pub to_table: MapProof<Hash, Hash>,
    /// Proof of the specific wallet in this table.
    pub to_wallet: MapProof<PublicKey, Wallet>,
}

/// Wallet history.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletHistory {
    /// Proof of the list of transaction hashes.
    pub proof: ListProof<Hash>,
    /// List of above transactions.
    pub transactions: Vec<TransactionMessage>,
}

/// Wallet information.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Proof of the last block.
    pub block_proof: BlockProof,
    /// Proof of the appropriate wallet.
    pub wallet_proof: WalletProof,
    /// History of the appropriate wallet.
    pub wallet_history: Option<WalletHistory>,
}



/// Запрос информации о поединке.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DuelQuery {
    /// Ключ поединка.
    pub key: PublicKey,
}

/// Запрос поединков судьи в которых он еще не проголосовал.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct JudgeDuelsQuery {
    /// Ключ судьи.
    pub judge_key: PublicKey,
}

/// Запрос рейтинга игрока.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RatingQuery {
    /// Ключ игрока.
    pub player_key: PublicKey,
}

/// Информаци по поединку.
#[derive(Debug, Serialize, Deserialize)]
pub struct DuelInfo {
    /// Ключ поединка.
    pub key: PublicKey,

    /// Ключ арбитра.
    pub arbiter_key: PublicKey,

    /// Ключ играка 1.
    pub player1_key: PublicKey,
    /// Ключ играка 2.
    pub player2_key: PublicKey,

    /// Ключ судьи 1.
    pub judge1_key: PublicKey,
    /// Ключ судьи 2.
    pub judge2_key: PublicKey,
    /// Ключ судьи 3.
    pub judge3_key: PublicKey,

    /// Номер ситуации.
    pub situation_number: u64,

    /// Количество голосов за первого игрока.
    pub player1_votes: u64,

    /// Количество голосов за второго игрока.
    pub player2_votes: u64,
}

/// Рейтинг.
#[derive(Debug, Serialize, Deserialize)]
pub struct RatingInfo {
    /// Рейтинг.
    pub value: u64,
}




/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {
    /// Endpoint for getting a single wallet.
    pub fn wallet_info(state: &ServiceApiState, query: WalletQuery) -> api::Result<WalletInfo> {
        let snapshot = state.snapshot();
        let general_schema = blockchain::Schema::new(&snapshot);
        let currency_schema = Schema::new(&snapshot);

        let max_height = general_schema.block_hashes_by_height().len() - 1;

        let block_proof = general_schema
            .block_and_precommits(Height(max_height))
            .unwrap();

        let to_table: MapProof<Hash, Hash> =
            general_schema.get_proof_to_service_table(CRYPTOCURRENCY_SERVICE_ID, 0);

        let to_wallet: MapProof<PublicKey, Wallet> =
            currency_schema.wallets().get_proof(query.pub_key);

        let wallet_proof = WalletProof {
            to_table,
            to_wallet,
        };

        let wallet = currency_schema.wallet(&query.pub_key);

        let explorer = BlockchainExplorer::new(state.blockchain());

        let wallet_history = wallet.map(|_| {
            let history = currency_schema.wallet_history(&query.pub_key);
            let proof = history.get_range_proof(0..history.len());

            let transactions = history
                .iter()
                .map(|record| explorer.transaction_without_proof(&record).unwrap())
                .collect::<Vec<_>>();

            WalletHistory {
                proof,
                transactions,
            }
        });

        Ok(WalletInfo {
            block_proof,
            wallet_proof,
            wallet_history,
        })
    }

    /// Возвращает информацию о поединке.
    pub fn duel_info(state: &ServiceApiState, query: DuelQuery) -> api::Result<DuelInfo> {
        let snapshot = state.snapshot();
        let currency_schema = Schema::new(&snapshot);

        if let Some(duel) = currency_schema.duel(&query.key) {
            Ok(DuelInfo {
                key: duel.key,
                arbiter_key: duel.arbiter_key,
                player1_key: duel.player1_key,
                player2_key: duel.player2_key,
                judge1_key: duel.judge1_key,
                judge2_key: duel.judge2_key,
                judge3_key: duel.judge3_key,
                situation_number: duel.situation_number,
                player1_votes: duel.player1_votes,
                player2_votes: duel.player2_votes,
            })
        } else {
            Err(api::Error::NotFound(String::from("duel")))
        }
    }

    /// Возвращает поединке судьи в которых он еще не проголосовал.
    pub fn judge_duels(state: &ServiceApiState, query: JudgeDuelsQuery) -> api::Result<Vec<DuelInfo>> {
        let snapshot = state.snapshot();
        let currency_schema = Schema::new(&snapshot);

        // Получаем ключи поединков в которых судья голосовал
        let exclude_duel_keys = currency_schema.votes().iter()
            .filter(|x| x.1.judge_key == query.judge_key)
            .map(|x| x.1.duel_key)
            .collect::<Vec<_>>();

        // Получаем поединки в которых участвовал судья и еще не проголосовал
        let duels = currency_schema.duels().iter()
            .filter(|x| (
                x.1.judge1_key == query.judge_key ||
                    x.1.judge2_key == query.judge_key ||
                    x.1.judge3_key == query.judge_key) && !exclude_duel_keys.contains(&x.1.key))
            .map(|x| DuelInfo {
                key: x.1.key,
                arbiter_key: x.1.arbiter_key,
                player1_key: x.1.player1_key,
                player2_key: x.1.player2_key,
                judge1_key: x.1.judge1_key,
                judge2_key: x.1.judge2_key,
                judge3_key: x.1.judge3_key,
                situation_number: x.1.situation_number,
                player1_votes: x.1.player1_votes,
                player2_votes: x.1.player2_votes,
            })
            .collect::<Vec<_>>();

        Ok(duels)
    }

    /// Возвращает рейтинг игрока.
    pub fn rating(state: &ServiceApiState, query: RatingQuery) -> api::Result<RatingInfo> {
        let snapshot = state.snapshot();
        let currency_schema = Schema::new(&snapshot);

        if let Some(rating) = currency_schema.rating(&query.player_key) {
            Ok(RatingInfo {
                value: rating.value,
            })
        } else {
            Ok(RatingInfo {
                value: 0,
            })
        }
    }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/wallets/info", Self::wallet_info)
            .endpoint("v1/duel/info", Self::duel_info)
            .endpoint("v1/judge/duels", Self::judge_duels)
            .endpoint("v1/rating", Self::rating);
    }
}
