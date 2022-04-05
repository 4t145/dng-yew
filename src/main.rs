
mod components;
mod ws;
mod consts;
mod utils;
mod locals;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::start_app::<components::App>();
}
