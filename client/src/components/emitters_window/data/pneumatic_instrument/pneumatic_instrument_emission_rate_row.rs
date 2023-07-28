use crate::{
    components::emitters_window::{
        data::entry::{EditFieldProp, Entry},
        delete_entry::DeleteEntryComponent,
    },
    models::{
        mutations::manual_mutation::{
            delete_entry::{DeleteEntryVariant, Variables as VariablesDeleteEntry},
            update_field::{UpdateFieldVariant, Variables as VariablesUpdateField},
        },
        queries::pneumatic_instrument::get_pneumatic_instrument_emission_rates::GetPneumaticInstrumentEmissionRatesGetPneumaticInstrumentEmissionRates,
    },
};
use common::UpdateFieldValueEnum::{
    FloatValue, NaiveDateTimeValue, NaiveDateValue, OptionStringValue, StringValue, UuidValue,
};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub pneumatic_instrument_emission_rate:
        GetPneumaticInstrumentEmissionRatesGetPneumaticInstrumentEmissionRates,
    pub handle_update_field: Callback<VariablesUpdateField>,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(PneumaticInstrumentEmissionRateRowComponent)]
pub fn pneumatic_instrument_emission_rate_row_component(
    Props {
        row_num,
        pneumatic_instrument_emission_rate,
        handle_update_field,
        handle_delete_entry,
    }: &Props,
) -> Html {
    let pneumatic_instrument_emission_rate = pneumatic_instrument_emission_rate.clone();
    let id = pneumatic_instrument_emission_rate.id;
    let created_by = pneumatic_instrument_emission_rate
        .created_by
        .map(|cb| cb.email);
    let updated_by = pneumatic_instrument_emission_rate
        .updated_by
        .map(|ub| ub.email);

    html! {
        <>
            <DeleteEntryComponent {id} {row_num} col_num={1} delete_entry_variant={DeleteEntryVariant::PNEUMATIC_INSTRUMENT_EMISSION_RATE} handle_delete_entry={handle_delete_entry.clone()} />
            <Entry {id} {row_num} col_num={2} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_EMISSION_RATE_DATE}} value={NaiveDateValue(pneumatic_instrument_emission_rate.date)} />
            <Entry {id} {row_num} col_num={3} edit_field={EditFieldProp {handle_update_field: handle_update_field.clone(), update_field_variant: UpdateFieldVariant::PNEUMATIC_INSTRUMENT_EMISSION_RATE_RATE}} value={FloatValue(pneumatic_instrument_emission_rate.rate)} display_value={StringValue(format!("{:.4}", pneumatic_instrument_emission_rate.rate))} />
            <Entry {id} {row_num} col_num={4} value={OptionStringValue(created_by)} />
            <Entry {id} {row_num} col_num={5} value={NaiveDateTimeValue(pneumatic_instrument_emission_rate.created_at)} />
            <Entry {id} {row_num} col_num={6} value={OptionStringValue(updated_by)} />
            <Entry {id} {row_num} col_num={7} value={NaiveDateTimeValue(pneumatic_instrument_emission_rate.updated_at)} />
            <Entry {id} {row_num} col_num={8} value={UuidValue(id)} />
        </>
    }
}
