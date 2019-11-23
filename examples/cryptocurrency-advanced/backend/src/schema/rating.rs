//! Rating.

use super::*;

use super::super::proto;

/// Рейтинг игрока.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Rating", serde_pb_convert)]
pub struct Rating {
    /// Ключ играка.
    pub player_key: PublicKey,

    /// Рейтинг.
    pub value: u64,
}

impl Rating {
    /// Создает голос.
    pub fn new(
        &player_key: &PublicKey,
        value: u64
    ) -> Self
    {
        Self {
            player_key,
            value,
        }
    }
}

impl<T> Schema<T>
where
    T: IndexAccess,
{
    /// Рейтинги играков.
    pub fn ratings(&self) -> ProofMapIndex<T, PublicKey, Rating> {
        ProofMapIndex::new("mwf.ratings", self.access.clone())
    }

    /// Возвращает рейтинг игрока.
    pub fn rating(&self, key: &PublicKey) -> Option<Rating> {
        self.ratings().get(key)
    }

    /// Увеличивает рейтинг игрока.
    pub fn increment_rating(
        &mut self,
        player_key: &PublicKey
    )
    {
        if let Some(rating) = self.rating(player_key) {
            let mut clone = rating.clone();
            clone.value += 1;

            self.ratings().put(player_key, clone);
        } else {
            self.ratings().put(&player_key, Rating::new(
                player_key,
                1 as u64
            ));
        }

    }
}
