#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::sync::Mutex;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use rusqlite::Connection;

use crate::game::{
    score::{ScoreHistory, ScoreRecord},
    settings::GameSettings,
};

#[cfg(not(target_arch = "wasm32"))]
/// Wraps a SQLite connection as a Bevy resource.
///
/// `Connection` is not `Sync`, so we wrap it in a `Mutex`.
#[derive(Resource)]
pub struct Database {
    conn: Mutex<Connection>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct MemoryDatabase {
    settings: GameSettings,
    scores: ScoreHistory,
}

#[cfg(target_arch = "wasm32")]
/// In the browser we keep settings and score history in memory only.
#[derive(Resource, Default)]
pub struct Database {
    state: Mutex<MemoryDatabase>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Database {
    /// Open (or create) the database in the platform data directory.
    ///
    /// Falls back to the current directory if no data directory is available.
    pub fn open() -> Self {
        let path = db_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        info!("opening database at {}", path.display());
        let conn = Connection::open(&path).expect("failed to open database");
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.migrate();
        db
    }

    // -- migrations --------------------------------------------------------

    fn migrate(&self) {
        let conn = self.conn.lock().expect("db lock poisoned");
        conn.execute_batch(
            "
                CREATE TABLE IF NOT EXISTS settings (
                    id          INTEGER PRIMARY KEY CHECK (id = 1),
                    n           INTEGER NOT NULL DEFAULT 2,
                    rounds      INTEGER NOT NULL DEFAULT 24,
                    round_time  REAL    NOT NULL DEFAULT 3.0,
                    position    INTEGER NOT NULL DEFAULT 1,
                    color       INTEGER NOT NULL DEFAULT 1,
                    shape       INTEGER NOT NULL DEFAULT 1,
                    sound       INTEGER NOT NULL DEFAULT 1
                );

                INSERT OR IGNORE INTO settings (id) VALUES (1);

                CREATE TABLE IF NOT EXISTS scores (
                    id               INTEGER PRIMARY KEY AUTOINCREMENT,
                    played_at        TEXT    NOT NULL DEFAULT (datetime('now')),
                    n                INTEGER NOT NULL,
                    total_rounds     INTEGER NOT NULL,
                    round_duration   REAL    NOT NULL,
                    correct          INTEGER NOT NULL,
                    wrong            INTEGER NOT NULL,
                    f1_score_percent INTEGER NOT NULL
                );
                ",
        )
        .expect("failed to run migrations");
    }

    // -- settings ----------------------------------------------------------

    pub fn load_settings(&self) -> GameSettings {
        let conn = self.conn.lock().expect("db lock poisoned");
        conn.query_row("SELECT * FROM settings WHERE id = 1", [], |row| {
            Ok(GameSettings {
                n: row.get::<_, usize>("n")?,
                rounds: row.get::<_, usize>("rounds")?,
                round_time: row.get::<_, f32>("round_time")?,
                position: row.get::<_, bool>("position")?,
                color: row.get::<_, bool>("color")?,
                shape: row.get::<_, bool>("shape")?,
                sound: row.get::<_, bool>("sound")?,
            })
        })
        .unwrap_or_default()
    }

    pub fn save_settings(&self, s: &GameSettings) {
        let conn = self.conn.lock().expect("db lock poisoned");
        conn.execute(
            "UPDATE settings SET n=?1, rounds=?2, round_time=?3,
                 position=?4, color=?5, shape=?6, sound=?7
                 WHERE id = 1",
            rusqlite::params![
                s.n,
                s.rounds,
                s.round_time,
                s.position,
                s.color,
                s.shape,
                s.sound,
            ],
        )
        .expect("failed to save settings");
    }

    // -- scores ------------------------------------------------------------

    pub fn load_scores(&self) -> ScoreHistory {
        let conn = self.conn.lock().expect("db lock poisoned");
        let mut stmt = conn
            .prepare(
                "SELECT n, total_rounds, round_duration, correct, wrong, f1_score_percent, played_at
                 FROM scores ORDER BY id DESC LIMIT 50",
            )
            .expect("failed to prepare score query");

        let records = stmt
            .query_map([], |row| {
                Ok(ScoreRecord {
                    n: row.get("n")?,
                    total_rounds: row.get("total_rounds")?,
                    round_duration: row.get("round_duration")?,
                    correct: row.get("correct")?,
                    wrong: row.get("wrong")?,
                    f1_score_percent: row.get("f1_score_percent")?,
                    played_at: row.get("played_at")?,
                })
            })
            .expect("failed to load scores")
            .filter_map(|r| r.ok())
            .collect();

        ScoreHistory(records)
    }

    pub fn insert_score(&self, record: &ScoreRecord) {
        let conn = self.conn.lock().expect("db lock poisoned");
        conn.execute(
            "INSERT INTO scores (n, total_rounds, round_duration, correct, wrong, f1_score_percent)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                record.n,
                record.total_rounds,
                record.round_duration,
                record.correct,
                record.wrong,
                record.f1_score_percent,
            ],
        )
        .expect("failed to insert score");
    }
}

#[cfg(target_arch = "wasm32")]
impl Database {
    /// Open the in-memory fallback used by the browser build.
    pub fn open() -> Self {
        Database::default()
    }

    pub fn load_settings(&self) -> GameSettings {
        self.state
            .lock()
            .expect("db lock poisoned")
            .settings
            .clone()
    }

    pub fn save_settings(&self, s: &GameSettings) {
        self.state.lock().expect("db lock poisoned").settings = s.clone();
    }

    pub fn load_scores(&self) -> ScoreHistory {
        self.state.lock().expect("db lock poisoned").scores.clone()
    }

    pub fn insert_score(&self, record: &ScoreRecord) {
        let mut state = self.state.lock().expect("db lock poisoned");
        let mut record = record.clone();
        record.played_at = session_timestamp();
        state.scores.0.insert(0, record);
        state.scores.0.truncate(50);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn db_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nback")
        .join("nback.db")
}

#[cfg(target_arch = "wasm32")]
fn session_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_default()
}

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            save_settings.run_if(resource_changed::<GameSettings>),
        );
    }
}

/// Persist settings to the database whenever they change.
fn save_settings(db: Res<Database>, settings: Res<GameSettings>) {
    db.save_settings(&settings);
    info!("settings saved");
}
