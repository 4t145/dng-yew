use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew_agent::{Dispatched};

use crate::{ws::{WsReqAgent}};


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
    Poll,
    Help
}

#[derive(Debug, Clone, Properties, PartialEq, Deserialize, Serialize)]
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
                <div class={classes!("command")}>
                    {msg}
                </div>
            }
        },
        Chat{sender, msg} => {
            let msg = format!("{}: {}", sender, msg);
            html! {
                <div class={classes!("chat")}>
                    {msg}
                </div>
            }
        },
        Notice{msg} => {
            html! {
                <div class={classes!("notice")}>
                    {msg}
                </div>
            }
        },
        Warn{msg} => {
            html! {
                <div class={classes!("warn")}>
                    {msg}
                </div>
            }
        },
        GameState{msg} => {
            html! {
                <div class={classes!("game-state")}>
                    {msg}
                </div>
            }
        },
        
        Poll => {
            let oninput = Callback::from(move |evt:InputEvent|{
                let mut ws = WsReqAgent::dispatcher();
                let target = evt.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let score = i8::from_str_radix(target.value().as_str(), 10).unwrap();
                ws.send(crate::ws::Req::Mark{score});
            });
            html! {
                <div class="poll">
                    <form {oninput}>
                        <div class="poll-topic">{"打个分吧"}</div>
                        <div class="poll-option" >
                            <label id="option-1">
                                <input type="radio" name="vote" value="-1"/>
                                <div class="poll-option-label" style = "color:#ff7777;">{"又摆摆 --p"}</div>
                            </label>
                            <label id="option-2">
                                <input type="radio"  name="vote" value="0"/>
                                <div class="poll-option-label" style = "color:#ccffcc;">{"一般般 o_o"}</div>
                            </label>
                            <label id="option-3">
                                <input type="radio"  name="vote" value="1"/>
                                <div class="poll-option-label" style = "color:#55ccff;">{"优棒棒 ^^b"}</div>
                            </label>
                        </div>
                    </form>
                </div>
            }
        }
        Help => {
            html! {
                <div class="help">
                    {r#"help> 帮助"#}<br/>
                    {r#"# 快捷键"#}<br/>
                    {r#"crtl+z 撤销，crtl+z 重做，crtl+x 清屏"#}<br/>
                    {r#"在铅笔模式下， 右键按下使用橡皮擦"#}<br/>
                    {r#"E键可以在铅笔和橡皮擦之间切换, 主要是方便数位笔使用者"#}<br/>
                    {r#"右键调色板小方块可以修改调色板颜色"#}<br/>
                    {r#""#}<br/>
                    {r#"# 关键命令"#}<br/>
                    {r#"/ready 准备"#}<br/>
                    {r#"/unready 取消准备"#}<br/>
                    {r#"/name <名字> 设置名字"#}<br/>
                    {r#"/lexicon <文件网址> 从github上下载词库"#}<br/>
                    {r#"/lexicon <词库代码> 从词库服务器设置词库"#}<br/>
                    {r#""#}<br/>
                    {r#"更详细帮助参考：<todo>"#}<br/>
                    {r#"/help 调出此帮助"#}<br/>
                </div>
            }
        }
    }
}
