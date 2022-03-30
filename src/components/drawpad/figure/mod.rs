
use serde::{Serialize, Deserialize};
use crate::consts::*;

use super::frame::Frame;
mod draw;
#[macro_export] 
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {crate::components::drawpad::Color::new($r as u8,$g as u8,$b as u8)};
}

type Data = [[Color; DRAWPAD_H]; DRAWPAD_W];
#[inline]
fn clear(data:&mut Data) {
    data.iter_mut().for_each(|col|col.fill(Color::white()));
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {pub r:u8,pub g:u8,pub b:u8}
impl Color {
    pub const fn new(r:u8,g:u8,b:u8) -> Self{Self{
        r,g,b
    }}

    pub const fn white() -> Self {
        Self{r:0xff, g:0xff, b:0xff}
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::white()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Tool {
    Eraser,
    Pencil
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Instruction {
    PointerDown((u8, u8)),
    PointerMove((u8, u8)),
    PointerUp((u8, u8)),
    SetColor(Color),
    SetTool(Tool),
    Clear,
    Redo,
    Undo,
    
    Reset,
}

pub enum Operation {
    Pencil {
        path:Vec<(u8,u8)>,
        color: Color,
    },
    Eraser {
        path:Vec<(u8,u8)>,
        size: u8,
    },
    Clear,
}

impl Operation {
    fn render(&self, data:&mut Data) {
        match self {
            Operation::Pencil { path, color } => {
                if path.len() == 0 {unreachable!();}
                else if path.len() == 1 {draw::point(data, path[0], *color)}
                else {
                    path.windows(2).for_each(|segment|draw::line(data,segment[0], segment[1], *color))
                }
            },
            Operation::Eraser { path, size } => {
                if path.len() == 0 {unreachable!();}
                else if path.len() == 1 {draw::fill_square(data, path[0], *size, Color::white())}
                else {
                    path.windows(2).for_each(|segment|draw::line_with_width(data,segment[0], segment[1], *size, Color::white()))
                }
            },
            Operation::Clear => {clear(data)}
        }
    }
}

struct History {
    stack: Vec<Operation>,
    been_rendered: usize,
    should_render: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(64),
            been_rendered:0,
            should_render:0,
        }
    }

    pub fn redo(&mut self) {
        if self.should_render < self.stack.len() {
            self.should_render += 1;
        }
    }

    pub fn undo(&mut self) {
        if self.should_render > 0 {
            self.should_render -= 1;
        }
    }

    pub fn push(&mut self, op: Operation) {
        self.stack.truncate(self.should_render);
        self.stack.push(op);
        self.should_render += 1;
    }

    pub fn clear(&mut self) {
        self.should_render = 0;
        self.been_rendered = 0;
        self.stack.clear();
    }

    pub fn render(&mut self, data:&mut Data) {
        if self.been_rendered > self.should_render {
            clear(data);
            self.been_rendered = 0;
        }
        for idx in self.been_rendered .. self.should_render {
            let op = &self.stack[idx];
            op.render(data);
        }
        self.been_rendered = self.should_render;
    }
}

pub struct FigureLocal {
    data: Data,
    path: Option<Vec<(u8,u8)>>,
    history: History,
    color: Color,
    tool: Tool,
    size: u8,
}


impl FigureLocal {
    pub fn blank() -> Self {
        FigureLocal {
            data: [[rgb!(0xff,0xff,0xff); DRAWPAD_H]; DRAWPAD_W],
            history: History::new(),
            path: None,
            color: rgb!(0,0,0),
            tool: Tool::Pencil,
            size: 16
        }
    }

    pub fn render_frame(&mut self, f: &Frame) {
        for i in &f.instructions {
            self.excute(i)
        }
        self.render()
    }

    fn excute(&mut self, ins: &Instruction) {
        match ins {
            Instruction::PointerDown(coor) => {
                let mut path = Vec::with_capacity(8);
                path.push(*coor);
                self.path = Some(path);
            },
            Instruction::PointerMove(coor) => {
                self.path.as_mut().map(|p|p.push(*coor));
            },
            Instruction::PointerUp(coor) => {
                if let Some(mut path) = self.path.take() {
                    path.push(*coor);
                    let operation = match self.tool {
                        Tool::Eraser => Operation::Eraser { 
                            path, 
                            size: self.size,
                        },
                        Tool::Pencil => Operation::Pencil{
                            path, 
                            color: self.color.clone(), 
                        },
                    };
                    
                    self.history.push(operation);
                }
            },
            Instruction::SetColor(c) => self.color=*c ,
            Instruction::SetTool(t) => self.tool=*t,
            Instruction::Clear => {
                clear(&mut self.data);
                self.history.push(Operation::Clear);
            },
            Instruction::Redo => {
                self.history.redo();
            },
            Instruction::Undo => {
                self.history.undo();
            },
            Instruction::Reset => {
                self.history.clear();
                clear(&mut self.data);
            },
        }
    }
    
    fn render(&mut self) {
        // render history
        self.history.render(&mut self.data);

        // render now
        if let Some(path) = self.path.clone() {
            let operation = match self.tool {
                Tool::Eraser => Operation::Eraser { 
                    path, 
                    size: self.size,
                },
                Tool::Pencil => Operation::Pencil{
                    path, 
                    color: self.color.clone(), 
                },
            };
            operation.render(&mut self.data);
        }
    }

    #[inline]
    pub fn get_ref(&self) -> &Data {
        &self.data
    }
}

pub trait Figure {
    fn excute(&mut self, ins: &Instruction);

}