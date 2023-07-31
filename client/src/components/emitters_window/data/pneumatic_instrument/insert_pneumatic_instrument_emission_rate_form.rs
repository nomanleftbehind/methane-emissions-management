use crate::{
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument_emission_rate::{
            InsertPneumaticInstrumentEmissionRateInput, Variables,
        },
        NaiveDateTime,
    },
    utils::insert_form::*,
};

insert_form!(
    component_name: InsertPneumaticInstrumentEmissionRateForm,
    id: pneumatic_instrument_id,
    id_is_in_input: id_is_in_input,
    has_modal: no_modal,
    form_input: (
        Variables,
        insert_pneumatic_instrument_emission_rate_input,
        InsertPneumaticInstrumentEmissionRateInput
    ),
    class: "pneumatic-instrument-emission-rate-form",
    state_handles: (
        date,
        mandatory: yes,
        button_disable: disable,
        onchange_date_callback(onchange_date, handle_date),
        date_input_html_callback(grid_column_num: 2)
    ),
    (
        rate,
        mandatory: yes,
        button_disable: disable,
        onchange_float_callback(onchange_rate, handle_rate),
        float_input_html_callback(grid_column_num: 3)
    )
);
