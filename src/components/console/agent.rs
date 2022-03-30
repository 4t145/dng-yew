use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::console::item::{ItemKind, ItemProps};

pub struct ConsoleAgent {
    link: AgentLink<Self>,
    console: Option<HandlerId>,
    // subscribers: HashSet<HandlerId>,
}

impl Agent for ConsoleAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ItemKind;
    type Output = ItemProps;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            console: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        if let Some(console) = self.console {
            self.link.respond(console, ItemProps{kind:msg.clone()});
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.console.replace(id);
    }

    fn disconnected(&mut self, _id: HandlerId) {
        self.console.take();
    }
}