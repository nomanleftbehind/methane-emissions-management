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
    pub null_option: bool,
    pub value: String,
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
        null_option,
        value,
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
                    if *null_option {
                        <option key="" value="" selected={value == ""}>{ "None" }</option>
                    }
                    {
                        id_selection
                        .into_iter()
                        .map(|IdSelectionIdSelection { id, name }| {
                            let id_string = id.to_string();
                            let selected = &id_string == value;
                            html! {
                                <option key={id_string.clone()} value={id_string} {selected}>{ name }</option>
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
