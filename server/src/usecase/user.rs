use crate::app_error::error::AppError;
use crate::domain::{model::user::User as UserModel, repository::{db_conn::DBConn, user::User as UserRepository}};

pub struct User {
    repo: Box<dyn UserRepository + Send + Sync>,
    conn: Box<dyn DBConn + Send + Sync + 'static>,
}
impl User {
    pub fn new(repo: Box<dyn UserRepository + Send + Sync>, conn: Box<dyn DBConn + Send + Sync>) -> Self {
        User { repo, conn }
    }

    pub async fn create(
        &self,
        name: String,
        birthday: i32,
        email: String,
        blood_type: i32,
    ) -> Result<String, AppError> {
        let name = UserModel::new_user_name(name)?;
        let birthday = UserModel::new_user_birthday(birthday)?;
        let email = UserModel::new_user_email(email)?;
        let _new_user = UserModel::new_create(name, birthday, email, blood_type);

        self.repo
            .create(self.conn, _new_user.clone())
            .await?;

        Ok(_new_user.user_id)
    }

    pub async fn update(&self, user_id: String, name: String) -> Result<(), AppError> {
        let name = UserModel::new_user_name(name)?;
        let _new_user = UserModel::new_update(user_id, name);

        self.repo.update(self.conn, _new_user).await?;

        Ok(())
    }
}
