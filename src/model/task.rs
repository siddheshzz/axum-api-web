use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};

use serde::{Deserialize, Serialize};

use sqlb::Fields;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub title: String,
    
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

// region:    --- TaskBmc
pub struct TaskBmc;


impl DbBmc for TaskBmc{
    const TABLE: &'static str = "task";
}

impl TaskBmc{

    pub async fn create(
        ctx: &Ctx,
        mm: &ModelManager,
        task_c: TaskForCreate,
    ) -> Result<i64>{
        
        base::create::<Self,_>(ctx, mm, task_c).await
    }

    pub async fn get(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
    ) -> Result<Task>{
        
        // let db = mm.db();
        // print!("Arrived in get func");

        // let t= sqlx::query_as(
        //     "SELECT title FROM task WHERE id=$1",
        // )
        // .bind(id)
        // .fetch_optional(db)
        // .await?
        // .ok_or(Error::EntityNotFound{entity:"task",id})?;
        // Ok(t)


        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
    ) -> Result<Vec<Task>>{
        
        // let db = mm.db();

        // let tasks = sqlx::query_as(
        //     "SELECT id, title FROM task ORDER BY id",
        // )
        // .fetch_all(db)
        // .await?
        // .into_iter()
        // .collect();
        // Ok(tasks)


        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn delete(
        _ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
    ) -> Result<()>{
        
        // let db = mm.db();

        // let count= sqlx::query(
        //     "DELETE FROM tasks WHERE id=$1",
        // )
        // .bind(id)
        // .execute(db)
        // .await?
        // .rows_affected();

        // if count == 0{
        //     return Err(Error::EntityNotFound{entity:"task",id});
        // }
        // Ok(())

        base::delete::<Self>(_ctx, mm, id).await
    }
}



#[cfg(test)]
mod tests{
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
    use serial_test::serial;
    use crate::_dev_utils;
    

    #[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";

        println!("Arrived in test_create_ok");
		// -- Exec
		let task_c = TaskForCreate {
			title: fx_title.to_string(),
		};
		let id = TaskBmc::create(&ctx, &mm, task_c).await?;
        println!("Arrived in test_create_ok,{id}");

		// -- Check
		let task = TaskBmc::get(&ctx, &mm, id).await?;
        
		assert_eq!(task.title, fx_title);

		// -- Clean
		// TaskBmc::delete(&ctx, &mm, id).await?;



		Ok(())
	}

    #[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "task",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
		_dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

		// -- Exec
		let tasks = TaskBmc::list(&ctx, &mm).await?;

		// -- Check
		let tasks: Vec<Task> = tasks
			.into_iter()
			.filter(|t| t.title.starts_with("test_list_ok-task"))
			.collect();
		assert_eq!(tasks.len(), 2, "number of seeded tasks.");

		// -- Clean
		for task in tasks.iter() {
			TaskBmc::delete(&ctx, &mm, task.id).await?;
		}

		Ok(())
	}

	// #[serial]
	// #[tokio::test]
	// async fn test_update_ok() -> Result<()> {
	// 	// -- Setup & Fixtures
	// 	let mm = _dev_utils::init_test().await;
	// 	let ctx = Ctx::root_ctx();
	// 	let fx_title = "test_update_ok - task 01";
	// 	let fx_title_new = "test_update_ok - task 01 - new";
	// 	let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
	// 		.await?
	// 		.remove(0);

	// 	// -- Exec
	// 	TaskBmc::update(
	// 		&ctx,
	// 		&mm,
	// 		fx_task.id,
	// 		TaskForUpdate {
	// 			title: Some(fx_title_new.to_string()),
	// 		},
	// 	)
	// 	.await?;

	// 	// -- Check
	// 	let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
	// 	assert_eq!(task.title, fx_title_new);

	// 	Ok(())
	// }

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "task",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}
}