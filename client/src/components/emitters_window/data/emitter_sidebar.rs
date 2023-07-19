// use crate::models::queries::get_object::get_object::GetObjectVariant;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub sidebar_items: Vec<GetObjectVariant>,
    pub selected_sidebar_item: Option<GetObjectVariant>,
    pub select_sidebar_item_callback: Callback<GetObjectVariant>,
}

#[function_component(EmitterSidebar)]
pub fn emitter_sidebar(
    Props {
        sidebar_items,
        selected_sidebar_item,
        select_sidebar_item_callback,
    }: &Props,
) -> Html {
    let emitter_sidebar_iter = sidebar_items.iter().enumerate().map(|(key, sidebar_item)| {
        let onclick = {
            let select_sidebar_item_callback = select_sidebar_item_callback.clone();
            let sidebar_item = sidebar_item.clone();
            Callback::from(move |_| {
                let sidebar_item = sidebar_item.clone();
                select_sidebar_item_callback.emit(sidebar_item);
            })
        };
        let class = selected_sidebar_item
            .as_ref()
            .and_then(|selected_sidebar_item| {
                (selected_sidebar_item == sidebar_item).then(|| "active")
            });
        html! {
            <button {key} class={classes!(class)} {onclick}>{ sidebar_item }</button>
        }
    });

    html! {
        <div class={classes!("emitter-data-side")}>
            { for emitter_sidebar_iter }
        </div>
    }
}
