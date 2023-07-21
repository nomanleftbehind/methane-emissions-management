use crate::graphql::{
    context::ContextExt,
    models::{dropdown_selection::DropdownSelection, input::GetDropdownSelectionInput},
    sql::dropdown_selection,
};
use async_graphql::{Context, Error, Object};

#[derive(Default, Clone)]
pub struct DropdownSelectionQuery;

#[Object]
impl DropdownSelectionQuery {
    async fn get_dropdown_selection(
        &self,
        ctx: &Context<'_>,
        get_dropdown_selection_input: GetDropdownSelectionInput,
    ) -> Result<Vec<DropdownSelection>, Error> {
        let pool = ctx.db_pool();
        dropdown_selection::get_dropdown_selection(pool, get_dropdown_selection_input)
            .await
            .map_err(Error::from)
    }
}
