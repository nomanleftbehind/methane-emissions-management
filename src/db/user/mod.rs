pub mod repository;

use crate::schema::user;
use crate::schemas::root::Context;
use juniper::{graphql_object, GraphQLInputObject};
use uuid::Uuid;

#[derive(Queryable, Clone)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    role: String,
}

#[derive(GraphQLInputObject, Insertable)]
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
    // The full name method is useful outside of GraphQL,
    // so we define it as a normal method.
    pub fn build_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }

    fn full_name(&self) -> String {
        self.build_full_name()
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn password(&self) -> &str {
        &self.password
    }

    fn role(&self, arg: String, _ctx: &Context) -> String {

        // println!("role arg: {:?}", ctx.db_pool);
        println!("role arg: {}", arg);
        arg
        // &self.role
    }
}
