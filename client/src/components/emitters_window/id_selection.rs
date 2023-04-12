use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::id_selection::{
        id_selection::{IdSelectionIdSelection, IdSelectionVariant, ResponseData, Variables},
        IdSelection,
    },
    pages::ModalVariant,
};
use yew::{function_component, html, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct IdSelectionProp {
    pub variant: IdSelectionVariant,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub id_selection: IdSelectionProp,
    pub onchange: Callback<Event>,
}

#[function_component(IdSelectionComponent)]
pub fn id_selection(
    Props {
        id_selection:
            IdSelectionProp {
                variant,
                modal_variant_handle,
            },
        onchange,
    }: &Props,
) -> Html {
    let get_id_selection = {
        let variables = Variables {
            variant: variant.clone(),
        };
        use_query_with_deps::<IdSelection, _>(variables, ())
    };

    match get_id_selection {
        QueryResponse {
            data: Some(ResponseData { id_selection }),
            ..
        } => html! {
                <select name="id-select" onchange={onchange.clone()}>
                    {
                        id_selection
                        .into_iter()
                        .map(|IdSelectionIdSelection { id, name }| {
                            html! {
                                <option key={id.to_string()} value={id.to_string()}>{ name }</option>
                            }
                        })
                        .collect::<Html>()
                    }
                </select>
        },
        QueryResponse {
            error: Some(error), ..
        } => {
            modal_variant_handle.emit(Some(ModalVariant::Error(error)));
            Html::default()
        }
        _ => Html::default(),
    }
}
