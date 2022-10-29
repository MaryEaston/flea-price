use num::*;

use yew::prelude::*;
use yew_router::prelude::*;

use chrono::Local;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/eq/:id")]
    Equation { id: i64 },
}

fn get_t() -> i64{
    let sec: i64 = Local::now().timestamp() - 1666985400;
    let t: i64 = sec / 60;
    return t;
}

fn eq_str(id:i64) -> String{
    let eq: &str;
    match id {
        1 => eq = "\\log_2 t",
        _ => eq = "\\mathrm{Error!}",
    }
    return eq.to_string();
}

fn calc(time:i64, id:i64) -> i64{
    let value: i64;
    let time: i64 = time / 420;
    match id {
        1 => {
            value = (time as f64).log(2.0) as i64;
        },
        _ => {
            value = 99999999;
        },
    }
    return value;
}

fn switch(route: &Route) -> Html {
    let time: i64 = get_t();
    match route {
        Route::Home => {
            html! { <h1>{ "Home" }</h1> }
        },
        Route::Post { id } => {
            html! {<p>{format!("You are looking at Post {}", id)}</p>}
        },
        Route::Equation { id } => {
            html! {
                <div id="price">
                    <p id="eq">{"$"}{eq_str(*id)}{"$"}</p>
                    <p id="time">{"$t = "}{time}{"$分"}</p>
                    <p id="prise">
                        <span id="value">{calc(time, *id)}</span>
                        <span id="yen">{"円"}</span>
                    </p>
                </div>
            }
        }
    }
}

enum Msg {
}

struct Model {
}

impl Model{
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div id="app">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                <p id="reload"><a href="">{"reload"}</a></p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
