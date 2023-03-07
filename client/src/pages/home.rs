use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Welcome..." }</h1>
                    <h2 class="subtitle">{ "...to the best yew content" }</h2>
                </div>
            </div>

            <div class="tile is-child">
                <figure class="image is-3by1">
                    <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
                </figure>
            </div>

            // <div class="tile is-parent container">
            //     <InfoTiles />
            // </div>
        </div>
    }
}
