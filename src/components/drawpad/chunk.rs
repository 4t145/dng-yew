use super::Frame;

pub struct ChunkUnloader {
    chunk: Vec<Frame>,
    frame_idx: usize,
}


impl ChunkUnloader {
    pub fn new() -> Self {
        Self {
            chunk: Vec::new(),
            frame_idx: 0
        }
    }

    pub fn unload(&mut self, new_chunk: Option<Vec<Frame>>) -> Frame {
        if let Some(new_chunk) = new_chunk {
            let mut instructions = Vec::new();
            for idx in self.frame_idx..self.chunk.len() {
                instructions.append(&mut self.chunk[idx].instructions);
            }
            self.chunk = new_chunk;
            self.frame_idx = 0;
            return Frame {instructions};
        } else {
            if self.frame_idx < self.chunk.len() {
                let mut frame = Frame {instructions:vec![]};
                std::mem::swap(&mut frame, &mut self.chunk[self.frame_idx]);
                self.frame_idx += 1;
                return frame
            } else {
                return Frame{instructions:vec![]};
            }
        }
    }
}

pub struct ChunkLoader<const SIZE:usize> {
    chunk: Option<Vec<Frame>>
}


impl<const SIZE:usize> ChunkLoader<SIZE> {
    pub fn new() -> Self {
        Self {
            chunk: None
        }
    }

    pub fn load(&mut self, f:Frame) -> Option<Vec<Frame>> {
        if let Some(ref mut chunk) = self.chunk {
            if chunk.len() == SIZE-1 {
                chunk.push(f);
                let res = self.chunk.take();
                return res;
            } else {
                chunk.push(f);
                None
            }
        } else {
            let mut chunk = Vec::with_capacity(SIZE);
            chunk.push(f);
            self.chunk = Some(chunk);
            None
        }  
    }
}
