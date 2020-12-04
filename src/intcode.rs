use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
pub enum IntState {
    Ready,
    Waiting,
    Finished,
}

#[derive(PartialEq, Eq)]
pub enum ParamMode {
    Immediate,
    Position,
}

pub struct Computer {
    pub memory: Vec<i64>,
    pub ptr: usize,
    pub state: IntState,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    pub verbose: u32,
}


impl Computer {
    pub fn new(code: &str) -> Computer {
        let memory: Vec<i64> = code.split(',').map(|x| x.trim()).filter_map(|x| x.parse::<i64>().ok()).collect();
        Computer {
            memory,
            ptr: 0,
            state: IntState::Ready,
            input: VecDeque::new(),
            output: VecDeque::new(),
            verbose: 0,
        }
    }

    pub fn run(&mut self) -> &IntState {
        if self.verbose > 1 {
            println!("[{}]{:?}", self.ptr, self.memory.iter().map(|x| x.to_string()))
        }
        match self.op() {
            1 => self.run_add(),
            2 => self.run_mult(),
            3 => self.read(),
            4 => self.write(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equal(),
            99 => self.stop(),
            _ => { println!("[{}]BAD OP: {}", self.ptr, self.op()); self.stop();},
        };

        &self.state
    }

    pub fn peek(&self, ix: usize) -> i64 {
        self.memory[ix]
    }

    pub fn poke(&mut self, ix: usize, val: i64) {
        self.memory[ix] = val
    }

    pub fn stop(&mut self) {
        if self.verbose > 1 {
            println!("[{}]{:?}", self.ptr, self.memory.iter().map(|x| x.to_string()));
        }
        if self.verbose > 0 {
            println!("[STOP]");
        }
        self.state = IntState::Finished;
    }

    fn debug_at(&self, i: usize) -> String {
        match self.memory.get(self.ptr + i) {
            Some(v) => v.to_string(),
            None => String::from(" "),
        }
    }

    pub fn op(&self) -> i64 {
        if self.verbose > 0 {
            println!("[{}] ..,{},{},{},{},{},...", self.ptr,
                     self.debug_at(0),
                     self.debug_at(1),
                     self.debug_at(2),
                     self.debug_at(3),
                     self.debug_at(4))
        }
        self.memory[self.ptr] % 100
    }

    pub fn mode_for(&self, jump: usize) -> ParamMode {
        let op = self.memory[self.ptr];
        let modes: Vec<char> = op.to_string().chars().rev().collect();
        match modes.get(jump + 1) {
            Some('1') => ParamMode::Immediate,
            _ => ParamMode::Position,
        }
    }


    pub fn in_bound(&mut self, jump: usize) -> bool {
            match self.mode_for(jump) {
                ParamMode::Immediate => true,
                ParamMode::Position => (self.memory[self.ptr + jump] as usize) < self.memory.len(),
            }
    }

    pub fn param(&self, jump: usize) -> i64 {
        match self.mode_for(jump) {
            ParamMode::Immediate => self.ahead(jump),
            ParamMode::Position => self.indr_ahead(jump),
        }
    }

    pub fn ahead(&self, jump: usize) -> i64 {
        self.memory[self.ptr + jump]
    }

    pub fn indr_ahead(&self, jump: usize) -> i64 {
        let indirection = self.memory[self.ptr + jump];
        match self.memory.get(indirection as usize) {
            Some(v) => *v,
            None =>  { panic!("Out of bounds!"); },
        }
    }

    pub fn read(&mut self) {
        if let Some(val) = self.input.pop_front() {
            let target = self.ahead(1);
            if self.verbose > 0 {
                println!("[{}] {} << {}", self.ptr, target, val);
            }
            self.memory[target as usize] = val;
            self.state = IntState::Ready;
            self.ptr += 2;
        } else {
            self.state = IntState::Waiting;
            if self.verbose > 0 {
                println!("[{}] WAITING...", self.ptr);
            }
        }
    }

    pub fn write(&mut self) {
        let val = self.param(1);
        self.output.push_back(val);
        self.state = IntState::Ready;
        if self.verbose > 0 {
            println!("[{}] {} >>", self.ptr, val);
        }
        self.ptr += 2;
    }

    pub fn run_add(&mut self) {
        let res = self.param(1) + self.param(2); 
        let target = self.ahead(3);
        if self.verbose > 0 {
            println!("[{}] {} + {} -> {}", self.ptr, self.param(1), self.param(2), self.ahead(3));
        }
        self.memory[target as usize] = res;
        self.state = IntState::Ready;
        self.ptr += 4;
    }

    pub fn run_mult(&mut self) {
        let res = self.param(1) * self.param(2); 
        let target = self.ahead(3);
        if self.verbose > 0 {
            println!("[{}] {} * {} -> {}", self.ptr, self.param(1), self.param(2), self.ahead(3));
        }
        self.memory[target as usize] = res;
        self.state = IntState::Ready;
        self.ptr += 4;
    }

    pub fn jump_if_true(&mut self) {
        if self.param(1) != 0 {
            if self.verbose > 0 {
                println!("[{}] TRUE({}) ===> {}", self.ptr, self.param(1), self.param(2) as usize);
            }
            self.ptr = self.param(2) as usize;
        } else {
            if self.verbose > 0 {
                println!("[{}] XX TRUE({}) ===> {}", self.ptr, self.param(1), self.param(2));
            }
            self.ptr += 3;
        }
    }

    pub fn jump_if_false(&mut self) {
        if self.param(1) == 0 {
            if self.verbose > 0 {
                println!("[{}] FALSE({}) ===> {}", self.ptr, self.param(1), self.param(2));
            }
            self.ptr = self.param(2) as usize;
        } else {
            if self.verbose > 0 {
                println!("[{}] XX FALSE({}) ===> {}", self.ptr, self.param(1), self.param(2));
            }
            self.ptr += 3;
        }
    }

    pub fn less_than(&mut self) {
        let target = self.ahead(3);
        if self.in_bound(1) && self.in_bound(2) && self.param(1) < self.param(2) {
            if self.verbose != 0 {
                println!("[{}] {} < {} 1 -> {}", self.ptr, self.param(1), self.param(2), target);
            }
            self.memory[target as usize] = 1;
        } else {
            if self.verbose != 0 {
                println!("[{}] {} < {} 0  -> {}", self.ptr, self.ahead(1), self.ahead(2), target);
            }
            self.memory[target as usize] = 0;
        }
        self.ptr += 4;
    }

    pub fn equal(&mut self) {
        let target = self.ahead(3);

        if self.in_bound(1) && self.in_bound(2) &&  self.param(1) == self.param(2) {
            if self.verbose > 0 {
                println!("[{}] {} == {} 1 -> {}", self.ptr, self.param(1), self.param(2), target);
            }
            self.memory[target as usize] = 1;
        } else {
            if self.verbose > 0 {
                println!("[{}] {} == {} 0 -> {}", self.ptr, self.ahead(1), self.ahead(2), target);
            }
            self.memory[target as usize] = 0;
        }
        self.ptr += 4;
    }
}
