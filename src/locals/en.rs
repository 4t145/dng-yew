use super::Locals;

const HELP:&str = 
r#"
help> HELP

switch to Chinese: /lang zh

# Shortkey
crtl+z undo, crtl+z redo, crtl+x clear canvas
in pencil mode, press right key to use eraser
E can switch between pencil and eraser
right click little palette block to change it's color

# Command
/ready become ready
/unready become unready
/name <name> set your name
/lexicon <file-url> download lexicon from github
/lexicon <lexicon code> set lexicon of lexicon server

# Example Lexicon Repo
https://github.com/4t145/dng-lex/tree/main/en

more reference: https://github.com/4t145/dng-yew
/help to show this
"#;

pub const EN:Locals = Locals {
    me: "me",

    game_start: "Game Start",
    game_end: "Game Over",
    turn_start: "Turn Start",
    turn_end: "Turn Over",
    mark_start: "Mark Start",
    mark_end: "Mark Over",

    mark: "Give this a mark",
    vote_down:    "Sucks         --p",
    vote_neutral: "Okay          o_o",
    vote_up:      "Masterpiece   ^^b",

    key_word: "Keyword: ",

    unsupported: "Unsupported Command",
    check_your_input: "Please check your input",
    lack_of_parameter: "Lack of Parameter",

    input_placeholder: "input here",

    help: HELP
};