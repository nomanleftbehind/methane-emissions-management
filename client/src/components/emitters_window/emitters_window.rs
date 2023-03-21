use crate::components::emitters_window::{
    compressors::CompressorsComp,
    controllers::ControllersComp,
    navbar::{EmittersData, Navbar},
    secondary_navbar::SecondaryNavbar,
    CompressorSubData, ControllerSubData, Emitters, TankFarmSubData,
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

    let emitters_state = use_state_eq(|| Emitters::Controllers(ControllerSubData::Controller));

    let emitters = *emitters_state;
    let controller_sub_data = *controller_sub_data_state;
    let compressor_sub_data = *compressor_sub_data_state;
    let tank_farm_sub_data = *tank_farm_sub_data_state;

    let emitters_data = EmittersData {
        emitters,
        controller_sub_data,
        compressor_sub_data,
        tank_farm_sub_data,
    };

    let on_emitters_change = Callback::from(move |e: Emitters| {
        emitters_state.set(e);
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

    let m = match emitters {
        Emitters::Controllers(controller_sub_data) => match controller_sub_data {
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
        Emitters::Compressors(compressor_sub_data) => match compressor_sub_data {
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
        Emitters::TankFarms(tank_farm_sub_data) => match tank_farm_sub_data {
            TankFarmSubData::TankFarm => {
                html! {
                    <div>{ "Tank Farm" }</div>
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
            <Navbar {emitters_data} on_emitters_change={on_emitters_change.clone()} />
            <SecondaryNavbar
                {emitters}
                {on_emitters_change}
                {on_controller_sub_data_change}
                {on_compressor_sub_data_change}
                {on_tank_farm_sub_data_change}
            />
            { m }
        </div>
    }
}
