use crate::{
    authentication::{register, validate_credentials, Credentials, AUTH_COOKIE_NAME},
    graphql::{
        context::ContextExt,
        models::{LoginUserInput, RegisterUserInput, User},
    },
};
use async_graphql::{Context, Error, Object};
use http::header::SET_COOKIE;
use secrecy::Secret;

fn logged_in_err() -> Error {
    Error::new("Already logged in")
}

#[derive(Default, Clone)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    async fn register(
        &self,
        ctx: &Context<'_>,
        register_user_input: RegisterUserInput,
    ) -> Result<User, Error> {
        let pool = ctx.db_pool();

        let result = register(pool, register_user_input).await?;
        Ok(result)
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        login_user_input: LoginUserInput,
    ) -> Result<User, Error> {
        let pool = ctx.db_pool();

        if let Ok(_cookie) = ctx.get_cookie() {
            Err(logged_in_err())
        } else {
            let LoginUserInput { email, password } = login_user_input;

            println!("email: {}, password: {}", email, password);

            let credentials = Credentials {
                email,
                password: Secret::new(password),
            };

            let user = validate_credentials(credentials, pool).await?;

            let session_manager = ctx.get_session_manager()?;

            session_manager
                .create_session(user.id)
                .await?
                .set_cookie(ctx)
                .await?;

            Ok(user)
        }
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        ctx.insert_http_header(
            SET_COOKIE,
            format!(
                "{}=deleted; Max-Age=-1; SameSite=None; Secure; Path=/",
                AUTH_COOKIE_NAME
            ),
        );
        println!("Logging out cookie {}", AUTH_COOKIE_NAME);
        Ok(true)
    }
}
