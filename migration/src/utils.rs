use anyhow::{anyhow, Result};
use async_std::{
    fs::{self, File},
    io::{prelude::BufReadExt, BufReader},
    path::PathBuf,
    prelude::StreamExt,
};
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DatabaseBackend, EntityTrait, IdenStatic, Statement},
};

pub async fn create_one_table<E>(db: &dyn ConnectionTrait, builder: DatabaseBackend, schema: &Schema, e: E) -> Result<(), DbErr>
where
    E: EntityTrait,
{
    match db.execute(builder.build(schema.create_table_from_entity(e).to_owned().if_not_exists())).await {
        Ok(_) => println!("Table created successfuly: {}", e.table_name()),
        Err(e) => println!("{}", e),
    };

    Ok(())
}
