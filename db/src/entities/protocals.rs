#[cfg(feature = "test-helpers")]
use fake::{faker::lorem::en::*, Dummy};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Postgres;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug, Deserialize)]
pub struct Protocal {
    pub id: Uuid,
}

#[derive(Deserialize, Validate, Clone)]
#[cfg_attr(feature = "test-helpers", derive(Serialize, Dummy))]
pub struct ProtocalChangeset {
}

pub async fn load_all(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Vec<Protocal>, crate::Error> {
    let protocals = sqlx::query_as!(Protocal, "SELECT id,  FROM protocals")
        .fetch_all(executor)
        .await?;
    Ok(protocals)
}

pub async fn load(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Protocal, crate::Error> {
    match sqlx::query_as!(
        Protocal,
        "SELECT id,  FROM protocals WHERE id = $1",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(protocal) => Ok(protocal),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn create(
    protocal: ProtocalChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Protocal, crate::Error> {
    protocal.validate()?;

    let record = sqlx::query!(
        "INSERT INTO protocals () VALUES () RETURNING id",
    )
    .fetch_one(executor)
    .await
    .map_err(crate::Error::DbError)?;

    Ok(Protocal {
        id: record.id,
    })
}

pub async fn update(
    id: Uuid,
    protocal: ProtocalChangeset,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<Protocal, crate::Error> {
    protocal.validate()?;

    match sqlx::query!(
        "UPDATE protocals SET  WHERE id = $1 RETURNING id",
        id
    )
    .fetch_optional(executor)
    .await
    .map_err(crate::Error::DbError)?
    {
        Some(record) => Ok(Protocal {
            id: record.id,
        }),
        None => Err(crate::Error::NoRecordFound),
    }
}

pub async fn delete(
    id: Uuid,
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<(), crate::Error> {
    match sqlx::query!("DELETE FROM protocals WHERE id = $1 RETURNING id", id)
        .fetch_optional(executor)
        .await
        .map_err(crate::Error::DbError)?
    {
        Some(_) => Ok(()),
        None => Err(crate::Error::NoRecordFound),
    }
}
