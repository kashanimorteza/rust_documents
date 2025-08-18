//--------------------------------------------------------------------------------- Location
// src/orm/user.rs

//--------------------------------------------------------------------------------- Description
// CRUD logic methods for the User model

//--------------------------------------------------------------------------------- Import
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait};
use crate::models::user::{ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel};

//--------------------------------------------------------------------------------- Action
impl UserModel 
{
	pub async fn select_all(db: &DbConn) -> Result<Vec<UserModel>, DbErr> 
    {
		UserEntity::find().all(db).await
	}

	pub async fn select_by_id(db: &DbConn, id: i32) -> Result<Option<UserModel>, DbErr> 
    {
		UserEntity::find_by_id(id).one(db).await
	}

	pub async fn insert(db: &DbConn, active: UserActiveModel) -> Result<UserModel, DbErr> 
    {
		active.insert(db).await
	}

	pub async fn update(db: &DbConn, active: UserActiveModel) -> Result<UserModel, DbErr> 
    {
		active.update(db).await
	}

	pub async fn delete(db: &DbConn, id: i32) -> Result<u64, DbErr> {
		let res = UserEntity::delete_by_id(id).exec(db).await?;
		Ok(res.rows_affected)
	}
}


