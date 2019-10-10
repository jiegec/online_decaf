use crate::common::{Msg, Request, Response};
use driver;
use yew::worker::*;

pub struct Runner {
    link: AgentLink<Runner>,
}

impl Agent for Runner {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;
    fn create(link: AgentLink<Self>) -> Self {
        Runner { link }
    }
    fn update(&mut self, _msg: Self::Message) {}
    fn handle(&mut self, msg: Self::Input, id: HandlerId) {
        let pa = match msg.pa.as_ref() {
            "PA1-A" => driver::Pa::Pa1a,
            "PA1-B" => driver::Pa::Pa1b,
            "PA2" => driver::Pa::Pa2,
            "PA3" => driver::Pa::Pa3,
            "PA4" => driver::Pa::Pa4,
            "PA5-MIPS" => driver::Pa::Pa5,
            "PA5-WAST" => driver::Pa::Pa5Wast,
            _ => unreachable!(),
        };

        let alloc = driver::Alloc::default();
        let result = driver::compile(&msg.code, &alloc, pa.to_cfg());
        let valid = result.is_ok();
        let (output, status) = match result {
            Ok(output) => (output, format!("Compliation for pa {} success", msg.pa)),
            Err(err) => (
                String::new(),
                format!("Compliation for pa {} failed with {:?}", msg.pa, err),
            ),
        };
        self.link.response(id, Response { output, status, valid });
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}
