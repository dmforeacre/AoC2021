#[derive(Default)]
pub struct OutputStruct {
    pub parseTime: u128,
    pub calcTime: u128,
    pub answer: String,
}

pub fn new() -> OutputStruct {
    OutputStruct{ 
        parseTime: Default::default(),
        calcTime: Default::default(),
        answer: Default::default(),
    }
}