pub enum MemorySegment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

impl MemorySegment {
    fn to_string(&self) -> String {
        match self {
            MemorySegment::Local => String::from("local"),
            MemorySegment::Argument => String::from("argument"),
            MemorySegment::This => String::from("this"),
            MemorySegment::That => String::from("that"),
            MemorySegment::Constant => String::from("constant"),
            MemorySegment::Static => String::from("static"),
            MemorySegment::Pointer => String::from("pointer"),
            MemorySegment::Temp => String::from("temp"),
        }
    }
}

pub enum MathInstr {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl MathInstr {
    fn to_string(&self) -> String {
        match self {
            MathInstr::Add => String::from("add"),
            MathInstr::Sub => String::from("sub"),
            MathInstr::Neg => String::from("neg"),
            MathInstr::Eq => String::from("eq"),
            MathInstr::Gt => String::from("gt"),
            MathInstr::Lt => String::from("lt"),
            MathInstr::And => String::from("and"),
            MathInstr::Or => String::from("or"),
            MathInstr::Not => String::from("not"),
        }
    }
}

pub struct VMWriter {
    pub result: Vec<String>,
}

impl VMWriter {
    pub fn new() -> Self {
        Self { result: vec![] }
    }

    pub fn write_push(&mut self, segment: MemorySegment, index: usize) {
        self.result
            .push(format!("push {} {}", segment.to_string(), index));
    }

    pub fn write_pop(&mut self, segment: MemorySegment, index: usize) {
        self.result
            .push(format!("pop {} {}", segment.to_string(), index));
    }

    pub fn write_arithmetic(&mut self, instr: MathInstr) {
        self.result.push(instr.to_string());
    }

    pub fn write_label(&mut self, label: String) {
        self.result.push(format!("label {}", label));
    }

    pub fn write_goto(&mut self, label: String) {
        self.result.push(format!("goto {}", label));
    }

    pub fn write_if_goto(&mut self, label: String) {
        self.result.push(format!("if-goto {}", label));
    }

    pub fn write_call(&mut self, name: String, num_args: usize) {
        self.result.push(format!("call {} {}", name, num_args));
    }

    pub fn write_function(&mut self, name: String, num_locals: usize) {
        self.result
            .push(format!("function {} {}", name, num_locals));
    }

    pub fn write_return(&mut self) {
        self.result.push(String::from("return"));
    }
}
