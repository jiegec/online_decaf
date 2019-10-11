#![recursion_limit = "256"]

mod common;
pub mod runner;

use common::{Msg, Request};
use stdweb::{js, web::window, Value};
use yew::agent::{Bridge, Bridged};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    console: ConsoleService,
    runner: Box<dyn Bridge<runner::Runner>>,
    code: String,
    output: String,
    exec_output: Option<String>,
    status: String,
    pa: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|resp| Msg::RunnerResp(resp));
        Model {
            console: ConsoleService::new(),
            runner: runner::Runner::bridge(callback),
            code: String::new(),
            output: String::new(),
            exec_output: None,
            status: String::new(),
            pa: String::from("PA1-A"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let should_run = match msg {
            Msg::InputCode(input) => {
                self.code = input.value;
                true
            }
            Msg::InputPa(input) => {
                if let html::ChangeData::Select(select) = input {
                    self.pa = select.raw_value();
                }
                true
            }
            Msg::RunnerResp(resp) => {
                self.output = resp.output;
                self.status = resp.status;

                if self.pa == "PA5-WAST" && resp.valid {
                    let code = self.output.clone();

                    let output: Value = js! {
                        return window.execute(@{code});
                    };

                    self.exec_output = output.as_str().map(|s| s.to_owned());
                }

                false
            }
            Msg::RunTac => {
                let window = window();
                js! {
                    let code = btoa(@{&self.output});
                    @{window}.location.href = "/online_tac_vm/#" + code;
                };

                false
            }
        };

        if should_run {
            self.status = format!("Running...");
            self.exec_output = None;
            self.runner.send(Request {
                code: self.code.clone(),
                pa: self.pa.clone(),
            });
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let exec_block = self.exec_output.as_ref().map(|e| html! {
            <div>
                <h3> { "Execute output" } </h3>
                <h4> { "Hint: long-running or indefinite code might cause your browser to freeze" } </h4>
                <pre>{ e } </pre>
            </div>
        }).unwrap_or_else(|| html! { <div></div> });
        let tac_block = if &self.pa == "PA3" || &self.pa == "PA4" {
            html! {
                <div>
                    <button onclick=|_| Msg::RunTac> { "Run in Tac VM" } </button>
                </div>
            }
        } else {
            html! {
                <div></div>
            }
        };

        html! {
            <div>
                <h1> { "Online Decaf Compiler" }</h1>
                <h3> { "Input" } </h3>
                <form>
                    <label for="code"> { "Code" } </label>
                    <textarea style="height: 50vh" name="code" oninput=|content| Msg::InputCode(content)></textarea>
                    <label for="pa"> { "PA selection" } </label>
                    <select name="pa" id="pa" onchange=|content| Msg::InputPa(content)>
                        <option> { "PA1-A" } </option>
                        <option> { "PA1-B" } </option>
                        <option> { "PA2" } </option>
                        <option> { "PA3" } </option>
                        <option> { "PA4" } </option>
                        <option> { "PA5-MIPS" } </option>
                        <option> { "PA5-WAST" } </option>
                    </select>
                </form>

                { exec_block }
                { tac_block }

                <h3> { "Status" } </h3>
                <pre>{ &self.status } </pre>
                <h3> { "Output" } </h3>
                <pre>{ &self.output } </pre>
            </div>
        }
    }
}
