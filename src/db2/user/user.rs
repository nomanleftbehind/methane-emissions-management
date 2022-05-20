use juniper::{GraphQLInputObject, GraphQLObject/* , ID */};
// use juniper::integrations::uuid::;
use crate::schema::user;
use uuid::Uuid;

#[derive(GraphQLObject, Queryable, Clone)]
#[graphql(description = "User")]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    #[graphql(skip)]
    password: String,
    role: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "user"]
#[graphql(description = "User input")]
struct NewUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    role: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct UserNewForm {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl User {
    fn id(&self) -> Uuid /*ID*/ {
        self.id.clone()
        // ID::new(self.id.to_string())
    }

    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }

    fn email(&self) -> String {
        self.email.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }

    fn role(&self) -> String {
        self.role.clone()
    }
}
