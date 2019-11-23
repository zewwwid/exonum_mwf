//! Vote.

use super::*;

use super::super::proto;

/// Голос.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Vote", serde_pb_convert)]
pub struct Vote {
    /// Ключ голоса.
    pub key: PublicKey,

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
        let (key, _) = exonum::crypto::gen_keypair();

        Self {
            key,
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
    /// Возвращает голоса.
    pub fn votes(&self) -> ProofMapIndex<T, PublicKey, Vote> {
        ProofMapIndex::new("mwf.votes", self.access.clone())
    }

    /// Создает голос.
    pub fn create_vote(
        &mut self,
        duel_key: &PublicKey,
        judge_key: &PublicKey,
        player_key: &PublicKey
    )
    {
        let vote = Vote::new(
            duel_key,
            judge_key,
            player_key
        );

        self.votes().put(&vote.key, vote.clone());
    }
}
