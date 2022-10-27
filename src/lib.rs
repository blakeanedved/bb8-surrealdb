use surrealdb::{Datastore, Session};

type Connection = (Datastore, Session);

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
            connection_type: ConnectionType::TiKV(format!("file://{}", uri.as_ref())),
        }
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for SurrealdbConnectionManager {
    type Connection = Connection;
    type Error = surrealdb::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok((
            match &self.connection_type {
                ConnectionType::Memory => Datastore::new("memory").await?,
                ConnectionType::File(path) => Datastore::new(path.as_ref()).await?,
                #[cfg(feature = "tikv")]
                ConnectionType::TiKV(uri) => Datastore::new(uri.as_ref()).await?,
            },
            self.session.clone(),
        ))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let (ds, ses) = conn;
        ds.execute("SELECT * FROM 1;", ses, None, false).await?;
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
