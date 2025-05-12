//! A key-value store backend for the tower-sessions crate.

use std::{borrow::Borrow, sync::LazyLock};

use kv::{Key, KeyValueStore, KvPrimitive, KvTransaction, StrictSlug, Value};
use tower_sessions::{
  session::{Id, Record},
  session_store::Error,
  SessionStore,
};

/// A key-value store backend for the tower-sessions crate.
#[derive(Clone, Debug)]
pub struct TowerSessionsKvStore {
  kv: KeyValueStore,
}

impl TowerSessionsKvStore {
  /// Create a new key-value store backend.
  #[must_use]
  pub fn new(kv: KeyValueStore) -> Self { Self { kv } }
}

static SESSION_NS_SEGMENT: LazyLock<StrictSlug> =
  LazyLock::new(|| StrictSlug::new("session".to_string()));

fn session_id_to_key(id: &Id) -> Key {
  Key::new_lazy(&SESSION_NS_SEGMENT).with(StrictSlug::new(id.to_string()))
}

#[async_trait::async_trait]
impl SessionStore for TowerSessionsKvStore {
  async fn save(&self, session_record: &Record) -> Result<(), Error> {
    let key = session_id_to_key(&session_record.id);
    let value = Value::serialize(session_record).map_err(|e| {
      Error::Encode(format!("Failed to serialize session record: {e}"))
    })?;

    let mut txn =
      self.kv.begin_pessimistic_transaction().await.map_err(|e| {
        Error::Backend(format!("Failed to start pessimistic transaction: {e}"))
      })?;

    let mut txn = {
      if let Err(e) = txn.put(&key, value).await.map_err(|e| {
        Error::Backend(format!("Failed to put session record: {e}"))
      }) {
        txn.rollback().await.map_err(|e| {
          Error::Backend(format!("Failed to rollback transaction: {e}"))
        })?;
        return Err(Error::Backend(format!(
          "Failed to put session record: {e}"
        )));
      }
      txn
    };

    if let Err(e) = txn.commit().await {
      txn.rollback().await.map_err(|e| {
        Error::Backend(format!("Failed to rollback transaction: {e}"))
      })?;
      return Err(Error::Backend(format!("Failed to commit transaction: {e}")));
    }

    Ok(())
  }

  async fn create(&self, session_record: &mut Record) -> Result<(), Error> {
    self.save(session_record.borrow()).await
  }

  async fn load(&self, session_id: &Id) -> Result<Option<Record>, Error> {
    let mut txn =
      self.kv.begin_optimistic_transaction().await.map_err(|e| {
        Error::Backend(format!("Failed to start optimistic transaction: {e}"))
      })?;

    let key = session_id_to_key(session_id);

    let (mut txn, value) =
      match txn.get(&key).await.map_err(|e| {
        Error::Backend(format!("Failed to get session record: {e}"))
      }) {
        Ok(Some(v)) => (txn, v),
        Ok(None) => {
          txn.rollback().await.map_err(|e| {
            Error::Backend(format!("Failed to rollback transaction: {e}"))
          })?;
          return Ok(None);
        }
        Err(e) => {
          txn.rollback().await.map_err(|e| {
            Error::Backend(format!("Failed to rollback transaction: {e}"))
          })?;
          return Err(e);
        }
      };

    if let Err(e) = txn.commit().await {
      txn.rollback().await.map_err(|e| {
        Error::Backend(format!("Failed to rollback transaction: {e}"))
      })?;
      return Err(Error::Backend(format!("Failed to commit transaction: {e}")));
    }

    let record = value.deserialize().map_err(|e| {
      Error::Decode(format!("Failed to deserialize session record: {e}"))
    })?;

    Ok(Some(record))
  }

  async fn delete(&self, session_id: &Id) -> Result<(), Error> {
    // delete if it exists
    let mut txn =
      self.kv.begin_pessimistic_transaction().await.map_err(|e| {
        Error::Backend(format!("Failed to start pessimistic transaction: {e}"))
      })?;

    let key = session_id_to_key(session_id);

    let mut txn = {
      if let Err(e) = txn.delete(&key).await.map_err(|e| {
        Error::Backend(format!("Failed to delete session record: {e}"))
      }) {
        txn.rollback().await.map_err(|e| {
          Error::Backend(format!("Failed to rollback transaction: {e}"))
        })?;
        return Err(Error::Backend(format!(
          "Failed to delete session record: {e}"
        )));
      }
      txn
    };

    if let Err(e) = txn.commit().await {
      txn.rollback().await.map_err(|e| {
        Error::Backend(format!("Failed to rollback transaction: {e}"))
      })?;
      return Err(Error::Backend(format!("Failed to commit transaction: {e}")));
    }

    Ok(())
  }
}
