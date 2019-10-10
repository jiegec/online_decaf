#![recursion_limit = "256"]

mod common;
pub mod runner;

use common::{Msg, Request};
use yew::agent::{Bridge, Bridged};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    console: ConsoleService,
    runner: Box<dyn Bridge<runner::Runner>>,
    code: String,
    output: String,
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
                false
            }
        };

        if should_run {
            self.status = format!("Running...");
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
                <h3> { "Status" } </h3>
                <pre>{ &self.status } </pre>
                <h3> { "Output" } </h3>
                <pre>{ &self.output } </pre>
            </div>
        }
    }
}
