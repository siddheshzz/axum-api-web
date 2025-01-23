use std::{fs, path::PathBuf, time::Duration};


use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL:&str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

//sql files
const SQL_RECREATE_DB: &str = "sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "sql/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");
  {
      // Create the app_db/a[[_user with the postgres user
      let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
      pexec(&root_db, SQL_RECREATE_DB).await?;
  }

  // Get sql files
  let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
  .filter_map(|entry| entry.ok().map(|e| e.path()))
  .collect();

  paths.sort();
  
  // Execute easch file
  let app_db = new_db_pool(PG_DEV_APP_URL).await?;

  for path in paths {
    if let Some(path) = path.to_str() {
        // let path = path.replace('\\','/'); // windows
        if path.ends_with(".sql")&& path!= SQL_RECREATE_DB{
            pexec(&app_db, &path).await?;
        }
    }
}

Ok(())



}


async fn pexec(db:&Db,file: &str) -> Result<(), sqlx::Error>{ 
    info!("{:<12} - pexec {file}", "FOR-DEV-ONLY");
    // Read the file
    let content = fs::read_to_string(file)?;
    // Todo make the split more sql proof
    let sqls: Vec<&str> = content.split(";").collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
    
    
    // sqlx::query_file(file).execute(db).await
}


async fn new_db_pool(db_con_url:&str) -> Result<Db, sqlx::Error>{
    PgPoolOptions::new()
    .max_connections(1)
    .acquire_timeout(Duration::from_millis(500))
    .connect(db_con_url)
    .await
}