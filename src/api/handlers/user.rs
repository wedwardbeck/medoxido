use crate::api::{ApiContext};
// use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
// use axum::routing::{get, post};
use axum::Json;

use crate::api::error::{Error};
use crate::api::extractor::AuthUser;

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

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq)]
#[serde(default)] // fill in any missing fields with `..UpdateUser::default()`
struct UpdateUser {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
    // bio: Option<String>,
    // image: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    email: String,
    token: String,
    username: String,
    // bio: String,
    // image: Option<String>,
}

pub async fn create_user(
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>, Error>{
    let password_hash = hash_password(req.user.password).await?;

    let sql = ctx.db.query(
        "CREATE user SET username = $username, email = $email, password = $password;")
        .bind(("username", req.user))
        .bind(("email", req.name))
        .bind(("password", password_hash))
        .await
        .on_constraint("user_username_key", |_| {
            Error::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            Error::unprocessable_entity([("email", "email taken")])
        })?;
    let user_id: Option<UserBody<User>> = sql.take(0)?;

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            username: req.user.username,
            // bio: "".to_string(),
            // image: None,
        },
    }))
}

pub async fn login_user(
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>, Error> {
    let sql = ctx.db.query(
        "SELECT id, username, email, password_hash FROM user WHERE username = $username;")
        .bind(("username", req.user.username))
        .await?
        .ok_or_else(|| Error::unprocessable_entity([("email", "does not exist")]))?;
    let user: Option<UserBody<User>> = sql.take(0)?;

    verify_password(req.user.password, user.password_hash).await?;
            // select user_id, email, username, bio, image, password_hash
        // req.user.email,

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&ctx),
            username: user.username,
            // bio: user.bio,
            // image: user.image,
        },
    }))
}

pub async fn get_current_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
) -> Result<Json<UserBody<User>>, Error> {
    let sql = ctx.db.query(
        "SELECT username, FROM user WHERE id = type::thing('user'. $id;")
        .bind(("id", auth_user.user_id))
        .await?
        .ok_or_else(|| Error::unprocessable_entity([("email", "does not exist")]))?;
    let user: Option<UserBody<User>> = sql.take(0)?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
            // bio: user.bio,
            // image: user.image,
        },
    }))
}

pub async fn update_user(
    auth_user: AuthUser,
    ctx: State<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>, Error> {
    if req.user == UpdateUser::default() {
        // If there's no fields to update, these two routes are effectively identical.
        return get_current_user(auth_user, ctx).await;
    }

    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let sql = ctx.db.query(
        "UPDATE user SET username = $username, email = $email, password = $password
        WHERE id - type::thing('user', $id);")
        .bind(("username", req.user.username))
        .bind(("email", req.user.email))
        .bind(("password", password_hash))
        .bind(("id", auth_user.user_id))
        .await
        .on_constraint("user_username_key", |_| {
            Error::unprocessable_entity([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            Error::unprocessable_entity([("email", "email taken")])
        })?;
    let user: Option<UserBody<User>> = sql.take(0)?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
            // bio: user.bio,
            // image: user.image,
        },
    }))
}

async fn hash_password(password: String) -> Result<String, Error> {
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

async fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
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
