use std::str::FromStr;
use std::error::Error;

/// A struct to hold a set of instructions, which can be concatenated as a string to form an .asm
/// file.
struct Assembly {
    // Keeps track of the number of branch operations performed so far, so that we may assign the
    // TRUE(N) and CONT(N) labels correctly for the next one.
    branch_counter: usize,
    instructions: String,
}
impl Assembly {
    fn new() -> Self {
        Self {
            branch_counter: 0,
            instructions: String::new(),
        }
    }

    fn append(&mut self, branches: usize, appending: &str) {
        self.branch_counter += branches;
        self.instructions += appending;
    }
}

/// Adds some assembly commands to the buffer.
trait Write {
    fn write(&self, assembly: &mut Assembly);
}

/// Loads a single element from the stack such that it is available at "M."
fn read_one_input(assembly: &mut Assembly) {
    assembly.append(0, "\
        @SP\n\
        M=M-1\n\
        @SP\n\
        A=M\n\
    ");
}

/// Loads two elements from the stack: one available at "D," the other at "M."
fn read_two_inputs(assembly: &mut Assembly) {
    read_one_input(assembly);
    assembly.append(0, "\
        D=M\n\
        @SP\n\
        M=M-1\n\
        @SP\n\
        A=M\n\
    ");
}

/// After an operation has been performed, writes the output back onto the stack.
fn write_output(assembly: &mut Assembly) {
    assembly.append(0, "\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n\
    ");
}

/// Operation that takes one input and writes its result to the stack.
enum UnaryOpType {
    Neg,
    Not,
}
struct UnaryOp {
    typ: UnaryOpType,
}
impl UnaryOp {
    fn new(typ: UnaryOpType) -> Self {
        Self{typ}
    }
}
impl Write for UnaryOp {
    fn write(&self, assembly: &mut Assembly) {
        read_one_input(assembly);
        assembly.append(0, match self.typ {
            UnaryOpType::Neg => "D=-M\n",
            UnaryOpType::Not => "D=!M\n",
        });
        write_output(assembly);
    }
}

/// Operation that takes two inputs and writes the result to the stack.
enum BinaryOpType {
    Add,
    And,
    Or,
    Sub,
}
struct BinaryOp {
    typ: BinaryOpType,
}
impl BinaryOp {
    fn new(typ: BinaryOpType) -> Self {
        Self{typ}
    }
}
impl Write for BinaryOp {
    fn write(&self, assembly: &mut Assembly) {
        read_two_inputs(assembly);
        assembly.append(0,match self.typ {
            BinaryOpType::Add => "D=D+M\n",
            BinaryOpType::And => "D=D&M\n",
            BinaryOpType::Or => "D=D|M\n",
            BinaryOpType::Sub => "D=M-D\n",
        });
        write_output(assembly);
    }
}

/// Perform a comparison on two inputs.  Write -1 to the stack if true, 0 if false.
enum ComparisonType {
    Eq,
    Gt,
    Lt,
}
struct Comparison {
    typ: ComparisonType,
}
impl Comparison {
    fn new(typ: ComparisonType) -> Self {
        Self{typ}
    }
}
impl Write for Comparison {
    fn write(&self, assembly: &mut Assembly) {
        let tr = format!("TRUE{}", assembly.branch_counter);
        let co = format!("CONT{}", assembly.branch_counter);
        let jump = match self.typ {
            ComparisonType::Eq => "JEQ",
            ComparisonType::Gt => "JGT",
            ComparisonType::Lt => "JLT",
        };

        read_two_inputs(assembly);
        assembly.append(1, format!("\
            D=M-D\n\
            @{}\n\
            D;{}\n\
            D=0\n\
            @{}\n\
            0;JMP\n\
            ({})\n\
            D=-1\n\
            @{}\n\
            0;JMP\n\
            ({})\n\
        ", tr, jump, co, tr, co, co).as_str());
        write_output(assembly);
    }
}

/// A segment of memory.  Each of these segments has slightly unique properties when being pushed to
/// and popped from.
#[derive(Clone,Copy)]
enum Segment {
    Constant,
    Argument,
    Local,
    Pointer,
    Static,
    Temp,
    That,
    This,
}
impl Segment {
    fn into_variable(self) -> String {
        match self {
            Segment::Argument => "@ARG",
            Segment::Constant => "", // TODO: return error here
            Segment::Local    => "@LCL",
            Segment::Pointer  => "", // TODO: ditto
            Segment::Static   => "", // TODO: ditto
            Segment::Temp     => "", // TODO: ditto
            Segment::That     => "@THAT",
            Segment::This     => "@THIS",
        }.to_owned()
    }
}
impl FromStr for Segment {
    type Err = ();
    fn from_str(input: &str) -> Result<Segment, Self::Err> {
        match input {
            "argument" => Ok(Segment::Argument),
            "constant" => Ok(Segment::Constant),
            "local"    => Ok(Segment::Local),
            "pointer"  => Ok(Segment::Pointer),
            "static"   => Ok(Segment::Static),
            "temp"     => Ok(Segment::Temp),
            "that"     => Ok(Segment::That),
            "this"     => Ok(Segment::This),
            _          => Err(()),
        }
    }
}

/// Push commands for certain kinds of segments (specifically: pointer, static, temp) target
/// predetermined locations in memory.  This helper function generates assembly for them.
fn push(addr: String) -> String {
    format!("\
        @{}\n\
        D=M\n\
    ", addr.as_str())
}

struct Push {
    segment: Segment,
    value: u16,
}
impl Push {
    fn new(segment: Segment, value: u16) -> Self {
        Self{segment, value}
    }
}
impl Write for Push {
    fn write(&self, assembly: &mut Assembly) {
        match self.segment {
            Segment::Constant => assembly.append(0,format!("\
                @{}\n\
                D=A\n\
            ", self.value).as_str()),
            Segment::Pointer => assembly.append(0,push((self.value + 3).to_string()).as_str()),
            Segment::Static => assembly.append(0,push((self.value + 16).to_string()).as_str()),
            Segment::Temp => assembly.append(0,push(format!("R{}", (self.value + 5))).as_str()),
            _ => assembly.append(0,format!("\
                @{}\n\
                D=A\n\
                {}\n\
                A=M+D\n\
                D=M\n\
            ", self.value, self.segment.into_variable()).as_str()),
        }

        assembly.append(0, "\
            @SP\n\
            A=M\n\
            M=D\n\
            @SP\n\
            M=M+1\n\
        ");
    }
}

/// Pop commands for certain kinds of segments (specifically: pointer, static, temp) target
/// predetermined locations in memory.  This helper function generates assembly for them.
fn pop(addr: String) -> String {
    format!("\
        @SP\n\
        M=M-1\n\
        A=M\n\
        D=M\n\
        @{}\n\
        M=D\n\
    ", addr.as_str())
}

struct Pop {
    segment: Segment,
    value: u16,
}
impl Pop {
    fn new(segment: Segment, value: u16) -> Self {
        Self{segment, value}
    }
}
impl Write for Pop {
    fn write(&self, assembly: &mut Assembly) {
        match self.segment {
            Segment::Pointer => assembly.append(0,pop((self.value + 3).to_string()).as_str()),
            Segment::Static => assembly.append(0,pop((self.value + 16).to_string()).as_str()),
            Segment::Temp => assembly.append(0,pop(format!("R{}", (self.value + 5))).as_str()),
            _ => assembly.append(0,format!("\
                @{}\n\
                D=A\n\
                {}\n\
                D=M+D\n\
                @R13\n\
                M=D\n\
                @SP\n\
                M=M-1\n\
                A=M\n\
                D=M\n\
                @R13\n\
                A=M\n\
                M=D\n\
            ", self.value, self.segment.into_variable()).as_str()),
        }
    }
}

pub fn translate(text: String) -> Result<String,Box<dyn Error>> {
    let asm_str = text.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && s.chars().nth(0).unwrap() != '/')
        .fold(Ok(Assembly::new()), |acc, line| {
            match acc {
                Err(e) => Err(e),
                Ok(mut acc) => {
                    let asm = &mut acc;
                    println!("line: {}", line);
                    let words = line.split_whitespace().collect::<Vec<&str>>();
                    asm.append(0, format!("\n// {}:\n", line).as_str());
                    // TODO: if validating input in the future, confirm the correct number of words
                    let cmd = words[0].to_lowercase();
                    match cmd.as_str() {
                        "pop" => Pop::new(
                            // TODO: if validating input in the future, replace "unwrap"
                            Segment::from_str(&words[1].to_lowercase()).unwrap(),
                            words[2].parse::<u16>().unwrap(),
                        ).write(asm),
                        "push" => Push::new(
                            // TODO: if validating input in the future, replace "unwrap"
                            Segment::from_str(&words[1].to_lowercase()).unwrap(),
                            words[2].parse::<u16>().unwrap(),
                        ).write(asm),
                        "neg"  => UnaryOp::new(UnaryOpType::Neg).write(asm),
                        "not"  => UnaryOp::new(UnaryOpType::Not).write(asm),
                        "add"  => BinaryOp::new(BinaryOpType::Add).write(asm),
                        "and"  => BinaryOp::new(BinaryOpType::And).write(asm),
                        "or"   => BinaryOp::new(BinaryOpType::Or).write(asm),
                        "sub"  => BinaryOp::new(BinaryOpType::Sub).write(asm),
                        "eq"   => Comparison::new(ComparisonType::Eq).write(asm),
                        "gt"   => Comparison::new(ComparisonType::Gt).write(asm),
                        "lt"   => Comparison::new(ComparisonType::Lt).write(asm),
                        _ => return Err(format!("unknown command: {}", cmd)),
                    };
                    Ok(acc)
                }
            }
        })?.instructions;
    Ok(asm_str)
}