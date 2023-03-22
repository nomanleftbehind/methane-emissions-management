use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::components::emitters_window::{CompressorSubData, ControllerSubData, TankFarmSubData};

use super::Emitter;

#[derive(PartialEq)]
pub struct EmitterView {
    pub emitter: Emitter,
    pub controller_sub_data: ControllerSubData,
    pub compressor_sub_data: CompressorSubData,
    pub tank_farm_sub_data: TankFarmSubData,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emitter_view: EmitterView,
    pub on_emitter_change: Callback<Emitter>,
}

#[function_component(EmitterPrimaryNavbar)]
pub fn emitter_primary_navbar(
    Props {
        emitter_view:
            EmitterView {
                emitter,
                controller_sub_data,
                compressor_sub_data,
                tank_farm_sub_data,
            },
        on_emitter_change,
    }: &Props,
) -> Html {
    let emitter_vec = vec![
        Emitter::Controller(*controller_sub_data),
        Emitter::Compressor(*compressor_sub_data),
        Emitter::TankFarm(*tank_farm_sub_data),
    ];

    let emitter_iter = emitter_vec.into_iter().enumerate().map(|(key, e)| {
        let on_emitter_change = on_emitter_change.clone();
        let onclick = move |_| {
            on_emitter_change.emit(e);
        };

        html! {
            <div class={classes!("navbar-button-wrapper")}>
                <button {key} {onclick} class={classes!(
                    "emitters-navigation-button",
                    (emitter == &e).then(|| "active")
                )}>{ e }
                </button>
            </div>
        }
    });

    html! {
        <nav class={classes!("emitters-navbar")}>
            { for emitter_iter }
        </nav>
    }
}
