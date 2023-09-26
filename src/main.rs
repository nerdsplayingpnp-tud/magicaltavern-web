mod types;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar/>
            <h1>{ "Hello yew and trunk! :)" }</h1>
            <p>{"This is some text."}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(Navbar)]
fn navbar() -> Html {
    html! {
        <>
            <header class="p-3 text-bg-dark">
                <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
                <a href="/" class="navbar-brand">
                    <img class="bi me-2" src="public/img/dice-anim-fix.gif" style="width: 32px; height: 32px" alt="Ein rotierender, dunkelblauer D20, welcher nach einer 180-Grad-Rotation in Regenbogenfarben erscheint, und nach einer weiteren 180-Grad-Drehung wieder dunkelblau ist."/>
                </a>

                <ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
                    <li><a href="#" class="nav-link px-2 text-secondary">{"Home"}</a></li>
                    <li><a href="#" class="nav-link px-2 text-white">{"Features"}</a></li>
                    <li><a href="#" class="nav-link px-2 text-white">{"Pricing"}</a></li>
                    <li><a href="#" class="nav-link px-2 text-white">{"FAQs"}</a></li>
                    <li><a href="#" class="nav-link px-2 text-white">{"About"}</a></li>
                </ul>

                <form class="col-12 col-lg-auto mb-3 mb-lg-0 me-lg-3" role="search">
                    <input type="search" class="form-control form-control-dark text-bg-dark" placeholder="Search..." aria-label="Search"/>
                </form>

                <div class="text-end">
                    <button type="button" class="btn btn-outline-light me-2">{"Login"}</button>
                    <button type="button" class="btn btn-warning">{"Sign-up"}</button>
                </div>
                </div>
            </header>
        </>
    }
}
