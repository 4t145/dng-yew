pub mod item;
pub mod agent;

use std::rc::Rc;

use yew_agent::{Bridge, Bridged, Dispatcher, Dispatched};
use yew::prelude::*;
use agent::ConsoleAgent;
use item::{ItemProps, Item};
use web_sys::{HtmlInputElement, HtmlElement};
use crate::{rgb};
use crate::ws::{WsRespAgent, WsReqAgent, Resp, Req};

use crate::components::drawpad::{DrawpadAgent, DrawpadReq, StreamMode};

pub struct Console {
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
            items: vec![ItemProps{kind:item::ItemKind::Help}],
            input_ref: NodeRef::default(),
            output_ref: NodeRef::default(),
            req_bus: WsReqAgent::dispatcher(),
            drawpad_agent: DrawpadAgent::dispatcher(),
            resp_bus:None,
            agent: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                        self.items.push(ItemProps{kind:Command { task: "console".to_string(), msg: input.to_string() }});
                        let mut paras = input.split_ascii_whitespace();
                        match paras.next() {
                            Some("/name") => {
                                if let Some(name) = paras.next() {
                                    self.req_bus.send(Req::SetName { name: name.to_string() });
                                }
                            },
                            Some("/sc") => {
                                if let Some(color) = paras.next() {
                                    if let Ok(color) = u32::from_str_radix(color, 16) {
                                        let r = ((color >> 16) & 0xff) as u8;
                                        let g = ((color >> 08) & 0xff) as u8;
                                        let b = ((color >> 00) & 0xff) as u8;
                                        self.items.push(ItemProps{
                                            kind: Command { task: "sc".to_string(), msg: "sc".to_string() } 
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
                                            self.req_bus.send(Req::LexiconGit(content.to_string()))
                                        } else {
                                            self.items.push(ItemProps{kind:Warn{ msg: "请检查你输入的命令是否正确".to_string() }})
                                        }
                                    }
                                }
                                _ => {
                                    self.items.push(ItemProps{kind:Warn{ msg: "不支持的命令".to_string() }})
                                }
                            },
                            Some("/help") => {
                                self.items.push(ItemProps{kind:Help})
                            }
                            _ => {}
                        }
                    } else {
                        self.items.push(
                            ItemProps{
                                kind: Chat { sender: "我".to_string(), msg: input.to_string() }
                            }
                        );
                        self.req_bus.send(Req::Chat { msg: input.to_string() });
                    }
                    input_element.set_value("");
                    true
                } else {
                    false
                }
            },
            Ws(resp) => {
               let kind =  match resp.as_ref() {
                    Resp::Chat { sender, msg } => Chat { sender: sender.clone(), msg: msg.clone()},
                    Resp::Notice { msg } => Notice { msg: msg.clone()},
                    Resp::Warn { msg } => Warn { msg: msg.clone()},
                    Resp::GameStart => GameState { msg: "本轮开始".to_string() },
                    Resp::GameEnd => GameState { msg: "本轮结束".to_string() },
                    Resp::Topic { topic_word } => GameState { msg: format!("关键词： {}", topic_word) },
                    Resp::TurnStart(_) => GameState { msg: "回合开始".to_string() },
                    Resp::TurnEnd => GameState { msg: "回合结束".to_string() },
                    Resp::MarkStart => GameState { msg: "评分开始".to_string() },
                    Resp::Poll => Poll,
                    Resp::MarkEnd => GameState { msg: "评分结束".to_string() },
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

        if let Some(output) = self.output_ref.cast::<HtmlElement>() {
            output.scroll_to_with_x_and_y(0.0, output.scroll_height().into());
        }
        html! {
            <div class="console">
                <div class="output" ref = {self.output_ref.clone()}>{vnodes}</div>
                <input type="text" placeholder="在此输入" ref = {self.input_ref.clone()} {onkeyup}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.agent = Some(ConsoleAgent::bridge(ctx.link().callback(ConsoleMsg::AddItem)));
            self.resp_bus =  Some(WsRespAgent::bridge(ctx.link().callback(ConsoleMsg::Ws)));
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}