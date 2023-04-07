use crate::{
    components::emitters_window::data::objects::ObjectsComponent,
    models::queries::get_object::get_object::GetObjectVariant::CONTROLLER_CHANGE_BY_CONTROLLER_ID,
    utils::error::AppError,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
    pub error_handle: Callback<Option<AppError>>,
}

#[function_component(ObjectDataComponent)]
pub fn object_data_component(
    Props {
        id,
        row_num,
        col_num,
        error_handle,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: 3/{};", row_num, col_num + 1);
    let id = Rc::new(*id);

    html! {
        <div class={classes!("emitter-data-wrapper")} {style}>
            <div class={classes!("emitter-data")}>
                <div class={classes!("emitter-data-side")}>
                    <button>{ "Changes" }</button>
                    <button>{ "Month Vent Overrides" }</button>
                </div>
                <div class={classes!("emitter-data-main")}>
                    <ObjectsComponent {id} {error_handle} object_variant={CONTROLLER_CHANGE_BY_CONTROLLER_ID} />
                </div>
            </div>
        </div>
    }
}
