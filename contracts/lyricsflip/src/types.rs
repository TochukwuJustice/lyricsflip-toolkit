use soroban_sdk::{contracttype, Address, BytesN, String};

/// Explicit discriminants make this a "simple" contracttype enum, which
/// soroban-sdk encodes as a plain `u32` (and the JS SDK decodes to a plain
/// `number`) rather than the `{tag, values}` shape used for enums with
/// associated data (see `Answer` below). Keep `frontend/src/lib/stellar/types.ts`'s
/// `GENRE_VALUES` ordering in sync with these values.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Genre {
    HipHop = 0,
    Pop = 1,
    Rock = 2,
    RnB = 3,
    Electronic = 4,
    Classical = 5,
    Jazz = 6,
    Country = 7,
    Blues = 8,
    Reggae = 9,
    Afrobeat = 10,
    Gospel = 11,
    Folk = 12,
}

impl Genre {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(Genre::HipHop),
            1 => Some(Genre::Pop),
            2 => Some(Genre::Rock),
            3 => Some(Genre::RnB),
            4 => Some(Genre::Electronic),
            5 => Some(Genre::Classical),
            6 => Some(Genre::Jazz),
            7 => Some(Genre::Country),
            8 => Some(Genre::Blues),
            9 => Some(Genre::Reggae),
            10 => Some(Genre::Afrobeat),
            11 => Some(Genre::Gospel),
            12 => Some(Genre::Folk),
            _ => None,
        }
    }
}

/// Ported from `onchain/src/utils/types.cairo`. `felt252`/`ByteArray` fields
/// become `String`; Starknet's `u8` fields become `u32` since Soroban has no
/// native u8 contract type.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub card_id: u64,
    pub genre: Genre,
    pub artist: String,
    pub title: String,
    pub year: u64,
    pub difficulty: u32,
    pub lyrics: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyCard {
    pub card_id: u64,
    pub genre: Genre,
    pub artist: String,
    pub title: String,
    pub year: u64,
    pub lyrics: String,
}

impl From<LegacyCard> for Card {
    fn from(card: LegacyCard) -> Self {
        Self {
            card_id: card.card_id,
            genre: card.genre,
            artist: card.artist,
            title: card.title,
            year: card.year,
            difficulty: 2,
            lyrics: card.lyrics,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundFilter {
    pub genre: Option<Genre>,
    /// The first year of a decade, for example 1990.
    pub decade: Option<u64>,
}

impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for RoundFilter {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &soroban_sdk::Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        use soroban_sdk::{symbol_short, Map, Symbol};
        let map: Map<Symbol, soroban_sdk::Val> = Map::try_from_val(env, val)?;
        let decade: Option<u64> = match map.get(symbol_short!("decade")) {
            Some(v) => Some(u64::try_from_val(env, &v)?),
            None => None,
        };
        let genre: Option<Genre> = match map.get(symbol_short!("genre")) {
            Some(v) => Some(Genre::try_from_val(env, &v)?),
            None => None,
        };
        Ok(RoundFilter { genre, decade })
    }
}

impl soroban_sdk::TryFromVal<soroban_sdk::Env, RoundFilter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &soroban_sdk::Env, filter: &RoundFilter) -> Result<Self, Self::Error> {
        use soroban_sdk::{symbol_short, Map, Symbol, TryIntoVal};
        let mut map: Map<Symbol, soroban_sdk::Val> = Map::new(env);
        if let Some(decade) = filter.decade {
            map.set(symbol_short!("decade"), decade.try_into_val(env)?);
        }
        if let Some(genre) = filter.genre {
            map.set(symbol_short!("genre"), genre.try_into_val(env)?);
        }
        Ok(map.into())
    }
}

impl soroban_sdk::TryFromVal<soroban_sdk::Env, &RoundFilter> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &soroban_sdk::Env, filter: &&RoundFilter) -> Result<Self, Self::Error> {
        soroban_sdk::Val::try_from_val(env, *filter)
    }
}

#[cfg(any(test, feature = "testutils"))]
impl TryFrom<&RoundFilter> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    fn try_from(filter: &RoundFilter) -> Result<Self, soroban_sdk::xdr::Error> {
        extern crate std;
        use soroban_sdk::xdr::{ScMap, ScMapEntry, ScSymbol, ScVal};
        let mut entries = std::vec::Vec::new();
        if let Some(decade) = filter.decade {
            let sym = ScSymbol(
                "decade"
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            );
            entries.push(ScMapEntry {
                key: ScVal::Symbol(sym),
                val: ScVal::U64(decade),
            });
        }
        if let Some(genre) = filter.genre {
            let sym = ScSymbol(
                "genre"
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            );
            entries.push(ScMapEntry {
                key: ScVal::Symbol(sym),
                val: ScVal::U32(genre as u32),
            });
        }
        let map_m = entries
            .try_into()
            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
        Ok(ScVal::Map(Some(ScMap(map_m))))
    }
}

#[cfg(any(test, feature = "testutils"))]
impl TryFrom<RoundFilter> for soroban_sdk::xdr::ScVal {
    type Error = soroban_sdk::xdr::Error;
    fn try_from(filter: RoundFilter) -> Result<Self, soroban_sdk::xdr::Error> {
        soroban_sdk::xdr::ScVal::try_from(&filter)
    }
}

#[cfg(any(test, feature = "testutils"))]
impl TryFrom<&soroban_sdk::xdr::ScVal> for RoundFilter {
    type Error = soroban_sdk::xdr::Error;
    fn try_from(val: &soroban_sdk::xdr::ScVal) -> Result<Self, soroban_sdk::xdr::Error> {
        use soroban_sdk::xdr::ScVal;
        match val {
            ScVal::Map(Some(map)) => {
                let mut genre = None;
                let mut decade = None;
                for entry in map.0.iter() {
                    if let ScVal::Symbol(sym) = &entry.key {
                        if sym.0.as_slice() == b"genre" {
                            if let ScVal::U32(g) = entry.val {
                                genre = Genre::from_u32(g);
                            }
                        } else if sym.0.as_slice() == b"decade" {
                            if let ScVal::U64(d) = entry.val {
                                decade = Some(d);
                            }
                        }
                    }
                }
                Ok(RoundFilter { genre, decade })
            }
            _ => Err(soroban_sdk::xdr::Error::Invalid),
        }
    }
}

#[cfg(any(test, feature = "testutils"))]
impl TryFrom<soroban_sdk::xdr::ScVal> for RoundFilter {
    type Error = soroban_sdk::xdr::Error;
    fn try_from(val: soroban_sdk::xdr::ScVal) -> Result<Self, soroban_sdk::xdr::Error> {
        RoundFilter::try_from(&val)
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerStats {
    pub total_rounds: u64,
    pub rounds_won: u64,
    pub current_streak: u64,
    pub max_streak: u64,
}

impl PlayerStats {
    pub fn zero() -> Self {
        PlayerStats {
            total_rounds: 0,
            rounds_won: 0,
            current_streak: 0,
            max_streak: 0,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Round {
    pub round_id: u64,
    pub admin: Address,
    pub genre: Genre,
    pub wager_amount: i128,
    pub start_time: u64,
    pub is_started: bool,
    pub is_completed: bool,
    pub end_time: u64,
    pub next_card_index: u32,
    pub is_cancelled: bool,
    /// Snapshotted card answer window in seconds (LF-158).
    pub answer_window: u64,
    /// Snapshotted lobby timeout in seconds (LF-158).
    pub lobby_timeout: u64,
    /// Snapshotted round duration in seconds (LF-158).
    pub duration: u64,
}

/// NFT reward milestones a player can claim once each via `claim_reward`.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Milestone {
    /// At least one round won.
    FirstWin = 0,
    /// A streak of at least 5 correct answers.
    Streak5 = 1,
    /// At least 10 rounds won.
    TenWins = 2,
}

/// Specifies what the player is asked to identify in a `QuestionCard`.
/// Using explicit discriminants keeps the on-chain ABI stable and makes the
/// JS SDK decode to a plain `number` (same encoding rule as `Genre`/`Role`).
/// Keep `frontend/src/lib/stellar/types.ts`'s `QUESTION_KIND_VALUES` in sync.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuestionKind {
    Title = 0,
    Artist = 1,
    Year = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuestionCard {
    pub lyric: String,
    pub timestamp: u64,
    /// Which attribute the options represent (Title / Artist / Year).
    pub kind: QuestionKind,
    pub option_one: String,
    pub option_two: String,
    pub option_three: String,
    pub option_four: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Answer {
    Artist(String),
    Year(u64),
    Title(String),
}

/// Position of a card in the global, genre, artist and year indexes.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPos {
    pub all: u32,
    pub genre: u32,
    pub artist: u32,
    pub year: u32,
}

/// Mirrors Cairo's single `ADMIN_ROLE` selector. Kept as an enum (instead of
/// dropping the parameter entirely) so `set_role`/`is_admin` keep the same
/// call shape as the original `ILyricsFlip` interface.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    Admin = 0,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    /// Nominated by `transfer_ownership`, pending `accept_ownership`.
    PendingOwner,
    Admin(Address),
    RoundCount,
    CardsCount,
    CardsPerRound,
    Card(u64),
    /// Last card id handed out. Ids are never reused, so this differs from
    /// `CardsCount` (the number of live cards) once cards are removed.
    LastCardId,
    /// sha256(title, 0x00, artist) -> card id. Rejects duplicate cards.
    CardKey(BytesN<32>),
    /// Where a card sits in each of the index lists below (for O(1) removal).
    CardPos(u64),
    /// Indexes are stored as a count plus one entry per item, so adding a
    /// card costs the same no matter how large the index already is.
    CardAt(u32),
    GenreCardCount(Genre),
    GenreCardAt((Genre, u32)),
    ArtistCardCount(String),
    ArtistCardAt((String, u32)),
    YearCardCount(u64),
    YearCardAt((u64, u32)),
    Round(u64),
    RoundPlayers(u64),
    RoundCards(u64),
    RoundScores(u64),
    RoundAnswerTimes(u64),
    RoundCardStartedAt((u64, u64)),
    PlayerCardStartedAt((u64, Address, u64)),
    PlayerCardIndex((u64, Address)),
    RoundPlayerAnswered((u64, Address, u64)),
    RoundFinalized(u64),
    PlayerStats(Address),
    RoundReady((u64, Address)),
    RoundReadyCount(u64),
    /// Ids of rounds that have been created but not yet started, in creation
    /// order. Backs the multiplayer lobby's `get_open_rounds` view.
    OpenRounds,
    /// Global cap on players per round (owner-configurable).
    MaxPlayers,
    /// Ledger timestamp at which a round was created; drives the lobby timeout.
    RoundCreatedAt(u64),
    /// Address of the `lyricsflip-nft` contract used to mint rewards.
    NftContract,
    /// Whether a player has already claimed a milestone reward.
    MilestoneClaimed((Address, Milestone)),
    /// Global emergency-pause flag. While set, round creation/joining and
    /// reward claims are blocked; refund and cancel paths stay open.
    Paused,
    /// Stored storage schema version (LF-157).
    SchemaVersion,
    /// Global card answer window in seconds (LF-158).
    CardAnswerWindow,
    /// Global lobby timeout in seconds (LF-158).
    LobbyTimeout,
    /// Global round duration override in seconds (LF-158). 0 = dynamic scaling.
    RoundDuration,
    /// Global per-card latency buffer in seconds for dynamic deadline calculation (LF-158).
    RoundBuffer,
}
