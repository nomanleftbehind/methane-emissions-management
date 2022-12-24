use self::user::UserMutations;
use async_graphql::MergedObject;

pub mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(pub UserMutations);

pub fn full_mutation() -> FullMutation {
    FullMutation(UserMutations)
}
