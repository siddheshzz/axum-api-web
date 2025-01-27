// region:    --- Modules
mod base;
mod error;
mod store;
pub mod task;


use store::{new_db_pool, Db};

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db:Db,

}

impl ModelManager {
	/// Creates a new ModelManager
	///
	/// The manager is created with a new database pool.
	///
	/// # Errors
	///
	/// This function returns an error if the database pool cannot be created.

	pub async fn new() -> Result<Self> {

		let db = new_db_pool().await?;

		// FIXME - TBC
		Ok(ModelManager {
			db
		})
	}

	/// Returns the sqlx db pool reference.
	/// (Only for the model layer)
	/// if one tries to access the db from the web layer, it will panic- stating db is private
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
