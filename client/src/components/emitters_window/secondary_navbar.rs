use crate::components::emitters_window::{
    CompressorSubData, ControllerSubData, Emitters, TankFarmSubData,
};
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emitters: Emitters,
    pub on_emitters_change: Callback<Emitters>,
    pub on_controller_sub_data_change: Callback<ControllerSubData>,
    pub on_compressor_sub_data_change: Callback<CompressorSubData>,
    pub on_tank_farm_sub_data_change: Callback<TankFarmSubData>,
}

#[function_component(SecondaryNavbar)]
pub fn secondary_navbar(
    Props {
        emitters,
        on_emitters_change,
        on_controller_sub_data_change,
        on_compressor_sub_data_change,
        on_tank_farm_sub_data_change,
    }: &Props,
) -> Html {
    let sub_data = match emitters {
        Emitters::Controllers(_) => {
            let controller_sub_data_vec = vec![
                ControllerSubData::Controller,
                ControllerSubData::ControllerChange,
                ControllerSubData::ControllerMonthHours,
                ControllerSubData::ControllerMonthVent,
            ];

            let controller_sub_data_iter =
                controller_sub_data_vec
                    .into_iter()
                    .enumerate()
                    .map(|(key, csd)| {
                        let on_controller_sub_data_change = on_controller_sub_data_change.clone();
                        let on_emitters_change = on_emitters_change.clone();
                        let onclick = move |_| {
                            on_controller_sub_data_change.emit(csd);
                            on_emitters_change.emit(Emitters::Controllers(csd));
                        };

                        html! {
                            <div class={classes!("navbar-button-wrapper")}>
                                <button {key} {onclick} class={classes!(
                                    "emitters-navigation-button",
                                    (emitters == &csd).then(|| "active")
                                )}>{ csd }
                                </button>
                            </div>
                        }
                    });
            html! {
                <>{ for controller_sub_data_iter }</>
            }
        }
        Emitters::Compressors(_) => {
            let compressor_sub_data_vec = vec![
                CompressorSubData::Compressor,
                CompressorSubData::CompressorChange,
                CompressorSubData::CompressorMonthHours,
                CompressorSubData::CompressorMonthVent,
            ];

            let compressor_sub_data_iter =
                compressor_sub_data_vec
                    .into_iter()
                    .enumerate()
                    .map(|(key, csd)| {
                        let on_compressor_sub_data_change = on_compressor_sub_data_change.clone();
                        let on_emitters_change = on_emitters_change.clone();
                        let onclick = move |_| {
                            on_compressor_sub_data_change.emit(csd);
                            on_emitters_change.emit(Emitters::Compressors(csd));
                        };

                        html! {
                            <div class={classes!("navbar-button-wrapper")}>
                                <button {key} {onclick} class={classes!(
                                    "emitters-navigation-button",
                                    (emitters == &csd).then(|| "active")
                                )}>{ csd }
                                </button>
                            </div>
                        }
                    });
            html! {
                <>{ for compressor_sub_data_iter }</>
            }
        }
        Emitters::TankFarms(_) => {
            let tank_farm_sub_data_vec = vec![
                TankFarmSubData::TankFarm,
                TankFarmSubData::TankFarmChange,
                TankFarmSubData::TankFarmMonthHours,
                TankFarmSubData::TankFarmMonthVent,
            ];

            let tank_farm_sub_data_iter =
                tank_farm_sub_data_vec
                    .into_iter()
                    .enumerate()
                    .map(|(key, tfsd)| {
                        let on_tank_farm_sub_data_change = on_tank_farm_sub_data_change.clone();
                        let on_emitters_change = on_emitters_change.clone();
                        let onclick = move |_| {
                            on_tank_farm_sub_data_change.emit(tfsd);
                            on_emitters_change.emit(Emitters::TankFarms(tfsd));
                        };

                        html! {
                            <div class={classes!("navbar-button-wrapper")}>
                                <button {key} {onclick} class={classes!(
                                    "emitters-navigation-button",
                                    (emitters == &tfsd).then(|| "active")
                                )}>{ tfsd }
                                </button>
                            </div>
                        }
                    });
            html! {
                <>{ for tank_farm_sub_data_iter }</>
            }
        }
    };

    html! {
        <nav class={classes!("emitters-navbar")}>
            { sub_data }
        </nav>
    }
}
