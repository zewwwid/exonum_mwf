//! Vote.

use super::*;

use super::super::proto;

/// Голос.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Vote", serde_pb_convert)]
pub struct Vote {
    /// Ключ поединка.
    pub duel_key: PublicKey,

    /// Ключ судьи.
    pub judge_key: PublicKey,

    /// Ключ играка.
    pub player_key: PublicKey,
}

impl Vote {
    /// Создает голос.
    pub fn new(
        &duel_key: &PublicKey,
        &judge_key: &PublicKey,
        &player_key: &PublicKey,
    ) -> Self
    {
        Self {
            duel_key,
            judge_key,
            player_key,
        }
    }
}

impl<T> Schema<T>
where
    T: IndexAccess,
{
    /// Возвращает голоса в поединке.
    pub fn duel_votes(&self) -> ProofMapIndex<T, PublicKey, Vote> {
        ProofMapIndex::new("mwf.duel_votes", self.access.clone())
    }

    /// Возвращает голоса судьи.
    pub fn judge_votes(&self) -> ProofMapIndex<T, PublicKey, Vote> {
        ProofMapIndex::new("mwf.judge_votes", self.access.clone())
    }

    /// Возвращает голоса за игрока.
    pub fn player_votes(&self) -> ProofMapIndex<T, PublicKey, Vote> {
        ProofMapIndex::new("mwf.player_votes", self.access.clone())
    }

    /// Создает голос.
    pub fn create_vote(
        &mut self,
        duel_key: &PublicKey,
        judge_key: &PublicKey,
        player_key: &PublicKey
    )
    {
        let vote = {
            Vote::new(
                duel_key,
                judge_key,
                player_key
            )
        };
        self.duel_votes().put(duel_key, vote.clone());
        self.judge_votes().put(judge_key, vote.clone());
        self.player_votes().put(player_key, vote.clone());
    }
}
