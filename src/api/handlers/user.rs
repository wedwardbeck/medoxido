
// use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::{ Thing, Datetime, Op };
// use anyhow::Context;
// use axum::routing::{get, post};

use crate::api::error::Error;
use crate::api::{ApiContext, Result};
use crate::api::extractor::AuthUser;

// pub(crate) fn router() -> Router<ApiContext> {

//     Router::new()
//         .route("/api/users", post(create_user))
//         .route("/api/users/login", post(login_user))
//         .route("/api/user", get(get_current_user).put(update_user))
// }

const USER: &str = "user";

#[derive(Serialize, Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct PassUser {
    username: String,
    password_hash: String,
    user_id: String,
    email: Option<String>
}

#[derive(Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    // password: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct UpdatePassword {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UserId {
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    token: String,
    username: String,
}

pub(crate) async fn create_user(
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    // front end will validate username and email for uniqueness prior to calling create
    // once Surreal has better error codes and info, we can incorporate unique check return here
    let mut sql = ctx.db.query(
        "CREATE user SET email = $email, username = $username, password_hash = $password_hash RETURN id AS user_id;")
        .bind(("email", &*req.user.email))
        .bind(("username", &*req.user.username))
        .bind(("password_hash", password_hash))
        .await?;
    let my_user_id: Option<UserId>  = sql.take((0, "user_id"))?;

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            token: AuthUser { user_id: my_user_id.expect("Failed to get user_id").user_id  }.to_jwt(&ctx),
            username: req.user.username,
        },
    }))
}

pub(crate) async fn login_user(
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let mut sql = ctx.db.query(
        "select id as user_id, email, username, password_hash
        from user where username = $1;")
        .bind(("username", req.user.username))
        .await?;

    let user: PassUser = sql.take(0)?;
    let dog = user.email;


    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&ctx),
            username: user.username,
        },
    }))
}

pub(crate) async fn get_current_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    let user: User = ctx.db.select((USER, &*auth_user.user_id)).await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}

pub(crate) async fn update_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        // If there's no fields to update, these two routes are effectively identical.
        return get_current_user(auth_user, ctx).await;
    }

    // WTB `Option::map_async()`
    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let mut sql = ctx.db.query(
        "UPDATE user SET email = $email, username = $username WHERE id = type::thing('user', $user);")
        .bind(("email", req.user.email))
        .bind(("username", req.user.username))
        .bind(("user", auth_user.user_id))
        .await?;
    let user: Option<User> = sql.take(0)?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}


pub(crate) async fn update_password(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<UpdatePassword>>,
) -> Result<Json<UserBody<User>>> {

    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let mut sql = ctx.db.query(
        "UPDATE user SET password_hash = $password_hash WHERE id = type::thing('user', $user);")
        .bind(("password_hash", password_hash))
        .bind(("user", auth_user.user_id))
        .await?;
    let user = sql.take(0)?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}

async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, &salt)
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")?
}
