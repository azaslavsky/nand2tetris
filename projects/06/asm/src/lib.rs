use std::str::FromStr;
use std::fmt::Error;
use std::num::ParseIntError;
use std::collections::HashMap;

enum CommandType {
    A,
    C,
}

trait Encodeable: Sized + Into<u16> {
    const CMD_TYPE: CommandType;
    const SHIFT_DISTANCE: u16;
    fn apply(self: Self, command: &Command) -> Command {
        let type_bit: u16 = match Self::CMD_TYPE {
            CommandType::A => 0b0,
            CommandType::C => 0b111,
        } << 13;
        let local = self.into();
        let encoded = (local << Self::SHIFT_DISTANCE) | type_bit | command.0;
        return Command(encoded)
    }
}

struct Addr(u16);
impl FromStr for Addr {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Addr(s.parse::<u16>()?))
    }
}
impl Encodeable for Addr {
    const CMD_TYPE: CommandType = CommandType::A;
    const SHIFT_DISTANCE: u16 = 0;
}
impl Into<u16> for Addr {
    fn into(self) -> u16 {
        self.0
    }
}

struct Comp(u16);
impl FromStr for Comp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Comp(match s {
            "0"   => 0b0101010,
            "1"   => 0b0111111,
            "-1"  => 0b0111010,
            "D"   => 0b0001100,
            "A"   => 0b0110000,
            "M"   => 0b1110000,
            "!D"  => 0b0001101,
            "!A"  => 0b0110001,
            "!M"  => 0b1110001,
            "-D"  => 0b0001111,
            "-A"  => 0b0110011,
            "-M"  => 0b1110011,
            "D+1" => 0b0011111,
            "A+1" => 0b0110111,
            "M+1" => 0b1110111,
            "D-1" => 0b0001110,
            "A-1" => 0b0110010,
            "M-1" => 0b1110010,
            "D+A" => 0b0000010,
            "D+M" => 0b1000010,
            "D-A" => 0b0010011,
            "D-M" => 0b1010011,
            "A-D" => 0b0000111,
            "M-D" => 0b1000111,
            "D&A" => 0b0000000,
            "D&M" => 0b1000000,
            "D|A" => 0b0010101,
            "D|M" => 0b1010101,
            _     => 0,
        }))
    }
}
impl Encodeable for Comp {
    const CMD_TYPE: CommandType = CommandType::C;
    const SHIFT_DISTANCE: u16 = 6;
}
impl Into<u16> for Comp {
    fn into(self) -> u16 {
        self.0
    }
}

struct Dest(u16);
impl FromStr for Dest {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dest(match s {
            "M"   => 0b001,
            "D"   => 0b010,
            "MD"  => 0b011,
            "A"   => 0b100,
            "AM"  => 0b101,
            "AD"  => 0b110,
            "AMD" => 0b111,
            _     => 0,
        }))
    }
}
impl Encodeable for Dest {
    const CMD_TYPE: CommandType = CommandType::C;
    const SHIFT_DISTANCE: u16 = 3;
}
impl Into<u16> for Dest {
    fn into(self) -> u16 {
        self.0
    }
}

struct Jump(u16);
impl FromStr for Jump {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Jump(match s {
            "JGT" => 0b001,
            "JEQ" => 0b010,
            "JGE" => 0b011,
            "JLT" => 0b100,
            "JNE" => 0b101,
            "JLE" => 0b110,
            "JMP" => 0b111,
            _     => 0,
        }))
    }
}
impl Encodeable for Jump {
    const CMD_TYPE: CommandType = CommandType::C;
    const SHIFT_DISTANCE: u16 = 0;
}
impl Into<u16> for Jump {
    fn into(self) -> u16 {
        self.0
    }
}

enum TokenClass {
    Addr,
    Comp,
    Dest,
    Jump,
    Label,
}

struct Token {
    class: TokenClass,
    // TODO: this could probably be a reference to avoid copying.
    word: Vec<char>,
    num: bool,
}

impl Token {
    fn new(class: TokenClass) -> Self {
        Self {
            class,
            word: vec![],
            num: true,
        }
    }

    fn encode(&self, command: &Command, symbol_table: &mut SymbolTable) -> Option<Command> {
        let expr = self.word.iter().collect::<String>();
        // TODO: Lot's of unwraps in this block, should implement better error handling.
        Some(match self.class {
            TokenClass::Addr  => {
                let a = match self.num {
                    true => Addr::from_str(expr.as_str()).unwrap(),
                    false => Addr(*(symbol_table.query(expr.as_str())?)),
                };
                a.apply(command)
            },
            TokenClass::Comp  => Comp::from_str(expr.as_str()).unwrap().apply(command),
            TokenClass::Dest  => Dest::from_str(expr.as_str()).unwrap().apply(command),
            TokenClass::Jump  => Jump::from_str(expr.as_str()).unwrap().apply(command),
            TokenClass::Label => return None,
        })
    }

    fn convert(&mut self, class: TokenClass) {
        self.class = class;
    }
}

struct SymbolTable {
    next_addr: u16,
    table: HashMap<String, u16>,
}
impl SymbolTable {
    fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("SP".to_string(), 0u16);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);
        for i in 0u16..16 {
            table.insert("R".to_owned() + &i.to_string(), i);
        }
        Self {
            next_addr: 16,
            table,
        }
    }

    // Looks up the value in the symbol table.  If it does not exist, assigns a new RAM location to
    // the symbol, and returns that instead.
    fn query(&mut self, key: &str) -> Option<&u16> {
        if !self.table.contains_key(key) {
            self.table.insert(key.to_string(), self.next_addr);
            self.next_addr += 1;
        }
        self.table.get(key)
    }

    fn insert(&mut self, key: String, addr: u16) {
        self.table.insert(key, addr);
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Command(u16);
impl Command {
    pub fn num(&self) -> u16 {
        self.0
    }
}
impl From<u16> for Command {
    fn from(num: u16) -> Self {
        Self(num)
    }
}
impl Into<u16> for Command {
    fn into(self) -> u16 {
        self.0
    }
}

// Does the first pass to identify all labels.  This is necessary because labels may reference
// locations ahead of them in the file, so we need to parse them separately.
fn parse_labels(text: &str, mut symbol_table: SymbolTable) -> SymbolTable {
    let mut token = Token::new(TokenClass::Label);
    let mut ignore_line = false;
    let mut next_line = 0;
    text.chars().for_each(|c| {
        if c == '\n' {
            ignore_line = false;
        }
        if ignore_line {
            return;
        }
        match c {
            '(' => {
                // Label start.
                token = Token::new(TokenClass::Label);
            },
            ')' => {
                // Label end - save the symbol, then go to the next line.
                symbol_table.insert(token.word.iter().collect(), next_line);
                token = Token::new(TokenClass::Label);
            },
            '/' => {
                // Comments immediately terminate the instruction.
                ignore_line = true;
            },
            '\n' | '\t' | '\r' => {
                if !token.word.is_empty() {
                    next_line += 1;
                }
                token = Token::new(TokenClass::Label);
            },
            _ => {
                // All other characters are part of declarations.
                token.word.push(c.clone());
            },
        }
    });
    symbol_table
}

fn parse_commands(text: &str, symbol_table: &mut SymbolTable) -> Vec<Command> {
    let mut token = Token::new(TokenClass::Dest);
    let mut inst = None;
    let mut ignore_line = false;
    let mut commands = vec![];
    text.chars().for_each(|c| {
        if c == '\n' {
            ignore_line = false;
        }
        if ignore_line {
            return;
        }
        match c {
            '@' => {
                // Start an address.
                token = Token::new(TokenClass::Addr);
            },
            ';' => {
                // Close the comp, and start a jump.
                token.convert(TokenClass::Comp); // <- TODO: Hacky
                // TODO: Lot's of unwraps here, could probably stand to have better error handling.
                inst = Some(token.encode(&inst.unwrap(), symbol_table).unwrap());
                token = Token::new(TokenClass::Jump);
            },
            '=' => {
                // Close the dest, and start a comp.
                inst = Some(token.encode(&inst.unwrap(), symbol_table).unwrap());
                token = Token::new(TokenClass::Comp);
            },
            '/' | '(' => {
                // Comments/labels immediately terminate the instruction.
                ignore_line = true;
            },
            '\n' | '\t' | '\r' => {
                if inst.is_none() {
                    // There was no instruction at all - just an empty line or a comment.
                    return;
                }
                commands.push(token.encode(&inst.unwrap(), symbol_table).unwrap());
                inst = None;
                token = Token::new(TokenClass::Dest);
            },
            _ => {
                // All other characters are part of declarations.
                if c.is_whitespace() {
                    return;
                }
                if inst.is_none() {
                    inst = Some(Command(0));
                }
                if !c.is_digit(10) {
                    token.num = false;
                }
                token.word.push(c.clone());
            },
        }
    });
    if inst.is_some() {
        commands.push(token.encode(&inst.unwrap(), symbol_table).unwrap());
    }
    commands
}

pub struct Parser {
    commands: Vec<Command>,
}
impl Parser {
    pub fn new(text: String) -> Self {
        Self {
            commands: parse_commands(text.as_str(), &mut parse_labels(text.as_str(),SymbolTable::new())),
        }
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
}