use crate::{
    components::emitters_window::data::{
        emitter_sidebar::EmitterSidebar, objects::ObjectsComponent,
    },
    models::queries::get_object::get_object::GetObjectVariant,
    utils::error::AppError,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
    pub sidebar_items: Vec<GetObjectVariant>,
    pub error_handle: Callback<Option<AppError>>,
}

#[function_component(ObjectDataComponent)]
pub fn object_data_component(
    Props {
        id,
        row_num,
        col_num,
        error_handle,
        sidebar_items,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: 3/{};", row_num, col_num + 1);
    let id = Rc::new(*id);

    let selected_sidebar_item_handle = use_state_eq(|| None);
    let selected_sidebar_item = (*selected_sidebar_item_handle).clone();

    let select_sidebar_item_callback = Callback::from(move |e: GetObjectVariant| {
        selected_sidebar_item_handle.set(Some(e));
    });

    html! {
        <div class={classes!("emitter-data-wrapper")} {style}>
            <div class={classes!("emitter-data")}>
                <EmitterSidebar sidebar_items={sidebar_items.clone()} selected_sidebar_item={selected_sidebar_item.clone()} {select_sidebar_item_callback} />
                if let Some(sidebar_item) = selected_sidebar_item {
                    <div class={classes!("emitter-data-main")}>
                        <ObjectsComponent {id} {error_handle} object_variant={sidebar_item} />
                    </div>
                }
            </div>
        </div>
    }
}
