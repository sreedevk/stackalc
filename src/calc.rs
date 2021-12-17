#[allow(dead_code)]

#[derive(Debug)]
pub struct Calc {
    pub frames: Vec<f64>
}

pub enum OptIdent {
    Add,
    Sub,
    Mul,
    Div
}

impl Calc {
    pub fn init() -> Calc {
        Calc {
            frames: Vec::with_capacity(64)
        }
    }

    fn circuit(iden: OptIdent) -> Box<dyn Fn(f64, f64) -> f64> {
        match iden {
            OptIdent::Add => Box::new(|x, y| x + y),
            OptIdent::Sub => Box::new(|x, y| x - y),
            OptIdent::Mul => Box::new(|x, y| x * y),
            OptIdent::Div => Box::new(|x, y| x / y)
        }
    }

    pub fn push(&mut self, framevar: f64) {
        self.frames.push(framevar);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.frames.pop()
    }

    pub fn reduce(&mut self, opt: OptIdent) {
        match self.pop() {
            Some(n1) => {
                match self.pop() {
                    Some(n2) => { self.push((Calc::circuit(opt))(n1, n2)) },
                    None => ()
                }
            },
            None => ()
        }
    }
}
