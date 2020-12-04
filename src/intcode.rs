#[derive(PartialEq, Eq)]
pub enum  IntState {
    Ready,
    Waiting,
    Finished,
}

pub struct Computer {
    pub memory: Vec<i64>,
    pub ptr: i64,
    pub state: IntState,
}


impl Computer {
    pub fn new(code: &str) -> Computer {
        let memory = code.split(',').filter_map(|x| x.parse::<i64>().ok()).collect();
        Computer {
            memory,
            ptr: 0,
            state: IntState::Ready,
        }
    }

    pub fn run(&mut self) -> &IntState {
        match self.memory[self.ptr as usize] {
            1 => self.run_add(),
            2 => self.run_mult(),
            _ => self.stop(),
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
        self.state = IntState::Finished;
    }

    pub fn ahead(&self, jump: i64) -> i64 {
        self.memory[(self.ptr + jump) as usize]
    }

    pub fn indr_ahead(&self, jump: i64) -> i64 {
        let indirection = self.memory[(self.ptr + jump) as usize];
        self.memory[indirection as usize]
    }

    pub fn run_add(&mut self) {
        let res = self.indr_ahead(1) + self.indr_ahead(2); 
        let target = self.ahead(3);
        self.memory[target as usize] = res;
        self.ptr += 4;
    }

    pub fn run_mult(&mut self) {
        let res = self.indr_ahead(1) * self.indr_ahead(2); 
        let target = self.ahead(3);
        self.memory[target as usize] = res;
        self.ptr += 4;
    }
}
