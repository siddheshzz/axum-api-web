use crate::ctx::Ctx;
// use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};

use serde::{Deserialize, Serialize};

use sqlx::FromRow;


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub title: String,
}

#[derive( Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive( Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

// region:    --- TaskBmc
pub struct TaskBmc;

impl TaskBmc{

    pub async fn create(
        _ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64>{
        
        let db = mm.db();

        let (id,) = sqlx::query_as::<_,(i64,)>(
            "INSERT INTO task (title) values ($1) returning id",
        )
        .bind(task_c.title)
        .fetch_one(db)
        .await?;
        Ok(id)
    }

    // pub async fn get(
    //     _ctx: &Ctx,
    //     mm: &ModelManager,
    //     task: i64,
    // ) -> Result<Task>{
        
    //     let db = mm.db();

    //     let t= sqlx::query_as(
    //         "SELECT title FROM tasks WHERE id=$1",
    //     )
    //     .bind(task)
    //     .fetch_one(db)
    //     .await?;
    //     Ok(t)
    // }


    // pub async fn delete(
    //     _ctx: &Ctx,
    //     mm: &ModelManager,
    //     task: i64,
    // ) -> Result<()>{
        
    //     let db = mm.db();

    //     let t= sqlx::query(
    //         "DELETE FROM tasks WHERE id=$1",
    //     )
    //     .bind(task)
    //     .execute(db)
    //     .await?
    //     .rows_affected();
    //     Ok(())
    // }
}



#[cfg(test)]
mod tests{
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use crate::_dev_utils;
    use crate::ctx::Ctx;
    use crate::model::ModelManager;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";

		// -- Exec
		let task_c = TaskForCreate {
			title: fx_title.to_string(),
		};
		let id = TaskBmc::create(&ctx, &mm, task_c).await?;


        let (title,):(String,) = sqlx::query_as(
            "SELECT title FROM task WHERE id = $1",)
        .bind(id)
        .fetch_one(mm.db())
        .await?;
        assert_eq!(title, fx_title);

		// // -- Check
		// let task = TaskBmc::get(&ctx, &mm, id).await?;
		// assert_eq!(task.title, fx_title);

		// // -- Clean
		// TaskBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}
}