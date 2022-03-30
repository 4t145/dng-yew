mod pallete;

use std::rc::Rc;

use wasm_bindgen::{JsCast};
use yew::{Context, Component, Html, html, NodeRef};
use web_sys::{HtmlInputElement, InputEvent, PointerEvent};
use yew_agent::{Dispatched, Dispatcher};

use crate::{utils::parse_color};

use super::drawpad::{DrawpadAgent, DrawpadReq};
use pallete::Pallete;

pub struct Colorpicker {
    color: String,
    input_ref: NodeRef,
    drawpad_agent: Dispatcher<DrawpadAgent>,
}
pub enum ColorPickerMsg {
    ChangeColor(String),
    OpenColorPicker
}
impl Component for Colorpicker {
    type Message = ColorPickerMsg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            color: "#333333".to_string(),
            input_ref: NodeRef::default(),
            drawpad_agent: DrawpadAgent::dispatcher(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ColorPickerMsg::ChangeColor(color) => {
                if let Some(c) = parse_color(color.as_str()) {
                    self.drawpad_agent.send(DrawpadReq::SetColor(c))
                }
                self.color = color;
                true
            },
            ColorPickerMsg::OpenColorPicker => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    input.click();
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().batch_callback(|evt:InputEvent|{
            if let Some(target) = evt.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    return Some(ColorPickerMsg::ChangeColor(input.value()));
                }
            }
            None
        });

        let onpointerdown = ctx.link().callback(|_evt:PointerEvent|{
            ColorPickerMsg::OpenColorPicker
        });

        let set_color = 
        Rc::new(ctx.link().callback(|c:String|{ColorPickerMsg::ChangeColor(c.clone())}));
        let style = format!("background-color: {};", self.color);
        html! {
            <div class="colorpicker">
                <div class="mainpicker" {onpointerdown}> 
                    <input type = {"color"} value = {self.color.clone()} ref={self.input_ref.clone()} {oninput} />
                    <div id="colorpicker-block" style = {style}></div>
                    <div id="colorpicker-border"/>
                </div>
                <div class="pallete-table">
                    <table cellSpacing="0" cellPadding="0">
                        <tbody>
                            <tr> 
                                <td><Pallete init_color={"#dd6666".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#774488".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#9badb7".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#df7126".to_string()} set_color={set_color.clone()}/></td>
                            </tr>
                            <tr> 
                                <td><Pallete init_color={"#5fcde4".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#37946e".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#222222".to_string()} set_color={set_color.clone()}/></td>
                                <td><Pallete init_color={"#6abe30".to_string()} set_color={set_color.clone()}/></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {

    }
}