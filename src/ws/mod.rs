
use yew_agent::{Dispatched};
use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use web_sys::{window, UrlSearchParams};
use bincode::{serialize, deserialize};


mod request;
mod response;
mod agent;

pub use request::PlayerRequest as Req;
pub use response::{PlayerResponse as Resp, PlayerState};
pub use agent::{WsRespAgent, WsReqAgent};

use crate::components::console::{agent::ConsoleAgent, item::ItemKind};


pub fn ws_service_init() -> Option<Sender<request::PlayerRequest>> {
    let mut console = ConsoleAgent::dispatcher();
    if let Some(window) = window() {
    if let Ok(url) = window.location().search() {
    if let Ok(urlsearch) = UrlSearchParams::new_with_str(url.as_str()) {
    if let Some(server) = urlsearch.get("server") {
        if let Ok(ws) = WebSocket::open(format!("ws://{}/login", server).as_str()) {
            let (mut ws_tx, mut ws_rx) = ws.split();

            let (req_tx, mut req_rx) = futures::channel::mpsc::channel::<request::PlayerRequest>(64);

            spawn_local(async move {
                while let Some(req) = req_rx.next().await {
                    if let Ok(bin) = serialize(&req) {
                        ws_tx.send(Message::Bytes(bin)).await.unwrap_or_default();
                    }
                }
            });

            spawn_local(async move {
                let mut dispatcher =  agent::WsRespAgent::dispatcher();
                while let Some(msg) = ws_rx.next().await {
                    match msg {
                        Ok(Message::Text(_)) => {}
                        Ok(Message::Bytes(bin)) => {
                            if let Ok(resp) = deserialize::<Resp>(&bin) {
                                dispatcher.send(resp)
                            }

                        }
                        Err(_e) => {

                        }
                    }
                }
            });
            console.send(ItemKind::GameState{msg: format!("已连接至服务器{}", server)});
            return Some(req_tx)
        } else {
            console.send(ItemKind::Warn{msg: format!("无法连接至服务器{}", server)});
            return None;
        }
    }}}}    
    console.send(ItemKind::Warn{msg: "url缺少参数server".to_string()});
    None
    
}
