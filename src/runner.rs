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
            _ => unreachable!(),
        };
        let (output, status) =
            match driver::compile(&msg.code, &driver::Alloc::default(), pa.to_cfg()) {
                Ok(output) => (output, format!("Compliation for pa {} success", msg.pa)),
                Err(err) => (
                    String::new(),
                    format!("Compliation for pa {} failed with {:?}", msg.pa, err),
                ),
            };
        self.link.response(id, Response { output, status });
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}
