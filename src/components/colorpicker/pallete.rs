
use std::rc::Rc;

use wasm_bindgen::{JsCast};
use yew::{Context, Component, Html, html, classes, NodeRef, Properties, Callback};
use web_sys::{HtmlInputElement, InputEvent, PointerEvent};


#[derive(Properties, PartialEq)]
pub struct PalleteProps {
    pub set_color: Rc<Callback<String>>,
    pub init_color: String,
}

pub struct Pallete {
    color: String,
    input_ref: NodeRef,
}

pub enum PalleteMsg {
    ChangeColor(String),
    Choose,
    Pick
}
impl Component for Pallete {
    type Message = PalleteMsg;
    type Properties = PalleteProps;


    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            color: "#ff33cc".to_string(),
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PalleteMsg::ChangeColor(color) => {
                self.color = color;
                true
            },
            PalleteMsg::Choose => {
                ctx.props().set_color.emit(self.color.clone());
                false
            }
            PalleteMsg::Pick => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    input.click();
                }
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().batch_callback(|evt:InputEvent|{
            if let Some(target) = evt.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    return vec![PalleteMsg::ChangeColor(input.value()), PalleteMsg::Choose];
                }
            }
            vec!()
        });

        let onpointerdown = ctx.link().batch_callback(|evt:PointerEvent|{
            if evt.buttons() == 2 {
                Some(PalleteMsg::Pick)
            } else if evt.buttons() == 1 {
                Some(PalleteMsg::Choose)
            } else {
                None
            }
        });

        let style = format!("background-color: {};", self.color);
        html! {
            <div class={classes!("pallete")}>
                <input type = {"color"} value = {self.color.clone()} ref={self.input_ref.clone()} {oninput} />
                <div class="pallete-block" style = {style}></div>
                <div class="pallete-border" {onpointerdown}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(PalleteMsg::ChangeColor(ctx.props().init_color.clone()))
        }
    }
}