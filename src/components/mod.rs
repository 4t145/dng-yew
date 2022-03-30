pub mod console;
pub mod drawpad;
mod players;
mod colorpicker;
use std::rc::Rc;

use yew::{Context, Component, Html, html, classes};
use yew_agent::{/* Dispatched, Dispatcher,  */Bridge, Bridged, Dispatcher, Dispatched};
use console::{
    Console,
    // agent::{ConsoleAgent},
    // item::{ItemKind}
};
use drawpad::{Drawpad, DrawpadReq, Tool};
use players::{Players};
use colorpicker::Colorpicker;
use crate::{ws::{PlayerState, WsRespAgent, Resp}, info};

use self::drawpad::DrawpadAgent;


pub struct App {
    drawpad: Dispatcher<DrawpadAgent>,
    player_states: [Option<PlayerState>; 8],
    drawer: u8,
    count_down: u8,

    resp_bus: Option<Box<dyn Bridge<WsRespAgent>>>,
}
pub enum AppMsg {
    Ws(Rc<Resp>),
    ClearButton,
    PencilButton,
    EraserButton,
}
impl Component for App {
    type Message = AppMsg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            drawpad: DrawpadAgent::dispatcher(),
            player_states: Default::default(),
            drawer: 0xff,
            count_down: 00,
            resp_bus: None
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Ws(resp) => {
                match resp.as_ref() {
                    Resp::PlayerStates(states) => {
                        for s in states {
                            self.player_states[s.idx as usize] = Some(s.clone());
                        }
                        true
                    },
                    Resp::CountDown(cd) => {
                        self.count_down = *cd;
                        true
                    },
                    Resp::TurnStart(drawer) => {
                        self.drawer = *drawer;
                        true
                    },
                    _ => {
                        false
                    }
                }
            },
            AppMsg::ClearButton => {self.drawpad.send(DrawpadReq::Clear);false},
            AppMsg::PencilButton => {self.drawpad.send(DrawpadReq::SetTool(Tool::Pencil));false},
            AppMsg::EraserButton => {self.drawpad.send(DrawpadReq::SetTool(Tool::Eraser));false},
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count_down = format!("{:02}", self.count_down);
        let clear = ctx.link().callback(|_| {AppMsg::ClearButton});
        let pencil = ctx.link().callback(|_| {AppMsg::PencilButton});
        let eraser = ctx.link().callback(|_| {AppMsg::EraserButton});

        html! {
            <div>
                <div class={classes!("app")}>
                    <Players states = {self.player_states.clone()} pin = {self.drawer}/>
                    <Console/>
                    <Drawpad/>
                    <div class={classes!("toolbar")}>
                        <Colorpicker/>
                        <div class="countdown"> {count_down}</div>
                        <div id="clear-button" onclick={clear}> </div>
                        <div id="pencil-button" onclick={pencil}> </div>
                        <div id="eraser-button" onclick={eraser}> </div>
                    </div>
                </div>
                <footer>
                    <p>
                        {"本项目目前"}
                        <svg aria-hidden="true" height="14" viewBox="0 0 16 16" version="1.1" width="14" data-view-component="true" class="octicon octicon-mark-github">
                            <path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                        </svg>
                        <a href="https://github.com/4t145/dng-server" target="_blank">{"github仓库"}</a>
                    </p>
                    <p>{"作者邮箱 📧 u4t145@163.com"}</p>
                </footer>
            </div>

        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            info!("{}", crate::consts::WELCOME_CONSOLE);
            self.resp_bus = Some(WsRespAgent::bridge(ctx.link().callback(AppMsg::Ws)));
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}