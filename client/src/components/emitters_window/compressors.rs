use crate::{
    components::emitters_window::{
        delete_entry::DeleteEntryComponent,
        entry::{EditFieldProp, Entry},
        expand_data::ExpandDataComponent,
    },
    hooks::{lazy_query, use_query_with_deps, QueryResponse},
    models::{
        mutations::manual_mutation::{
            delete_entry::{
                DeleteEntryVariant::COMPRESSOR, ResponseData as ResponseDataDeleteEntry,
                Variables as VariablesDeleteEntry,
            },
            update_field::{
                ResponseData as ResponseDataUpdateField,
                UpdateFieldVariant::{
                    COMPRESSOR_FDC_REC_ID, COMPRESSOR_INSTALL_DATE, COMPRESSOR_NAME,
                    COMPRESSOR_REMOVE_DATE, COMPRESSOR_SERIAL_NUMBER,
                },
                Variables as VariablesUpdateField,
            },
            DeleteEntry, UpdateField,
        },
        queries::compressor::{
            get_compressors::{EmittersByInput, ResponseData, Variables},
            GetCompressors,
        },
    },
    utils::{console_log, gen_style::gen_grid_style},
};
use common::UpdateFieldValueEnum::{
    NaiveDateTimeValue, NaiveDateValue, OptionNaiveDateValue, OptionStringValue, StringValue,
    UuidValue,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

/// In an effort to avoid cloning large amounts of data to create props when re-rendering,
/// a smart pointer is passed in props to only clone a reference to the data instead of the data itself.
#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(CompressorsComp)]
pub fn compressors_comp(Props { facility_id }: &Props) -> Html {
    let number_of_updated_fields_handle = use_state_eq(|| 0);
    let number_of_updated_fields = *number_of_updated_fields_handle;

    let get_compressors = {
        let variables = Variables {
            by: EmittersByInput {
                facility_id: **facility_id,
            },
        };
        use_query_with_deps::<GetCompressors, _>(
            variables,
            (facility_id.clone(), number_of_updated_fields),
        )
    };

    let handle_update_field = {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        Callback::from(move |variables: VariablesUpdateField| {
            let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match lazy_query::<UpdateField>(variables).await {
                    QueryResponse {
                        data: Some(ResponseDataUpdateField { update_field }),
                        ..
                    } => {
                        number_of_updated_fields_handle.set(number_of_updated_fields + update_field)
                    }
                    QueryResponse { error: Some(e), .. } => {
                        console_log!("Update error: {}", e);
                    }
                    _ => {}
                };
            });
        })
    };

    let handle_delete_entry = Callback::from(move |variables: VariablesDeleteEntry| {
        let number_of_updated_fields_handle = number_of_updated_fields_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match lazy_query::<DeleteEntry>(variables).await {
                QueryResponse {
                    data: Some(ResponseDataDeleteEntry { delete_entry }),
                    ..
                } => number_of_updated_fields_handle.set(number_of_updated_fields + delete_entry),
                QueryResponse { error: Some(e), .. } => {
                    console_log!("Delete error: {}", e);
                }
                _ => {}
            };
        });
    });

    let compressors = match get_compressors {
        QueryResponse {
            data: Some(ResponseData { compressors_by }),
            ..
        } => {
            let compressors_iter = compressors_by.into_iter().enumerate().map(|(row_num, c)| {
                let id = c.id;
                let created_by = c.created_by.map(|cb| cb.email);
                let updated_by = c.updated_by.map(|ub| ub.email);
                let row_num = row_num + 2;
                html! {
                    <>
                        <DeleteEntryComponent {id} col_num={1} {row_num} delete_entry_variant={COMPRESSOR} handle_delete_entry={handle_delete_entry.clone()} />
                        // <ExpandDataComponent {id} col_num={2} {row_num} />
                        <Entry {id} col_num={3} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: COMPRESSOR_NAME}} value={StringValue(c.name)} />
                        <Entry {id} col_num={4} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: COMPRESSOR_SERIAL_NUMBER}} value={StringValue(c.serial_number)} />
                        <Entry {id} col_num={5} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: COMPRESSOR_INSTALL_DATE}} value={NaiveDateValue(c.install_date)} />
                        <Entry {id} col_num={6} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: COMPRESSOR_REMOVE_DATE}} value={OptionNaiveDateValue(c.remove_date)} />
                        <Entry {id} col_num={7} {row_num} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: COMPRESSOR_FDC_REC_ID}} value={StringValue(c.fdc_rec_id)} />
                        <Entry {id} col_num={8} {row_num} value={OptionStringValue(created_by)} />
                        <Entry {id} col_num={9} {row_num} value={NaiveDateTimeValue(c.created_at)} />
                        <Entry {id} col_num={10} {row_num} value={OptionStringValue(updated_by)} />
                        <Entry {id} col_num={11} {row_num} value={NaiveDateTimeValue(c.updated_at)} />
                        <Entry {id} col_num={12} {row_num} value={UuidValue(id)} />
                    </>
                }
            });
            html! { for compressors_iter }
        }
        QueryResponse { error: Some(e), .. } => {
            html! {e}
        }
        _ => html! {},
    };

    html! {
        <div class={classes!("emitters", "compressors")}>
            <div class={classes!("sticky")} style={gen_grid_style(1, 1)}></div>
            <div class={classes!("sticky")} style={gen_grid_style(2, 1)}></div>
            <div class={classes!("sticky")} style={gen_grid_style(3, 1)}>{ "Name" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(4, 1)}>{ "Serial Number" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(5, 1)}>{ "Install Date" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(6, 1)}>{ "Remove Date" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(7, 1)}>{ "FDC ID" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(8, 1)}>{ "Created By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(9, 1)}>{ "Created At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(10, 1)}>{ "Updated By" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(11, 1)}>{ "Updated At" }</div>
            <div class={classes!("sticky")} style={gen_grid_style(12, 1)}>{ "ID" }</div>
            { compressors }
        </div>
    }
}
