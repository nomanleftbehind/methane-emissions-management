use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::components::emitters_window::{CompressorSubData, ControllerSubData, TankFarmSubData};

use super::Emitters;

#[derive(PartialEq)]
pub struct EmittersData {
    pub emitters: Emitters,
    pub controller_sub_data: ControllerSubData,
    pub compressor_sub_data: CompressorSubData,
    pub tank_farm_sub_data: TankFarmSubData,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub emitters_data: EmittersData,
    pub on_emitters_change: Callback<Emitters>,
}

#[function_component(Navbar)]
pub fn navbar(
    Props {
        emitters_data:
            EmittersData {
                emitters,
                controller_sub_data,
                compressor_sub_data,
                tank_farm_sub_data,
            },
        on_emitters_change,
    }: &Props,
) -> Html {
    let emitter_vec = vec![
        Emitters::Controllers(*controller_sub_data),
        Emitters::Compressors(*compressor_sub_data),
        Emitters::TankFarms(*tank_farm_sub_data),
    ];

    let emitter_iter = emitter_vec.into_iter().enumerate().map(|(key, e)| {
        let on_emitters_change = on_emitters_change.clone();
        let onclick = move |_| {
            on_emitters_change.emit(e);
        };

        html! {
            <div class={classes!("navbar-button-wrapper")}>
                <button {key} {onclick} class={classes!(
                    "emitters-navigation-button",
                    (emitters == &e).then(|| "active")
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
