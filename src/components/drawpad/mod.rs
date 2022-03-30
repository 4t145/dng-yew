use std::{vec, rc::Rc};

use bincode::serialize;
use gloo_timers::callback::Interval;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d,ImageData};
use wasm_bindgen::{JsCast, JsValue, Clamped};
use yew::{Component, NodeRef, html, Context, classes};
use yew::events::{PointerEvent, KeyboardEvent, MouseEvent};
use yew_agent::{Bridge, Bridged, Dispatcher, Dispatched};
// use serde::{Serialize, Deserialize};

mod figure;
mod agent;
mod chunk;
mod frame;


use chunk::{ChunkLoader, ChunkUnloader};
pub use figure::Color;
pub use agent::{DrawpadReq, DrawpadAgent};


use crate::info;
use crate::{rgb, consts::*, ws::{WsReqAgent, WsRespAgent},/*  info */};
use figure::{FigureLocal, Instruction};
use crate::ws::{Req, Resp};
use frame::Frame;

pub use self::figure::Tool;

#[derive(Debug, PartialEq, Eq)]
pub enum StreamMode {
    Push,
    Receive,
    Offline,
}
pub struct Drawpad {
    color: Color,
    tool: Tool,

    buttons: u16,

    canvas_ref: NodeRef,
    figure: FigureLocal,
    frame: Frame,

    chunk_to_unload: Vec<Frame>,
    chunk_unloader: ChunkUnloader,
    chunk_loader: ChunkLoader<15>,
    stream_mode: StreamMode,

    req_bus: Dispatcher<WsReqAgent>,


    frame_handle: Option<Interval>,

    console_bus: Option<Box<dyn Bridge<DrawpadAgent>>>,
    resp_bus: Option<Box<dyn Bridge<WsRespAgent>>>,

}

pub enum PointerAction {
    Down,
    Up,
    Move,
}

pub enum DrawpadMsg {
    Pointer {
        coor: (i32, i32),
        action: PointerAction,
        buttons: u16
    },
    Frame,
    CtrlZ,
    CtrlY,
    CtrlX,
    HotKeyE,
    Req(DrawpadReq),
    Ws(Rc<Resp>)
}

impl Drawpad {
    fn get_context(&self) -> Option<CanvasRenderingContext2d> {
        if let Some(canvas) = self.canvas_ref.cast::<HtmlCanvasElement>() {
            if let Ok(Some(ctx)) = canvas.get_context("2d") {

                if let Ok(ctx) = ctx.dyn_into::<CanvasRenderingContext2d>() {

                    return Some(ctx)
                }
            } 
        } 
        None
    }

    #[inline]
    fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        self.canvas_ref.cast::<HtmlCanvasElement>()
    }

    fn sync_figure(&mut self)  {
        if let Some(ctx) = self.get_context() {
            let raw = self.figure.get_ref();
            let data = unsafe {
                use std::mem::MaybeUninit;
                let mut data:[u8; DRAWPAD_W*DRAWPAD_H*4] = MaybeUninit::uninit().assume_init();
                let mut idx = 0;
                for y in 0..DRAWPAD_H {
                    for x in 0..DRAWPAD_W {
                        let c = raw[x][y];
                        data[idx] = c.r;
                        data[idx+1] = c.g;
                        data[idx+2] = c.b;
                        data[idx+3] = 0xff;
                        idx += 4;
                    }
                }
                data
            };
            if let Ok(canvas_data) = ImageData::new_with_u8_clamped_array(Clamped(&data), DRAWPAD_W as u32) {
                ctx.put_image_data(&canvas_data, 0.0, 0.0).unwrap_or_default();
            }
        }
    }

    /// this method will replace the current frame with a empty frame
    fn take_frame(&mut self) -> Frame {
        let mut frame = Frame{instructions:vec![]};
        std::mem::swap(&mut frame, &mut self.frame);
        return frame
    }

    /// the only way to push new instruction
    /// 
    fn push_instruction(&mut self, ins: Instruction) {
        // local figure
        self.frame.push(ins);
    }
}

impl Component for Drawpad {
    type Message = DrawpadMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        Drawpad {
            color: rgb!(0,0,0),
            tool: Tool::Pencil,
            buttons:0,

            canvas_ref: NodeRef::default(),
            figure: FigureLocal::blank(),

            frame: Frame::new(),
            chunk_to_unload: Vec::new(),
            frame_handle: None,

            stream_mode: StreamMode::Offline,
            chunk_loader: ChunkLoader::new(),
            chunk_unloader: ChunkUnloader::new(),
            req_bus: WsReqAgent::dispatcher(),


            console_bus: None,
            resp_bus: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {

        use DrawpadMsg::*;
        let onpointerdown = ctx.link().callback(|evt: PointerEvent| Pointer {
            coor: (evt.client_x(),evt.client_y()),
            action: PointerAction::Down,
            buttons: evt.buttons()
        });
        
        let onpointermove = ctx.link().callback(|evt: PointerEvent| Pointer {
            coor: (evt.client_x(),evt.client_y()),
            action: PointerAction::Move,
            buttons: evt.buttons()
        });

        let onpointerup = ctx.link().callback(|evt: PointerEvent| Pointer {
            coor: (evt.client_x(),evt.client_y()),
            action: PointerAction::Up,
            buttons: evt.buttons()
        });

        let oncontextmenu = ctx.link().batch_callback(|evt: MouseEvent| {
            evt.prevent_default();
            None
        });

        let onkeyup = ctx.link().batch_callback(|evt: KeyboardEvent| {
            if evt.ctrl_key() {

                match evt.key().as_str()  {
                    "z" => Some(CtrlZ),
                    "y" => Some(CtrlY),
                    "x" => Some(CtrlX),
                    _ => None
                }
            } else if evt.key().as_str() == "e" {
                Some(HotKeyE)
            } else {
                None
            }
        });

        html! {
            <div class={classes!("drawpad")} tabindex="1" {onkeyup}>
                <canvas ref = {self.canvas_ref.clone()} {onpointerdown} {onpointermove} {onpointerup} {oncontextmenu}/>
            </div>
        }
    }


    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {

        if first_render {
            self.console_bus = Some(DrawpadAgent::bridge(ctx.link().callback(DrawpadMsg::Req)));
            self.resp_bus = Some(WsRespAgent::bridge(ctx.link().callback(DrawpadMsg::Ws)));
        }

        if let Some(canvas) = self.get_canvas() {
            canvas.set_height(128);
            canvas.set_width(128);
        }

        if let Some(ctx) = self.get_context() {
            ctx.set_fill_style(&JsValue::from_str("#ffffff"));
            ctx.fill_rect(0.0, 0.0, 128.0, 128.0);
        } else {
            info!("can not load context");
        }

        let handle = {
            let link = ctx.link().clone();
            gloo_timers::callback::Interval::new(17, move||{link.send_message(DrawpadMsg::Frame)})
        };

        self.frame_handle = Some(handle);
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DrawpadMsg::Pointer { coor, action, buttons } => {
                // only main key pressed
                let canvas = self.get_canvas();
                if let Some(canvas) = canvas {
                    let x0 = canvas.offset_left();
                    let y0 = canvas.offset_top();
                    let x = ((coor.0 - x0).clamp(0, 512)/4) as u8;
                    let y = ((coor.1 - y0).clamp(0, 512)/4) as u8;
                    match action {
                        PointerAction::Down => {
                            match (self.tool, buttons) {
                                // pencil 
                                (Tool::Pencil, 0b00001)|(Tool::Eraser, 0b00010)  => {
                                    self.push_instruction(Instruction::SetColor(self.color));
                                    self.push_instruction(Instruction::SetTool(Tool::Pencil));
                                    self.push_instruction(Instruction::PointerDown((x,y)));

                                },
                                // eraser
                                (Tool::Pencil, 0b00010)|(Tool::Eraser, 0b00001) => {
                                    self.push_instruction(Instruction::SetTool(Tool::Eraser));
                                    self.push_instruction(Instruction::PointerDown((x,y)));
                                },
                                _ => {

                                }
                            }
                            self.buttons = buttons;
                        },
                        PointerAction::Up => {
                            if (self.buttons == 0b00001)||(self.buttons == 0b00010) {
                                self.push_instruction(Instruction::PointerUp((x,y)));
                            }
                            self.buttons = 0b00000;
                        },
                        PointerAction::Move => {
                            if (buttons == 0b00001)||(buttons == 0b00010) {
                                self.push_instruction(Instruction::PointerMove((x,y)));
                            }
                        },
                    };
                    
                }
                false
            },
            DrawpadMsg::CtrlY => {self.push_instruction(Instruction::Redo);false},
            DrawpadMsg::CtrlZ => {self.push_instruction(Instruction::Undo);false},
            DrawpadMsg::CtrlX => {self.push_instruction(Instruction::Clear);false}
            DrawpadMsg::Frame => {
                // get frame
                let frame = if self.stream_mode == StreamMode::Receive {
                    // in receive mode, from unloader
                    if self.chunk_to_unload.is_empty() {
                        self.chunk_unloader.unload(None)
                    } else {
                        let mut chunk_to_unload = vec![];
                        std::mem::swap(&mut chunk_to_unload, &mut self.chunk_to_unload);
                        self.chunk_unloader.unload(Some(chunk_to_unload))
                    }
                } else {
                    // otherwise, from local frame
                    self.take_frame()
                };

                // render local
                self.figure.render_frame(&frame);

                // in push mode, load frame
                if self.stream_mode == StreamMode::Push {
                    // in case we get a whole chunk, send it
                    if let Some(chunk) = self.chunk_loader.load(frame) {
                        if let Ok(bin) = serialize(&chunk) {
                            self.req_bus.send(Req::Chunk {bin})
                        }
                    }
                }

                // write fugure.data to canvas
                self.sync_figure();
                false
            },
            DrawpadMsg::Req(req) => {
                match req {
                    DrawpadReq::SetTool(t) => {self.tool = t; false},
                    DrawpadReq::SetColor(c) => {self.color = c; false},
                    DrawpadReq::SetStreamMode(stream_mode) => {self.stream_mode = stream_mode; false},
                    DrawpadReq::Clear => {self.push_instruction(Instruction::Clear); false},
                }
            },
            DrawpadMsg::Ws(resp) => {
                match resp.as_ref() {
                    Resp::Chunk { bin } => {
                        if self.stream_mode == StreamMode::Receive {
                            use bincode::deserialize;
                            if let Ok(mut chunk) = deserialize::<Vec<Frame>>(bin) {
                                self.chunk_to_unload.append(&mut chunk);                                
                            }
                        }
                    }
                    Resp::Topic { topic_word:_ } => self.stream_mode = StreamMode::Push,
                    Resp::GameStart|Resp::TurnEnd => {
                        self.push_instruction(Instruction::Reset);
                        self.stream_mode = StreamMode::Receive
                    },
                    Resp::GameEnd => self.stream_mode = StreamMode::Offline,
                    Resp::MarkEnd => self.push_instruction(Instruction::Reset),
                    _ => {}
                }
                false
            },
            DrawpadMsg::HotKeyE => {
                match self.tool {
                    Tool::Eraser => self.tool = Tool::Pencil,
                    Tool::Pencil => self.tool = Tool::Eraser,
                }
                false
            },
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {

    }
}
