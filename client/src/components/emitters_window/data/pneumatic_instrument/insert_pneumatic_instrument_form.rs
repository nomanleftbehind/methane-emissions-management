use crate::{
    components::emitters_window::data::entry::{DropdownSelectionComponent, DropdownSelectionProp},
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument::{
            InsertPneumaticInstrumentInput, Variables,
        },
        queries::dropdown_selection::get_dropdown_selection::DropdownSelectionVariant,
        NaiveDateTime,
    },
    pages::ModalVariant,
    utils::insert_form::*,
};
use common::PneumaticInstrumentType;

insert_form!(
    component_name: InsertPneumaticInstrumentForm,
    id: facility_id,
    id_is_in_input: id_is_not_in_input,
    has_modal: has_modal,
    form_input: (
        Variables,
        insert_pneumatic_instrument_input,
        InsertPneumaticInstrumentInput
    ),
    class: "pneumatic-instrument-form",
    state_handles:
    (
        type_,
        mandatory: yes,
        button_disable: disable,
        onchange_pneumatic_instrument_type_callback(onchange_type, handle_type),
        pneumatic_instrument_type_input_html_callback(grid_column_num: 3)
    ),
    (
        manufacturer_id,
        mandatory: yes,
        button_disable: disable,
        onchange_uuid_callback(onchange_manufacturer_id, handle_manufacturer_id),
        device_manufacturer_id_input_html_callback(grid_column_num: 4)
    ),
    (
        model,
        mandatory: no,
        button_disable: enable,
        onchange_string_callback(onchange_model, handle_model),
        string_input_html_callback(grid_column_num: 5)
    ),
    (
        serial_number,
        mandatory: no,
        button_disable: enable,
        onchange_string_callback(onchange_serial_number, handle_serial_number),
        string_input_html_callback(grid_column_num: 6)
    ),
    (
        start_date,
        mandatory: yes,
        button_disable: disable,
        onchange_date_callback(onchange_start_date, handle_start_date),
        date_input_html_callback(grid_column_num: 7)
    ),
    (
        end_date,
        mandatory: no,
        button_disable: enable,
        onchange_date_callback(onchange_end_date, handle_end_date),
        date_input_html_callback(grid_column_num: 8)
    ),
    (
        site_id,
        mandatory: yes,
        button_disable: disable,
        onchange_uuid_callback(onchange_site_id, handle_site_id),
        site_id_input_html_callback(grid_column_num: 9)
    )
);
