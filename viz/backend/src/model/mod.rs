mod db;
mod raw_data_dao;
mod viz_metadata_dao;
mod viz_questions_dao;
mod viz_categories_dao;

// re-export
pub use db::init_db;
pub use db::Db;
pub use raw_data_dao::{RawData, RawDataObj};
pub use viz_metadata_dao::{VizMetadata, VizMetadataObj};
pub use viz_questions_dao::{VizQuestions, VizQuestionsObj};
pub use viz_categories_dao::{VizCategories, VizCategoriesObj};

// region:    Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Entity Not Found - {0}[{1}] ")]
	EntityNotFound(&'static str, String),

	#[error(transparent)]
	SqlxError(#[from] sqlx::Error),

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}

// endregion: Error
