use axum::{Extension, Json};
use infrastructure::{
	context::AppContext,
	errors::{Error, HttpError, ValidationErrorItem},
	hash::hash_password,
};

/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
	user: T,
}

#[derive(serde::Deserialize)]
struct NewUser {
	username: String,
	email: String,
	password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
	email: String,
	token: String,
	username: String,
	bio: String,
	image: Option<String>,
}

async fn create_user(ctx: Extension<AppContext>, Json(req): Json<UserBody<NewUser>>) -> Result<Json<UserBody<User>>> {
	let password_hash = hash_password(req.user.password).await?;

	let user_id = sqlx::query_scalar!(
		// language=PostgreSQL
		r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
		req.user.username,
		req.user.email,
		password_hash
	)
	.fetch_one(&ctx.db)
	.await
	.on_constraint("user_username_key", |_| {
		HttpError::unprocessable_entity(vec![ValidationErrorItem {
			loc: vec!["body".to_owned(), "filterTypes".to_owned()],
			msg: format!("username taken"),
			ty: "value_error".to_owned(),
		}])
	})
	.on_constraint("user_email_key", |_| Error::unprocessable_entity([("email", "email taken")]))?;

	Ok(Json(UserBody {
		user: User {
			email: req.user.email,
			token: AuthUser { user_id }.to_jwt(&ctx),
			username: req.user.username,
			bio: "".to_string(),
			image: None,
		},
	}))
}
