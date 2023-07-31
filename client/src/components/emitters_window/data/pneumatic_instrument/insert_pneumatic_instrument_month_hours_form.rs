use crate::{
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument_month_hours::{
            InsertPneumaticInstrumentMonthHoursInput, Variables,
        },
        NaiveDateTime,
    },
    utils::insert_form::*,
};

insert_form!(
    component_name: InsertPneumaticInstrumentMonthHoursForm,
    id: pneumatic_instrument_id,
    id_is_in_input: id_is_in_input,
    has_modal: no_modal,
    form_input: (
        Variables,
        insert_pneumatic_instrument_month_hours_input,
        InsertPneumaticInstrumentMonthHoursInput
    ),
    class: "pneumatic-instrument-month-hours-form",
    state_handles: (
        month,
        mandatory: yes,
        button_disable: disable,
        onchange_date_callback(onchange_month, handle_month),
        date_input_html_callback(grid_column_num: 2)
    ),
    (
        hours_on,
        mandatory: yes,
        button_disable: disable,
        onchange_float_callback(onchange_hours_on, handle_hours_on),
        float_input_html_callback(grid_column_num: 3)
    )
);
