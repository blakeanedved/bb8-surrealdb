use std::collections::BTreeMap;
use surrealdb::{sql::Value, Datastore, Error, Response, Session};

pub struct Connection {
    ds: Datastore,
    ses: Session,
}

impl Connection {
    pub async fn execute(
        &self,
        txt: impl AsRef<str>,
        vars: Option<BTreeMap<String, Value>>,
        strict: bool,
    ) -> Result<Vec<Response>, Error> {
        Ok(self
            .ds
            .execute(txt.as_ref(), &self.ses, vars, strict)
            .await?)
    }
}

enum ConnectionType {
    Memory,
    File(String),
    #[cfg(feature = "tikv")]
    TiKV(String),
}

pub struct SurrealdbConnectionManager {
    connection_type: ConnectionType,
    session: Session,
}

impl SurrealdbConnectionManager {
    pub async fn memory(session: Session) -> Self {
        Self {
            session,
            connection_type: ConnectionType::Memory,
        }
    }

    pub async fn file(path: impl AsRef<str>, session: Session) -> Self {
        Self {
            session,
            connection_type: ConnectionType::File(format!("file://{}", path.as_ref())),
        }
    }

    #[cfg(feature = "tikv")]
    pub async fn tikv(uri: impl AsRef<str>, session: Session) -> Self {
        Self {
            session,
            connection_type: ConnectionType::TiKV(format!("tikv://{}", uri.as_ref())),
        }
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for SurrealdbConnectionManager {
    type Connection = Connection;
    type Error = surrealdb::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(Connection {
            ds: match &self.connection_type {
                ConnectionType::Memory => Datastore::new("memory").await?,
                ConnectionType::File(path) => Datastore::new(path.as_ref()).await?,
                #[cfg(feature = "tikv")]
                ConnectionType::TiKV(uri) => Datastore::new(uri.as_ref()).await?,
            },
            ses: self.session.clone(),
        })
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute("SELECT * FROM 1;", None, false).await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
