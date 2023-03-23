use crate::components::emitters_window::{
    compressors::CompressorsComp,
    controllers::ControllersComp,
    emitter_primary_navbar::{EmitterPrimaryNavbar, EmitterView},
    emitter_secondary_navbar::EmitterSecondaryNavbar,
    tank_farms::TankFarmsComp,
    CompressorSubData, ControllerSubData, Emitter, TankFarmSubData,
};
use std::rc::Rc;
use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(EmittersWindow)]
pub fn emitters_window(Props { facility_id }: &Props) -> Html {
    let controller_sub_data_state = use_state_eq(|| ControllerSubData::Controller);
    let compressor_sub_data_state = use_state_eq(|| CompressorSubData::Compressor);
    let tank_farm_sub_data_state = use_state_eq(|| TankFarmSubData::TankFarm);
    let emitter_state = use_state_eq(|| Emitter::Controller(ControllerSubData::Controller));

    let emitter = *emitter_state;
    let controller_sub_data = *controller_sub_data_state;
    let compressor_sub_data = *compressor_sub_data_state;
    let tank_farm_sub_data = *tank_farm_sub_data_state;

    let emitter_view = EmitterView {
        emitter,
        controller_sub_data,
        compressor_sub_data,
        tank_farm_sub_data,
    };

    let on_emitter_change = Callback::from(move |e: Emitter| {
        emitter_state.set(e);
    });

    let on_controller_sub_data_change = Callback::from(move |csd: ControllerSubData| {
        controller_sub_data_state.set(csd);
    });
    let on_compressor_sub_data_change = Callback::from(move |csd: CompressorSubData| {
        compressor_sub_data_state.set(csd);
    });
    let on_tank_farm_sub_data_change = Callback::from(move |tfsd: TankFarmSubData| {
        tank_farm_sub_data_state.set(tfsd);
    });

    let view = match emitter {
        Emitter::Controller(controller_sub_data) => match controller_sub_data {
            ControllerSubData::Controller => {
                html! {
                    <ControllersComp {facility_id} />
                }
            }
            ControllerSubData::ControllerChange => {
                html! {
                    <div>{ "Controller Change" }</div>
                }
            }
            ControllerSubData::ControllerMonthHours => {
                html! {
                    <div>{ "Controller Month Hours" }</div>
                }
            }
            ControllerSubData::ControllerMonthVent => {
                html! {
                    <div>{ "Controller Month Vent" }</div>
                }
            }
        },
        Emitter::Compressor(compressor_sub_data) => match compressor_sub_data {
            CompressorSubData::Compressor => {
                html! {
                    <CompressorsComp {facility_id} />
                }
            }
            CompressorSubData::CompressorChange => {
                html! {
                    <div>{ "Compressor Change" }</div>
                }
            }
            CompressorSubData::CompressorMonthHours => {
                html! {
                    <div>{ "Compressor Month Hours" }</div>
                }
            }
            CompressorSubData::CompressorMonthVent => {
                html! {
                    <div>{ "Compressor Month Vent" }</div>
                }
            }
        },
        Emitter::TankFarm(tank_farm_sub_data) => match tank_farm_sub_data {
            TankFarmSubData::TankFarm => {
                html! {
                    <TankFarmsComp {facility_id} />
                }
            }
            TankFarmSubData::TankFarmChange => {
                html! {
                    <div>{ "Tank Farm Change" }</div>
                }
            }
            TankFarmSubData::TankFarmMonthHours => {
                html! {
                    <div>{ "Tank Farm Month Hours" }</div>
                }
            }
            TankFarmSubData::TankFarmMonthVent => {
                html! {
                    <div>{ "Tank Farm Month Vent" }</div>
                }
            }
        },
    };

    html! {
        <div class={classes!("emitters-window")}>
            <EmitterPrimaryNavbar {emitter_view} on_emitter_change={on_emitter_change.clone()} />
            <EmitterSecondaryNavbar
                {emitter}
                {on_emitter_change}
                {on_controller_sub_data_change}
                {on_compressor_sub_data_change}
                {on_tank_farm_sub_data_change}
            />
            <div />
            { view }
        </div>
    }
}
