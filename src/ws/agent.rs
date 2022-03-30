use futures::{channel::mpsc::Sender, SinkExt};

use yew_agent::{Agent, AgentLink, Context, HandlerId, Dispatched};

use std::{collections::HashSet, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use crate::{components::console::{item::{ItemKind}, agent::ConsoleAgent}, info};
use super::{Req, Resp, ws_service_init};
pub struct WsRespAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for WsRespAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Resp;
    type Output = Rc<Resp>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        let rc = Rc::new(msg);
        for sub in &self.subscribers {
            if sub.is_respondable() {
                self.link.respond(*sub, rc.clone());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        info!("{:?}",id);
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

pub struct WsReqAgent {
    _link: AgentLink<Self>,
    sender: Option<Sender<Req>>
}

impl Agent for WsReqAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Req;
    type Output = Result<(),()>;

    fn create(link: AgentLink<Self>) -> Self {

        let sender = ws_service_init();

        Self {
            _link: link,
            sender
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        if let Some(mut tx) = self.sender.clone() {
            spawn_local(async move {
                match tx.send(msg).await {
                    Ok(_) => {},
                    Err(e) => {
                        if e.is_disconnected() {
                            let mut console = ConsoleAgent::dispatcher();
                            console.send(ItemKind::Warn{msg: "连接已断开!".to_string()});
                            panic!()
                        }
                    },
                }
            })
        }
    }

    fn connected(&mut self, _id: HandlerId) {
        
    }

    fn disconnected(&mut self, _id: HandlerId) {

    }
}