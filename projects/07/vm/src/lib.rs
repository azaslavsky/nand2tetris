use std::str::FromStr;
use std::error::Error;

/// A struct to hold a set of instructions, which can be concatenated as a string to form an .asm
/// file.
struct Assembly {
    // Certain operations need unique ids to identify them in relation to other, similar operations.
    // This counter keeps track of the next available unique id.
    uid_counter: usize,

    // If the code is currently in a function definition, store that functions name until the
    // corresponding return value is reached.
    function_name: Option<String>,

    // Notes how many static addresses had been allocated prior to the current file being ingested.
    // Should always start at 16.
    static_counter: u16,

    // Notes how many static addresses have been allocated by the current file.
    static_allocs: u16,
    instructions: String,
}
impl Assembly {
    fn new() -> Self {
        Self {
            uid_counter: 0,
            function_name: None,
            static_counter: 16,
            static_allocs: 0,
            instructions: String::new(),
        }
    }

    fn append(&mut self, appending: &str) {
        self.instructions += appending;
    }
}

/// Adds some assembly commands to the buffer.
trait Write {
    fn write(&self, assembly: &mut Assembly);
}

/// Loads a single element from the stack such that it is available at "M."
fn read_one_input(assembly: &mut Assembly) {
    assembly.append( "\
        @SP\n\
        M=M-1\n\
        @SP\n\
        A=M\n\
    ");
}

/// Loads two elements from the stack: one available at "D," the other at "M."
fn read_two_inputs(assembly: &mut Assembly) {
    read_one_input(assembly);
    assembly.append( "\
        D=M\n\
        @SP\n\
        M=M-1\n\
        @SP\n\
        A=M\n\
    ");
}

/// After an operation has been performed, writes the output back onto the stack.
fn write_output(assembly: &mut Assembly) {
    assembly.append( "\
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
        assembly.append( match self.typ {
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
        assembly.append(match self.typ {
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
        let jump = match self.typ {
            ComparisonType::Eq => "JEQ",
            ComparisonType::Gt => "JGT",
            ComparisonType::Lt => "JLT",
        };
        let tr = format!("TRUE{}", assembly.uid_counter);
        let co = format!("CONT{}", assembly.uid_counter);
        assembly.uid_counter += 1;

        read_two_inputs(assembly);
        assembly.append(format!("\
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
#[derive(Clone,Copy,PartialEq)]
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
            Segment::Constant => assembly.append(format!("\
                @{}\n\
                D=A\n\
            ", self.value).as_str()),
            Segment::Pointer => assembly.append(push((self.value + 3).to_string()).as_str()),
            Segment::Static => assembly.append(push((assembly.static_counter + self.value).to_string()).as_str()),
            Segment::Temp => assembly.append(push(format!("R{}", (self.value + 5))).as_str()),
            _ => assembly.append(format!("\
                @{}\n\
                D=A\n\
                {}\n\
                A=M+D\n\
                D=M\n\
            ", self.value, self.segment.into_variable()).as_str()),
        }

        assembly.append( "\
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
            Segment::Pointer => assembly.append(pop((self.value + 3).to_string()).as_str()),
            Segment::Static => assembly.append(pop((assembly.static_counter + self.value).to_string()).as_str()),
            Segment::Temp => assembly.append(pop(format!("R{}", (self.value + 5))).as_str()),
            _ => assembly.append(format!("\
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

struct Function<'a> {
    name: &'a str,
    locals: u16,
}
impl<'a> Function<'a> {
    fn new(name: &'a str, locals: u16) -> Self {
        Self{name, locals}
    }
}
impl<'a> Write for Function<'a> {
    fn write(&self, assembly: &mut Assembly) {
        assembly.function_name = Some(self.name.to_string());
        assembly.append(format!("({})\n", self.name).as_str());

        // Initialize the local variables to zero.
        for _ in 0..self.locals {
            assembly.append("\
                @0\n\
                D=A\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n\
            ");
        }

        // Adjust the LCL pointer to point back to where the first initialized local was.
        assembly.append(format!("\
            @{}\n\
            D=A\n\
            @SP\n\
            D=M-D\n\
            @LCL\n\
            M=D\n\
        ", self.locals.to_string()).as_str());
    }
}

struct Call<'a> {
    name: &'a str,
    args: u16,
}
impl<'a> Call<'a> {
    fn new(name: &'a str, args: u16) -> Self {
        Self{name, args}
    }
}
impl<'a> Write for Call<'a> {
    fn write(&self, assembly: &mut Assembly) {
        let ret = format!("RET{}", assembly.uid_counter);
        assembly.uid_counter += 1;

        // Push the return address onto the stack.
        assembly.append(format!("\
            @{}\n\
            D=A\n\
            @SP\n\
            A=M\n\
            M=D\n\
            @SP\n\
            M=M+1\n\
        ", ret).as_str());

        // Push each of the existing LCL, ARG, THIS and THAT pointers onto the stack, in order.
        for i in 1..=4 {
            assembly.append( format!("\
                @R{}\n\
                D=M\n\
                @SP\n\
                A=M\n\
                M=D\n\
                @SP\n\
                M=M+1\n\
            ", i.to_string()).as_str());
        }

        // Adjust the ARG and LCL pointers, and add an assembly label for the return address.
        assembly.append(format!("\
            @{}\n\
            D=A\n\
            @SP\n\
            D=M-D\n\
            @ARG\n\
            M=D\n\
            @SP\n\
            D=M\n\
            @LCL\n\
            M=D\n\
            @{}\n\
            0;JMP\n\
            ({})\n\
        ", (self.args + 5).to_string(), self.name, ret).as_str());
    }
}

struct Return;
impl Return {
    fn new() -> Self {
        Self
    }
}
impl Write for Return {
    fn write(&self, assembly: &mut Assembly) {
        // Save the value of LCL to R14, and the value of the return address to R15.
        assembly.append("\
            @LCL\n\
            D=M\n\
            @R14\n\
            M=D\n\
            @5\n\
            A=D-A\n\
            D=M\n\
            @R15\n\
            M=D\n\
        ");

        // Pop the return value, and replace the returning function's first argument with it.
        Pop::new(Segment::Argument, 0).write(assembly);

        // Increment the argument pointer - this will be the location of the stack pointer after the
        // returning function has fully exited.
        assembly.append("\
            @ARG\n\
            D=M+1\n\
            @SP\n\
            M=D\n\
        ");

        // Restore the calling functions THAT, THIS, ARG, and LCL pointers, in that order.
        for i in 1..=4 {
            assembly.append(format!("\
                @{}\n\
                D=A\n\
                @R14\n\
                A=M-D\n\
                D=M\n\
                @{}\n\
                M=D\n\
            ", i, 5 - i).as_str());
        }

        // Jump to the return address.
        assembly.append("\
            @R15\n\
            A=M\n\
            0;JMP\n\
        ");
    }
}

struct Label<'a> {
    label: &'a str,
}
impl<'a> Label<'a> {
    fn new(label: &'a str) -> Self {
        Self{label}
    }
}
impl<'a> Write for Label<'a> {
    fn write(&self, assembly: &mut Assembly) {
        assembly.append(format!("({}${})\n", assembly.function_name.as_ref().unwrap_or(&String::new()), self.label).as_str());
    }
}

struct Goto<'a> {
    label: &'a str,
}
impl<'a> Goto<'a> {
    fn new(label: &'a str) -> Self {
        Self{label}
    }
}
impl<'a> Write for Goto<'a> {
    fn write(&self, assembly: &mut Assembly) {
        assembly.append(format!("\
            @{}${}\n\
            0;JMP\n\
        ", assembly.function_name.as_ref().unwrap_or(&String::new()), self.label).as_str());
    }
}

struct IfGoto<'a> {
    label: &'a str,
}
impl<'a> IfGoto<'a> {
    fn new(label: &'a str) -> Self {
        Self{label}
    }
}
impl<'a> Write for IfGoto<'a> {
    fn write(&self, assembly: &mut Assembly) {
        assembly.append(format!("\
            @SP\n\
            M=M-1\n\
            A=M\n\
            D=M\n\
            @{}${}\n\
            D;JNE\n\
        ", assembly.function_name.as_ref().unwrap_or(&String::new()), self.label).as_str());
    }
}

struct Boot;
impl Boot {
    fn new() -> Self {
        Self
    }
}
impl Write for Boot {
    fn write(&self, assembly: &mut Assembly) {
        // Set the stack pointer to 256, because that's where the system expects to start, per the
        // spec.
        assembly.append("\
            @256\n\
            D=A\n\
            @SP\n\
            M=D\n\
        ");

        // Call into "Sys.init" (a special-cased, default start function) to get things rolling.
        Call::new("Sys.init", 0).write(assembly);

    }
}

pub struct Translator {
    assembly: Assembly,
}
impl Translator {
    pub fn new() -> Self {
        Self {
            assembly: Assembly::new(),
        }
    }

    pub fn add_vm_file(&mut self, text: &str) -> Result<(),Box<dyn Error>> {
        self.assembly.static_allocs = 0;
        text.split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && s.chars().nth(0).unwrap() != '/')
            .map(|line| {
                let asm = &mut self.assembly;
                let words = line.split_whitespace().collect::<Vec<&str>>();
                let cmd = words[0].to_lowercase();

                asm.append( format!("\n// {}:\n", line).as_str());
                println!("line: {}", line);

                // TODO: if validating input in the future, confirm the correct number of words
                match cmd.as_str() {
                    "boot" => Boot::new().write(asm),
                    "function" => Function::new(
                        words[1],
                        words[2].parse::<u16>().unwrap(),
                    ).write(asm),
                    "call" => Call::new(
                        words[1],
                        words[2].parse::<u16>().unwrap(),
                    ).write(asm),
                    "return" => Return::new().write(asm),
                    "label" => Label::new(words[1]).write(asm),
                    "goto" => Goto::new(words[1]).write(asm),
                    "if-goto" => IfGoto::new(words[1]).write(asm),
                    "pop" => {
                        let segment = Segment::from_str(&words[1].to_lowercase()).unwrap();
                        let index = words[2].parse::<u16>().unwrap();
                        Pop::new(segment, index).write(asm);
                        if segment == Segment::Static {
                            self.assembly.static_allocs = std::cmp::max(self.assembly.static_allocs, index + 1  );
                        }
                    },
                    "push" => {
                        let segment = Segment::from_str(&words[1].to_lowercase()).unwrap();
                        let index = words[2].parse::<u16>().unwrap();
                        Push::new(segment, index).write(asm);
                        if segment == Segment::Static {
                            self.assembly.static_allocs = std::cmp::max(self.assembly.static_allocs, index + 1);
                        }
                    },
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
                Ok(())
            })
            .collect::<Result<Vec<_>,_>>()?;

        self.assembly.static_counter += self.assembly.static_allocs;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        self.assembly.instructions.to_owned()
    }
}
