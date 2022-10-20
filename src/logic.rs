// and_gate
pub struct AndGate {
    c1: Box<dyn Logic>,
    c2: Box<dyn Logic>,
}

impl AndGate {
    pub fn create_from(c1: Box<dyn Logic>, c2: Box<dyn Logic>) -> AndGate {
        AndGate { c1, c2 }
    }
}

impl Logic for AndGate {
    fn solve(&self) -> bool {
        self.c1.solve() && self.c2.solve()
    }
}

// switch
pub struct Switch {
    on: bool,
}

impl Switch {
    pub fn create(on: bool) -> Switch {
        Switch { on }
    }

    pub fn toggle(&mut self){
        self.on = !self.on; 
    }

    pub fn set(&mut self, on:bool){
        self.on = on; 
    }
}

impl Logic for Switch {
    fn solve(&self) -> bool {
        self.on
    }
}

// lamp
pub struct Lamp {
    src: Box<dyn Logic>,
}

impl Lamp {
    pub fn create(src: Box<dyn Logic>) -> Lamp {
        Lamp { src }
    }

    pub fn is_glowing(&self) -> bool{
        self.src.solve()
    }
}

// interface: logic
pub trait Logic {
    fn solve(&self) -> bool;
}
