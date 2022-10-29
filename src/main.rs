use yew::prelude::*;
use yew_router::prelude::*;

use chrono::Local;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    // #[at("/")]
    // Home,
    #[at("/eq/:id")]
    Equation { id: i64 },
}

fn time_str() -> String{
    let dt = Local::now();
    return format!("{}", dt.format("%H:%M:%S"));
}

fn get_t() -> f64{
    let sec: i64 = Local::now().timestamp() - 1667091600;
    let time: f64 = (sec as f64) / 60.0 / 420.0;
    return time;
}

fn round_str(value : f64) -> String{
    let rounded = (value * 100.0).round() / 100.0;
    return format!("{}", rounded);
}

fn eq_str(id:i64) -> String{
    let eq: &str;
    match id {
        1 => eq = "\\log_2 t",
        _ => eq = "\\mathrm{Error!}",
    }
    return eq.to_string();
}

fn calc(time:f64, id:i64) -> i64{
    let value: i64;
    match id {
        1 => {
            value = time.log(2.0) as i64;
        },
        _ => {
            value = 99999999;
        },
    }
    return value;
}

fn switch(route: &Route) -> Html {
    let time : f64 = get_t();
    match route {
        // Route::Home => {
        //     html! {
        //         <h1>{ "Home" }</h1>
        //     }
        // },
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
                        <p id="text">{"$t = "}{round_str(time)}{"$ [$\\times$ 420 分]"}</p>
                    </div>
                    <div id="footer">
                        <p id="text"><a href="">{"更新"}</a></p>
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
            </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
