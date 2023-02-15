use self::{controller_month_vent::ControllerMonthVentMutations, user::UserMutations};
use async_graphql::MergedObject;

pub mod controller_month_vent;
pub mod user;

#[derive(MergedObject, Default, Clone)]
pub struct FullMutation(pub UserMutations, pub ControllerMonthVentMutations);

pub fn full_mutation() -> FullMutation {
    FullMutation(UserMutations, ControllerMonthVentMutations)
}
