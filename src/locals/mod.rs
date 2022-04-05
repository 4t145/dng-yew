mod zh;
mod en;
pub use {
    zh::ZH,
    en::EN,
};

macro_rules! locals {
    ($struct_id:ident;$($id:ident, )*) => {
        pub struct $struct_id<'a> {
            $(pub $id:&'a str,)*
        }
    };
}

locals!{
    Locals;


    me,

    lack_of_parameter,
    unsupported,
    check_your_input,

    game_start,
    game_end,
    turn_start,
    turn_end,
    mark_start,
    mark_end,

    mark,
    vote_up,
    vote_down,
    vote_neutral,
    key_word,

    input_placeholder,

    help,
}