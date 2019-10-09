use serde::*;
use yew::html::{ChangeData, InputData};
use yew::worker::Transferable;
pub enum Msg {
    InputCode(InputData),
    InputPa(ChangeData),
    RunnerResp(Response),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub code: String,
    pub pa: String,
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub output: String,
    pub status: String,
}

impl Transferable for Response {}
