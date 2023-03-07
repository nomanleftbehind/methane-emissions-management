use crate::graphql::models::{RegisterUserInput, Role, User};
use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

#[derive(Debug)]
pub struct UserSecret {
    pub id: Uuid,
    pub email: String,
    pub password: Secret<String>,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
}

impl From<User> for UserSecret {
    fn from(
        User {
            id,
            email,
            password,
            first_name,
            last_name,
            role,
        }: User,
    ) -> Self {
        Self {
            id,
            email,
            password: Secret::new(password),
            first_name,
            last_name,
            role,
        }
    }
}

#[tracing::instrument(name = "Get stored credentials", skip(email, pool))]
async fn get_stored_credentials(
    email: String,
    pool: &PgPool,
) -> Result<Option<UserSecret>, anyhow::Error> {
    let row = sqlx::query_as!(
        User,
        r#"SELECT id, email, password, first_name, last_name, role as "role: _" FROM "users" WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await?
    .map(|user| user.into());
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<User, AuthError> {
    let mut user = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some(UserSecret {
        id,
        email,
        password,
        first_name,
        last_name,
        role,
    }) = get_stored_credentials(credentials.email, pool).await?
    {
        let user_no_password = User {
            id,
            email,
            password: "".to_string(),
            first_name,
            last_name,
            role,
        };

        user = Some(user_no_password);
        expected_password_hash = password;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user.ok_or_else(|| anyhow::anyhow!("Unknown email."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate credentials",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

// #[tracing::instrument(name = "Change password", skip(password, pool))]
// pub async fn change_password(
//     user_id: uuid::Uuid,
//     password: Secret<String>,
//     pool: &PgPool,
// ) -> Result<(), anyhow::Error> {
//     let password = spawn_blocking_with_tracing(move || compute_password_hash(password))
//         .await?
//         .context("Failed to hash password")?;
//     sqlx::query!(
//         r#"
//         UPDATE users
//         SET password = $1
//         WHERE id = $2
//         "#,
//         password.expose_secret(),
//         user_id
//     )
//     .execute(pool)
//     .await
//     .context("Failed to change user's password in the database.")?;
//     Ok(())
// }

// fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
//     let salt = SaltString::generate(&mut rand::thread_rng());
//     let password = Argon2::new(
//         Algorithm::Argon2id,
//         Version::V0x13,
//         Params::new(15000, 2, 1, None).unwrap(),
//     )
//     .hash_password(password.expose_secret().as_bytes(), &salt)?
//     .to_string();
//     Ok(Secret::new(password))
// }

pub async fn register(
    pool: &PgPool,
    RegisterUserInput {
        email,
        password,
        first_name,
        last_name,
        role,
    }: RegisterUserInput,
) -> Result<User, sqlx::Error> {
    println!(
        "register input: {}, {}, {}, {}, {:?}",
        email, password, first_name, last_name, role
    );
    let salt = SaltString::generate(&mut rand::thread_rng());
    // Match production parameters
    let password = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();

    let user = if let Some(some_role) = role {
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (id, email, password, first_name, last_name, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING email, id, password, first_name, last_name, role as "role: _";"#,
            Uuid::new_v4(),
            email,
            password,
            first_name,
            last_name,
            some_role as Role
        )
        .fetch_one(pool)
        .await
    } else {
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (id, email, password, first_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING email, id, password, first_name, last_name, role as "role: _";
            "#,
            Uuid::new_v4(),
            email,
            password,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await
    };

    user
}
