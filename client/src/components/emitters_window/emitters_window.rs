use std::{fmt::Display, rc::Rc};

use uuid::Uuid;
use yew::{classes, function_component, html, use_state_eq, Callback, Html, Properties};

use crate::components::emitters_window::{compressors::CompressorsComp, navbar::Navbar};

use super::controllers::ControllersComp;

#[derive(PartialEq, Clone, Copy)]
pub enum Emitters {
    Controllers,
    Compressors,
    TankFarms,
}

impl Display for Emitters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Emitters::Controllers => write!(f, "Controllers"),
            Emitters::Compressors => write!(f, "Compressors"),
            Emitters::TankFarms => write!(f, "Tank Farms"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
}

#[function_component(EmittersWindow)]
pub fn emitters_window(Props { facility_id }: &Props) -> Html {
    let state = use_state_eq(|| Emitters::Controllers);

    let emitters = (*state).clone();

    let on_emitters_change = Callback::from(move |e: Emitters| {
        state.set(e);
    });

    let m = match emitters {
        Emitters::Controllers => {
            html! {
                <ControllersComp {facility_id} />
            }
        }
        Emitters::Compressors => {
            html! {
                <CompressorsComp {facility_id} />
            }
        }
        Emitters::TankFarms => {
            html! {}
        }
    };

    html! {
        <div class={classes!("emitters-window")}>
            <Navbar {emitters} {on_emitters_change} />
            { m }
        </div>
    }
}
