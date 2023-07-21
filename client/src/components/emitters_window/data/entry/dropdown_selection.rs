use crate::{
    hooks::{use_query_with_deps, QueryResponse},
    models::queries::dropdown_selection::{
        get_dropdown_selection::{
            DropdownSelectionVariant, GetDropdownSelectionGetDropdownSelection,
            GetDropdownSelectionInput, ResponseData, Variables,
        },
        GetDropdownSelection,
    },
    pages::ModalVariant,
};
use uuid::Uuid;
use yew::{function_component, html, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct DropdownSelectionProp {
    pub variant: DropdownSelectionVariant,
    pub id: Option<Uuid>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub dropdown_selection: DropdownSelectionProp,
    pub onchange: Callback<Event>,
    pub offer_null: Option<bool>,
    pub value: Option<String>,
    pub col_num: Option<usize>,
}

#[function_component(DropdownSelectionComponent)]
pub fn dropdown_selection(
    Props {
        dropdown_selection:
            DropdownSelectionProp {
                variant,
                id,
                modal_variant_handle,
            },
        onchange,
        offer_null,
        value,
        col_num,
    }: &Props,
) -> Html {
    let get_dropdown_selection = {
        let variables = Variables {
            get_dropdown_selection_input: GetDropdownSelectionInput {
                variant: variant.clone(),
                id: *id,
            },
        };
        use_query_with_deps::<GetDropdownSelection, _>(variables, ())
    };

    let style = col_num.map(|col_num| format!("grid-row: 1; grid-column: {};", col_num));

    let offer_null = offer_null.map_or_else(|| true, |offer_null| offer_null);

    match get_dropdown_selection {
        QueryResponse {
            data: Some(ResponseData {
                get_dropdown_selection,
            }),
            ..
        } => html! {
                <select name="id-select" {style} onchange={onchange.clone()}>
                    if offer_null {
                        <option key="" value="" selected={value.is_none()}>{ "None" }</option>
                    }
                    {
                        get_dropdown_selection
                        .into_iter()
                        .map(|GetDropdownSelectionGetDropdownSelection { id, name }| {
                            let selected = Some(&id) == value.as_ref();
                            let name = name.map_or_else(|| id.clone(), |name| name);
                            html! {
                                <option key={id.clone()} value={id} {selected}>{ name }</option>
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
