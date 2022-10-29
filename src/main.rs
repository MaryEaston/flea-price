use yew::prelude::*;
use yew_router::prelude::*;

use chrono::Local;

use  std::f64::consts::*;

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
        1 => eq = "y= \\left\\lceil 3\\sin \\left( 10\\pi x \\right)+1 \\right\\rceil \\times 10",
        2 => eq = "y= \\left\\lceil 2\\sin \\left( 10\\pi x \\right)+4 \\right\\rceil \\times 10",
        3 => eq = "y=\\left\\lceil 200 \\times \\frac{x+1}{x^{2}+x+1} \\right\\rceil \\times 10",
        4 => eq = "y=\\left\\lceil 200 \\times \\frac{x+1}{x^{2}+x+1} \\right\\rceil \\times 10",
        5 => eq = "y=\\left\\lceil 50 \\times \\frac{2x^{2}-x+1}{4x^{2}-x+1} \\right\\rceil \\times 10",
        6 => eq = "y=\\left\\lceil 30 \\times \\frac{2x^{2}-x+1}{4x^{2}-x+1} \\right\\rceil \\times 10",
        7 => eq = "y=\\left\\lceil 100 \\times \\frac{2x^{2}-x+1}{4x^{2}-x+1} \\right\\rceil \\times 10",
        8 => eq = "y=\\left\\lceil 350 \\times \\ln \\left( e-x \\right) \\right\\rceil \\times 10",
        9 => eq = "y=\\left\\lceil 500 \\times \\ln \\left( e-x \\right) \\right\\rceil \\times 10",
        10=> eq = "y=\\left\\lceil 400 \\times \\left( (\\frac{1}{2}+\\frac{x}{4} )\\sqrt{1-x}+\\frac{1}{2} \\right) \\right\\rceil \\times 10",
        11=> eq = "y=\\left\\lceil 100 \\times \\left( (\\frac{1}{2}+\\frac{x}{4} )\\sqrt{1-x}+\\frac{1}{2} \\right) \\right\\rceil \\times 10",
        12=> eq = "y=\\left\\lceil 40 \\times \\left( (\\frac{1}{2}+\\frac{x}{4} )\\sqrt{1-x}+\\frac{1}{2} \\right) \\right\\rceil \\times 10",
        13=> eq = "y=\\left\\lceil 150 \\times \\frac{4x+1}{4x^{2}+2x+1}\\right\\rceil \\times 10",
        14=> eq = "y=\\left\\lceil 200 \\times \\frac{4x+1}{4x^{2}+2x+1}\\right\\rceil \\times 10",
        15=> eq = "y=\\left\\lceil 20 \\times (-8x^{5}+32x^{4}-42x^{3}+22x^{2}-\\frac{9}{2}x+1) \\right\\rceil \\times 10",
        16=> eq = "y=\\left\\lceil 50 \\times (-8x^{5}+32x^{4}-42x^{3}+22x^{2}-\\frac{9}{2}x+1) \\right\\rceil \\times 10",
        _ => eq = "\\mathrm{Error!}",
    }
    return eq.to_string();
}

fn sin_expA(time:f64) -> f64{
    (3.0 * (10.0 * PI * time).sin() + 1.0).ceil() * 10.0
}

fn sin_expB(time:f64) -> f64{
    (2.0 * (10.0 * PI * time).sin() + 4.0).ceil() * 10.0
}

fn frac_expA(time:f64) -> f64{
    (time + 1.0)/(time*time + time + 1.0)
}

fn frac_expB(time:f64) -> f64{
    (2.0*time*time - time + 1.0)/(4.0*time*time - time + 1.0)
}

fn frac_expC(time:f64) -> f64{
    (4.0*time + 1.0)/(4.0*time*time + 2.0*time + 1.0)
}

fn ln_exp(time:f64) -> f64{
    (E - time).ln()
}

fn sqrt_exp(time:f64) -> f64{
    (1.0/2.0 + time/4.0) * (1.0 - time).sqrt() + 1.0/2.0
}

fn whole_exp(time:f64) -> f64{
    -8.0*time.powi(5) + 32.0*time.powi(4) - 42.0*time.powi(3) + 22.0*time.powi(2) - (9.0/2.0)*time + 1.0
}

fn calc(time:f64, id:i64) -> f64{
    let value: i64;
    match id {
        1 => {
            sin_expA(time)
        },
        2 => {
            sin_expB(time)
        },
        3 => {
            (200.0 * frac_expA(time)).ceil() * 10.0
        },
        4 => {
            (200.0 * frac_expA(time)).ceil() * 10.0
        },
        5 => {
            (50.0 * frac_expB(time)).ceil() * 10.0
        },
        6 => {
            (30.0 * frac_expB(time)).ceil() * 10.0
        },
        7 => {
            (100.0 * frac_expB(time)).ceil() * 10.0
        },
        8 => {
            (350.0 * ln_exp(time)).ceil() * 10.0
        },
        9 => {
            (500.0 * ln_exp(time)).ceil() * 10.0
        },
        10 => {
            (400.0 * sqrt_exp(time)).ceil() * 10.0
        },
        11 => {
            (100.0 * sqrt_exp(time)).ceil() * 10.0
        },
        12 => {
            (40.0 * sqrt_exp(time)).ceil() * 10.0
        },
        13 => {
            (150.0 * frac_expC(time)).ceil() * 10.0
        },
        14 => {
            (200.0 * frac_expC(time)).ceil() * 10.0
        },
        15 => {
            (20.0 * whole_exp(time)).ceil() * 10.0
        },
        16 => {
            (50.0 * whole_exp(time)).ceil() * 10.0
        },
        _ => {
             99999999.0
        },
    }
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
