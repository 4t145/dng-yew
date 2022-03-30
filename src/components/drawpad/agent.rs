use yew_agent::{Agent, AgentLink, Context, HandlerId};


use super::{figure::{Color, Tool}};
use super::StreamMode;
pub enum DrawpadReq {
    SetColor(Color),
    SetTool(Tool),
    Clear,
    SetStreamMode(StreamMode)
}

pub struct DrawpadAgent {
    link: AgentLink<Self>,
    drawpad: Option<HandlerId>,
}

impl Agent for DrawpadAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = DrawpadReq;
    type Output = DrawpadReq;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            drawpad: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {

        if let Some(drawpad) = self.drawpad {
            self.link.respond(drawpad, msg);
        } 
        
    }

    fn connected(&mut self, id: HandlerId) {
        self.drawpad.replace(id);
    }

    fn disconnected(&mut self, _id: HandlerId) {
        self.drawpad.take();
    }
}