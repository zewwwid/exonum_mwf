//! Duel.

use super::*;

use super::super::proto;

/// Поединок.
#[derive(Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::Duel", serde_pb_convert)]
pub struct Duel {
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

    /// Имя игрока 1.
    pub player1_name: String,

    /// Имя игрока 2.
    pub player2_name: String,

    /// Судья 1 проголосовал.
    pub judge1_voted: bool,

    /// Судья 2 проголосовал.
    pub judge2_voted: bool,

    /// Судья 3 проголосовал.
    pub judge3_voted: bool,

    /// Количество транзакция связанных с поединком.
    pub history_len: u64,

    /// `Hash` of the transactions history.
    pub history_hash: Hash,
}

impl Duel {
    /// Создает поединок.
    pub fn new(
        &key: &PublicKey,
        &arbiter_key: &PublicKey,
        &player1_key: &PublicKey,
        &player2_key: &PublicKey,
        &judge1_key: &PublicKey,
        &judge2_key: &PublicKey,
        &judge3_key: &PublicKey,
        player1_name: String,
        player2_name: String,
        situation_number: u64,
        history_len: u64,
        &history_hash: &Hash,
    ) -> Self
    {
        Self {
            key,
            arbiter_key,
            player1_key,
            player2_key,
            judge1_key,
            judge2_key,
            judge3_key,
            situation_number,
            player1_votes: 0 as u64,
            player2_votes: 0 as u64,
            player1_name,
            player2_name,
            judge1_voted: false,
            judge2_voted: false,
            judge3_voted: false,
            history_len,
            history_hash,
        }
    }
}

impl<T> Schema<T>
where
    T: IndexAccess,
{
    /// Возвращает поединки.
    pub fn duels(&self) -> ProofMapIndex<T, PublicKey, Duel> {
        ProofMapIndex::new("mwf.duels", self.access.clone())
    }

    /// Возвращает поединок по ключу.
    pub fn duel(&self, key: &PublicKey) -> Option<Duel> {
        self.duels().get(key)
    }

    /// Возвращает историю по поединку.
    pub fn duel_history(&self, public_key: &PublicKey) -> ProofListIndex<T, Hash> {
        ProofListIndex::new_in_family(
            "mwf.duel_history",
            public_key,
            self.access.clone(),
        )
    }

    /// Создает поединок.
    pub fn create_duel(
        &mut self,
        key: &PublicKey,
        arbiter_key: &PublicKey,
        player1_key: &PublicKey,
        player2_key: &PublicKey,
        judge1_key: &PublicKey,
        judge2_key: &PublicKey,
        judge3_key: &PublicKey,
        situation_number: u64,
        transaction: &Hash
    )
    {
        let duel = {
            let mut history = self.duel_history(key);
            history.push(*transaction);
            let history_hash = history.object_hash();

            let mut player1_name: String = String::from("");
            let mut player2_name: String = String::from("");

            if let Some(player1) = self.wallet(player1_key) {
                player1_name = player1.name.clone();
            }

            if let Some(player2) = self.wallet(player2_key) {
                player2_name = player2.name.clone();
            }

            Duel::new(
                key,
                arbiter_key,
                player1_key,
                player2_key,
                judge1_key,
                judge2_key,
                judge3_key,
                player1_name,
                player2_name,
                situation_number,
                history.len(),
                &history_hash)
        };
        self.duels().put(key, duel);
    }

//    /// Добавляет голос за первого игрока.
//    pub fn add_player1_vote(&self, duel: &Duel, transaction: &Hash) {
//        let mut history = self.duel_history(&duel.key);
//        history.push(*transaction);
//        let history_hash = history.object_hash();
//
//        let mut clone = duel.clone();
//        clone.player1_votes = clone.player1_votes + 1;
//        clone.history_len = clone.history_len + 1;
//        clone.history_hash = history_hash;
//
//        self.duels().put(&clone.key, clone.clone());
//    }
//
//    /// Добавляет голос за второго игрока.
//    pub fn add_player2_vote(&self, duel: &Duel, transaction: &Hash) {
//        let mut history = self.duel_history(&duel.key);
//        history.push(*transaction);
//        let history_hash = history.object_hash();
//
//        let mut clone = duel.clone();
//        clone.player2_votes = clone.player2_votes + 1;
//        clone.history_len = clone.history_len + 1;
//        clone.history_hash = history_hash;
//
//        self.duels().put(&clone.key, clone.clone());
//    }

    /// Добавляет голос к поединку.
    pub fn add_vote(&self, duel: &Duel, player_num: i32, judge_num: i32, transaction: &Hash) {
        let mut history = self.duel_history(&duel.key);
        history.push(*transaction);
        let history_hash = history.object_hash();

        let mut clone = duel.clone();

        if player_num == 1 {
            clone.player1_votes = clone.player1_votes + 1;
        } else {
            clone.player2_votes = clone.player2_votes + 1;
        }

        if judge_num == 1 {
            clone.judge1_voted = true;
        }

        if judge_num == 2 {
            clone.judge2_voted = true;
        }

        if judge_num == 3 {
            clone.judge3_voted = true;
        }

        clone.history_len = clone.history_len + 1;
        clone.history_hash = history_hash;

        self.duels().put(&clone.key, clone.clone());
    }
}
