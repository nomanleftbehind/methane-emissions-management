use super::{
    insert_pneumatic_instrument_form::InsertPneumaticInstrumentForm,
    pneumatic_instrument_row::PneumaticInstrumentRowComponent,
};
use crate::{
    models::{
        mutations::pneumatic_instrument::{
            insert_pneumatic_instrument::{
                ResponseData as ResponseDataInsertPneumaticInstrument,
                Variables as VariablesInsertPneumaticInstrument,
            },
            InsertPneumaticInstrument,
        },
        queries::pneumatic_instrument::{
            get_pneumatic_instruments::{
                GetPneumaticInstrumentsInput, PneumaticInstrumentsByVariant, ResponseData,
                Variables,
            },
            GetPneumaticInstruments,
        },
    },
    utils::get_data_component::get_data_component,
};

get_data_component!(
    component_name: PneumaticInstrumentsComponent,
    id: facility_id,
    class: "pneumatic-instruments",
    get_data_input: (
        get_pneumatic_instruments,
        Variables,
        get_pneumatic_instruments_input,
        GetPneumaticInstrumentsInput,
        PneumaticInstrumentsByVariant::FACILITY_ID,
        GetPneumaticInstruments,
        ResponseData
    ),
    insert_input: (VariablesInsertPneumaticInstrument, InsertPneumaticInstrument, ResponseDataInsertPneumaticInstrument, insert_pneumatic_instrument, InsertPneumaticInstrumentForm),
    data_row: pneumatic_instrument,
    column_names: "", "Type", "Manufacturer", "Model", "Serial Number", "StartDate", "End Date", "Site", "Created By", "Created At", "Updated By", "Updated At", "ID",
    column_start: 2
);
