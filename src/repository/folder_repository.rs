use crate::errors::db_error::DbError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::folder::{AllFolderDTO, InsertFolderDTO, SingleFolderDTO};

pub async fn get_all_by_user_id(client: &Client, user_id: i64) -> Result<Vec<AllFolderDTO>, DbError> {
    let stmt = include_str!("sql/folder/get_all_by_user_id.sql");
    let stmt = client.prepare(&stmt).await?;

    let folders = client
        .query(&stmt, &[&user_id])
        .await?
        .iter()
        .map(|row| AllFolderDTO::from_row_ref(row).unwrap())
        .collect::<Vec<AllFolderDTO>>();

    Ok(folders)
}

pub async fn get_one_by_id_user_id(client: &Client, folder_id : i64, user_id: i64) -> Result<SingleFolderDTO, DbError> {
    let stmt = include_str!("sql/folder/get_one_by_id_user_id.sql");
    let stmt = client.prepare(&stmt).await?;

    let folders = client
        .query(&stmt, &[
            &folder_id,
            &user_id
        ])
        .await?
        .iter()
        .map(|row| SingleFolderDTO::from_row_ref(row).unwrap())
        .collect::<Vec<SingleFolderDTO>>()
        .pop()
        .ok_or(DbError::NotFound)?;;

    Ok(folders)
}

pub async fn insert(client: &Client, insert_folder_dto: InsertFolderDTO, user_id: i64) -> Result<i64, DbError> {
    let _stmt = include_str!("./sql/folder/insert.sql");
    let stmt = client.prepare(&_stmt).await?;

    client
        .query(
            &stmt,
            &[
                &insert_folder_dto.name,
                &user_id
            ],
        )
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(DbError::NotFound)
}