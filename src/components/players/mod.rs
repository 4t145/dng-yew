
use yew::{function_component, Html, html, Properties, classes};

use crate::ws::PlayerState;


#[derive(Debug, Properties, PartialEq)]
struct PlayerItemProps {
    state: Option<PlayerState>,
    is_pinned: bool,
}
#[function_component(PlayerItem)]
fn player_item(props: &PlayerItemProps) -> Html {

    // online
    if let Some(state) = &props.state {
        let idx_id = format!("player-avatar-{}", state.idx);
        let ready_class = if state.ready{"player-avatar-ready"} else {"player-avatar-unready"};
        let show_pin = if props.is_pinned{"opacity: 1;"} else {"opacity: 0;"};
        return html!(
            <div class = "player-item">
                <div class = "player-pointer" style={show_pin}></div>
                <div class = {classes!(
                    "player-avatar-online", 
                    ready_class
                )} id = {idx_id}></div>
                <div class = "player-name"> {state.name.clone()} </div>
                <div class = "score">
                    <span class = "score-voteup">       {state.score[2]} </span>{" | "}
                    <span class = "score-voteneutral">  {state.score[1]} </span>{" | "}
                    <span class = "score-votedown">     {state.score[0]} </span>
                </div>
            </div>
        );
    // offline
    } else {
        return html!(
            <div class = "player-item">
                <div class = "player-pointer" style="opacity: 0;"></div>
                <div class = "player-avatar-offline"></div>
                <div class = "player-name"></div>
                <div class = "score"></div>
            </div>
        );
    }
}   

#[derive(Debug, PartialEq, Properties)]
pub struct PlayersProps {
    pub states: [Option<PlayerState>; 8],
    pub pin: u8
}

#[function_component(Players)]
pub fn players(props: &PlayersProps) -> Html { 
    let player_items:Vec<Html> = props.states.iter().zip(0..8).map(
        |(state, idx)| {
            html!(<PlayerItem state = {state.clone()} is_pinned={idx==props.pin}/>)
        }
    ).collect();

    html!(
        <div class = {classes!("players")}>
            {player_items}
        </div>
    )
}