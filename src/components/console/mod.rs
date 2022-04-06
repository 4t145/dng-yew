pub mod item;
pub mod agent;

use std::rc::Rc;

use yew_agent::{Bridge, Bridged, Dispatcher, Dispatched};
use yew::prelude::*;
use agent::ConsoleAgent;
use item::{ItemProps, Item};
use web_sys::{HtmlInputElement, HtmlElement};
use crate::{rgb, locals};
use crate::ws::{WsRespAgent, WsReqAgent, Resp, Req};

use crate::components::drawpad::{DrawpadAgent, DrawpadReq, StreamMode};

pub struct Console {

    local: locals::Locals<'static>, 

    items: Vec<ItemProps>,
    input_ref: NodeRef,
    output_ref: NodeRef,

    req_bus: Dispatcher<WsReqAgent>,
    
    drawpad_agent: Dispatcher<DrawpadAgent>,


    resp_bus: Option<Box<dyn Bridge<WsRespAgent>>>,
    agent: Option<Box<dyn Bridge<ConsoleAgent>>>,
}


pub enum ConsoleMsg {
    Ws(Rc<Resp>),
    AddItem(ItemProps),
    Submit
}

impl Component for Console {
    type Message = ConsoleMsg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            items: vec![ItemProps{kind:item::ItemKind::Help{local: &locals::ZH}}],

            local: locals::ZH,
            input_ref: NodeRef::default(),
            output_ref: NodeRef::default(),
            req_bus: WsReqAgent::dispatcher(),
            drawpad_agent: DrawpadAgent::dispatcher(),
            resp_bus:None,
            agent: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let local = &self.local;
        use ConsoleMsg::*;
        use item::ItemKind::*;
        match msg {
            AddItem(props) => {
                self.items.push(props);
                true
            },
            Submit => {
                if let Some(input_element) = self.input_ref.cast::<HtmlInputElement>() {
                    let input = input_element.value();
                    let input = input.trim();
                    if input.len() == 0 {
                        return false
                    }
                    if input.starts_with('/') {
                        self.items.push(ItemProps{kind:Command { task: "console".into(), msg: input.into() }});
                        let mut paras = input.split_ascii_whitespace();
                        match paras.next() {
                            Some("/name") => {
                                if let Some(name) = paras.next() {
                                    self.req_bus.send(Req::SetName { name: name.into() });
                                }
                            },
                            Some("/lang") => {
                                if let Some(lang) = paras.next() {
                                    use crate::locals::*;
                                    match lang.to_ascii_lowercase().as_str() {
                                        "zh" => self.local = ZH,
                                        "en" => self.local = EN,
                                        _ => {
                                            self.items.push(ItemProps{kind:Command { task: "lang".into(), msg: "no such localization, but you may help to translate".into() }});

                                        }
                                    }
                                }
                            },
                            Some("/sc") => {
                                if let Some(color) = paras.next() {
                                    if let Ok(color) = u32::from_str_radix(color, 16) {
                                        let r = ((color >> 16) & 0xff) as u8;
                                        let g = ((color >> 08) & 0xff) as u8;
                                        let b = ((color >> 00) & 0xff) as u8;
                                        self.items.push(ItemProps{
                                            kind: Command { task: "sc".into(), msg: "sc".into() } 
                                        });
                                        self.drawpad_agent.send(DrawpadReq::SetColor(rgb!(r,g,b)));
                                    }
                                }
                            },
                            Some("/mode") => {
                                match paras.next() {
                                    Some("watch") => {self.drawpad_agent.send(DrawpadReq::SetStreamMode(StreamMode::Receive))}
                                    Some("draw") => {self.drawpad_agent.send(DrawpadReq::SetStreamMode(StreamMode::Push))}
                                    Some("offline") => {self.drawpad_agent.send(DrawpadReq::SetStreamMode(StreamMode::Offline))}
                                    _ => {}
                                }
                            },
                            Some("/ready") => self.req_bus.send(Req::ImReady),
                            Some("/unready") => self.req_bus.send(Req::ImUnready),
                            Some("/lexicon") => match paras.next() {
                                Some(content) => {
                                    if let Ok(lex) = u32::from_str_radix(content, 16) {
                                        self.req_bus.send(Req::LexiconService(lex))
                                    } else {
                                        if content.contains("github.com") {
                                            self.req_bus.send(Req::LexiconGit(content.into()))
                                        } else {
                                            self.items.push(ItemProps{kind:Warn{ msg: local.check_your_input.into() }})
                                        }
                                    }
                                }
                                _ => {
                                    self.items.push(ItemProps{kind:Warn{ msg: local.unsupported.into() }})
                                }
                            },
                            Some("/help") => {
                                self.items.push(ItemProps{kind:Help{local}})
                            }
                            _ => {}
                        }
                    } else {
                        self.items.push(
                            ItemProps{
                                kind: Chat { sender: local.me.into() , msg: input.into() }
                            }
                        );
                        self.req_bus.send(Req::Chat { msg: input.into() });
                    }
                    input_element.set_value("");
                    true
                } else {
                    false
                }
            },
            Ws(resp) => {
                let local = &self.local;
                let kind =  match resp.as_ref() {
                    Resp::Chat { sender, msg } => Chat { sender: sender.clone(), msg: msg.clone()},
                    Resp::Notice { msg } => Notice { msg: msg.clone()},
                    Resp::Warn { msg } => Warn { msg: msg.clone()},
                    Resp::GameStart => GameState { msg: local.game_start.into() },
                    Resp::GameEnd => GameState { msg: local.game_end.into() },
                    Resp::Topic { topic_word } => GameState { msg: format!("{}{}", local.game_end, topic_word) },
                    Resp::TurnStart(_) => GameState { msg: local.turn_start.into() },
                    Resp::TurnEnd => GameState { msg: local.turn_end.into() },
                    Resp::MarkStart => GameState { msg: local.mark_start.into() },
                    Resp::Poll => Poll {local: local},
                    Resp::MarkEnd => GameState { msg: local.mark_end.into() },
                    _ => {
                        return false;
                    }
                };
                self.items.push(ItemProps{kind});
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let local = &self.local;
        
        let vnodes = self.items.iter().map(|p|{
            html!{<Item ..p.clone()/>}
        }).collect::<Html>();

        let onkeyup = ctx.link().batch_callback(|evt: KeyboardEvent| {
            if evt.key() == "Enter" {
                Some(Self::Message::Submit)
            } else {
                None
            }
        });

        html! {
            <div class="console">
                <div class="output" ref = {self.output_ref.clone()}>{vnodes}</div>
                <input type="text" placeholder={local.input_placeholder} ref = {self.input_ref.clone()} {onkeyup}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.agent = Some(ConsoleAgent::bridge(ctx.link().callback(ConsoleMsg::AddItem)));
            self.resp_bus =  Some(WsRespAgent::bridge(ctx.link().callback(ConsoleMsg::Ws)));
        }
        if let Some(output) = self.output_ref.cast::<HtmlElement>() {
            crate::info!("im here");
            output.scroll_to_with_x_and_y(0.0, output.scroll_height().into());
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}