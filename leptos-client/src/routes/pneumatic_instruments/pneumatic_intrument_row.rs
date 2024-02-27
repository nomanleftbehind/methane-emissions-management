use crate::{
    components::delete_entry::DeleteEntry,
    models::{
        mutations::manual_mutation::delete_entry::{self, DeleteEntryVariant},
        queries::pneumatic_instrument::get_pneumatic_instruments::GetPneumaticInstrumentsGetPneumaticInstruments,
    },
};
use leptos::*;

#[component]
pub fn pneumatic_instrument_row(
    pneumatic_instrument: Signal<GetPneumaticInstrumentsGetPneumaticInstruments>,
    row_num: Signal<usize>,
    // pneumatic_instrument: GetPneumaticInstrumentsGetPneumaticInstruments,
    // handle_update_field: Callback<VariablesUpdateField>,
    #[prop(into)] handle_delete_entry: Callback<delete_entry::Variables>,
    // modal_variant_handle: Callback<Option<ModalVariant>>,
) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(false);
    // let expanded_handle = use_state_eq(|| false);
    // let expanded = *expanded_handle;

    // let handle_expand_data = Callback::from(move |_| {
    //     expanded_handle.set(!expanded);
    // });
    let handle_expand_data = move || set_expanded.set(expanded.with(|e| !e));

    // let pneumatic_instrument = pneumatic_instrument.clone();
    let pneumatic_instrument = pneumatic_instrument.get();
    let GetPneumaticInstrumentsGetPneumaticInstruments {
        id,
        site,
        manufacturer,
        created_by,
        updated_by,
        type_,
        model,
        serial_number,
        start_date,
        end_date,
        created_at,
        updated_at,
        ..
    } = pneumatic_instrument;
    let site = site.map(|s| s.name);
    let manufacturer = manufacturer.map(|m| m.manufacturer);
    let end_date = end_date.map(|ed| ed.to_string());
    let created_by = created_by.map(|cb| cb.email);
    let updated_by = updated_by.map(|ub| ub.email);

    // let sidebar_items = vec![
    //     PneumaticInstrumentEmissionRate,
    //     PneumaticInstrumentMonthHours,
    //     PneumaticInstrumentMonthMethaneEmissionOverride,
    //     PneumaticInstrumentMonthMethaneEmission,
    // ];

    // let grid

    view! {
        <DeleteEntry
            id=Signal::derive(move || id)
            row_num=row_num
            col_num=Signal::derive(|| 1)
            delete_entry_variant=Signal::derive(|| DeleteEntryVariant::PNEUMATIC_INSTRUMENT)
            handle_delete_entry=handle_delete_entry
        />
        <div style:grid-column=3 style:grid-row=row_num>
            {type_.to_string()}
        </div>
        <div style:grid-column=4 style:grid-row=row_num>
            {manufacturer}
        </div>
        <div style:grid-column=5 style:grid-row=row_num>
            {model}
        </div>
        <div style:grid-column=6 style:grid-row=row_num>
            {serial_number}
        </div>
        <div style:grid-column=7 style:grid-row=row_num>
            {start_date.to_string()}
        </div>
        <div style:grid-column=8 style:grid-row=row_num>
            {end_date}
        </div>
        <div style:grid-column=9 style:grid-row=row_num>
            {site}
        </div>
        <div style:grid-column=10 style:grid-row=row_num>
            {created_by}
        </div>
        <div style:grid-column=11 style:grid-row=row_num>
            {created_at.to_string()}
        </div>
        <div style:grid-column=12 style:grid-row=row_num>
            {updated_by}
        </div>
        <div style:grid-column=13 style:grid-row=row_num>
            {updated_at.to_string()}
        </div>
        <div style:grid-column=14 style:grid-row=row_num>
            {id.to_string()}
        </div>
    }
}
