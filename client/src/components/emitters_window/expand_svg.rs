use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expanded: bool,
}

#[function_component(ExpandSvgComponent)]
pub fn expand_svg(Props { expanded }: &Props) -> Html {
    let transform = if !expanded {
        "rotate(90 0 0) translate(0 -24)"
    } else {
        "rotate(270 0 0) translate(-24 0)"
    };

    html! {
        <svg width="24" height="24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <g {transform} >
                <path d="m9 5.007 7 7-7 7" stroke="#000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </g>
        </svg>
    }
}
