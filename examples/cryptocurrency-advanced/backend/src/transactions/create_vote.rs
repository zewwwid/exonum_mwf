//! CreateDuel.

// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use super::*;
//use failure::err_msg;

/// Транзакция голосования.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::CreateVote")]
pub struct CreateVote {
    /// Ключ поединка.
    pub duel_key: PublicKey,

    /// Ключ играка.
    pub player_key: PublicKey,
}

impl CreateVote {
    #[doc(hidden)]
    pub fn sign(
        duel_key: &PublicKey,
        player_key: &PublicKey,
        pk: &PublicKey,
        sk: &SecretKey
    ) -> Signed<RawTransaction>
    {
        Message::sign_transaction(
            Self {
                duel_key: duel_key.to_owned(),
                player_key: player_key.to_owned(),
            },
            CRYPTOCURRENCY_SERVICE_ID,
            *pk,
            sk,
        )
    }
}

impl Transaction for CreateVote {
    fn execute(&self, context: TransactionContext) -> ExecutionResult {
        let judge_key = context.author();
        let hash = context.tx_hash();

        let mut schema = Schema::new(context.fork());

        let duel_key = self.duel_key;
        let player_key = self.player_key;

        // Если поединок существует
        if let Some(duel) = schema.duel(&duel_key) {
            // Проверяем, что игрок, за которого оставлен голос, участвовал в поединке
            if !(duel.player1_key == player_key || duel.player2_key == player_key) {
                Err(Error::PlayerInDuelNotFound)?;
            }

            // Проверяем, что судья судил покдинок
            if !(duel.judge1_key == judge_key || duel.judge2_key == judge_key || duel.judge3_key == judge_key) {
                Err(Error::JudgeInDuelNotFound)?;
            }

            // Проверяем, что судья еще не голосовал в поединке
            for vote in schema.votes().iter() {
                if vote.1.duel_key == duel_key &&  vote.1.judge_key == judge_key {
                    Err(Error::JudgeVoteInDuelAlreadyExists)?;
                }
            }

            if duel.judge1_key == judge_key {

            }

            // Сохраняем голос.
            schema.create_vote(
                &duel_key,
                &judge_key,
                &player_key
            );

            let player_num = if duel.player1_key == player_key {
                1
            } else {
                2
            };

            let judge_num = if duel.judge1_key == judge_key {
                1
            } else if duel.judge2_key == judge_key {
                2
            } else {
                3
            };

            // Добавляем голос к поединку
            schema.add_vote(&duel, player_num, judge_num, &hash);

            let duel = schema.duel(&duel_key).unwrap();
            // Если проголосовали все судьи, то определяем победителя и увеличиваем его рейтинг
            if duel.player1_votes + duel.player2_votes == 3 {
                // Если победил игрок 1
                if duel.player1_votes > duel.player2_votes {
                    schema.increment_rating(&duel.player1_key);
                }
                // Если победил игрок 2
                else {
                    schema.increment_rating(&duel.player2_key);
                }
            }

            Ok(())

        } else {
            Err(Error::DuelNotFound)?
        }
    }
}
