
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};
use yew_agent::{Dispatched};

use crate::{ws::{WsReqAgent}};


#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    Command {
        task: String,
        msg: String
    },
    Chat {
        sender: String,
        msg: String,
    },
    Notice {
        msg: String
    },
    Warn {
        msg: String
    },
    GameState {
        msg: String
    },
    Poll {
        local: *const crate::locals::Locals<'static>
    },
    Help {
        local: *const crate::locals::Locals<'static>
    }
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct ItemProps {
    pub kind: ItemKind
} 

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    use ItemKind::*;
    match &props.kind {
        Command{task, msg} => {
            let msg = format!("{}> {}", task, msg);
            html! {
                <div class="command">
                    {msg}
                </div>
            }
        },
        Chat{sender, msg} => {
            let msg = format!("{}: {}", sender, msg);
            html! {
                <div class="chat">
                    {msg}
                </div>
            }
        },
        Notice{msg} => {
            html! {
                <div class="notice">
                    {msg}
                </div>
            }
        },
        Warn{msg} => {
            html! {
                <div class="warn">
                    {msg}
                </div>
            }
        },
        GameState{msg} => {
            html! {
                <div class="game-state">
                    {msg}
                </div>
            }
        },
        
        Poll{local} => {
            let local = unsafe {&*(*local)};
            let oninput = Callback::from(move |evt:InputEvent|{
                let mut ws = WsReqAgent::dispatcher();
                let target = evt.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let score = i8::from_str_radix(target.value().as_str(), 10).unwrap();
                ws.send(crate::ws::Req::Mark{score});
            });
            html! {
                <div class="poll">
                    <form {oninput}>
                        <div class="poll-topic">{local.mark}</div>
                        <div class="poll-option" >
                            <label id="option-1">
                                <input type="radio" name="vote" value="-1"/>
                                <div class="poll-option-label" style = "color:#ff7777;">{local.vote_down}</div>
                            </label>
                            <label id="option-2">
                                <input type="radio"  name="vote" value="0"/>
                                <div class="poll-option-label" style = "color:#ccffcc;">{local.vote_neutral}</div>
                            </label>
                            <label id="option-3">
                                <input type="radio"  name="vote" value="1"/>
                                <div class="poll-option-label" style = "color:#55ccff;">{local.vote_up}</div>
                            </label>
                        </div>
                    </form>
                </div>
            }
        }
        Help{local} => {
            let local = unsafe {&*(*local)};

            let lines:Vec<VNode> = local.help.lines().map(|line|html!(<span>{line}<br/></span>)).collect();
            html! {
                <div class="help">
                    {lines}
                </div>
            }
        }
    }
}
