use yew::prelude::*;
use yew_router::prelude::*;

use chrono::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/eq/:id")]
    Equation { id: i64 },
}

fn time_str() -> String{
    let dt = Utc::now();
    return format!("{}", dt.format("%H:%M:%S"));
}

fn get_t() -> i64{
    let sec: i64 = Local::now().timestamp() - 1667091600;
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
                <>
                    <div id="eq_box">
                        <p id="text">{"価格関数 :"}</p>
                        <p id="eq">{"$"}{eq_str(*id)}{"$"}</p>
                    </div>
                    <div id="now">
                        <p id="text">{"ただいまの価格 :"}</p>
                        <p id="price">
                            <span id="value">{calc(time, *id)}</span>
                            <span id="yen">{"円"}</span>
                        </p>
                        <p id="text">{time_str()}</p>
                        <p id="text">{"$t = "}{time}{"$ [分]"}</p>
                    </div>
                </>
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

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let _link = ctx.link();
        html! {
            <>
            <div id="title">
                <p>{"大岡山最終処分場"}</p>
                <hr/>
            </div>
            <div id="main">
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                <p id="reload"><a href="">{"更新"}</a></p>
            </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
