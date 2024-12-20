use core::panic;

pub trait Convert {
    fn to_string(&self) -> String;
}
enum Reg {
    rax(String),
}
impl Convert for Reg {
    fn to_string(&self) -> String {
        match self {
            Reg::rax(str) => str.to_owned(),
        }
    }
}

impl Convert for Instr {
    fn to_string(&self) -> String {
        match self {
            Instr::Mov(r, i) => {
                let reg: String = match r {
                    Reg::rax(str) => "rax".to_string(),
                };
                return format!("Mov {}, {}", reg, i);
            }
            Instr::Add(r, i) => {
                let reg: String = match r {
                    Reg::rax(str) => "rax".to_string(),
                };
                return format!("Add {}, {}", reg, i);
            }
        }
    }
}

impl Convert for Vec<Instr> {
    fn to_string(&self) -> String {
        let mut rs: String = "".to_string();
        for i in self.iter() {
            rs += (i.to_string() + "\n").as_str();
        }
        rs
    }
}

enum Exp {
    Num(i64),
    Add1(Box<Exp>),
    //Sub1(Box<Exp>),
}

enum Instr {
    Mov(Reg, i64),
    Add(Reg, i64),
}

//str_to_exp --> exp_to_instr --> instr.to_string()
fn str_to_exp(content: &str) -> Result<Exp, String> {
    let content_trimmed: &str = content.trim();
    let len: usize = content_trimmed.len();
    match i64::from_str_radix(content_trimmed, 10) {
        Ok(x) => Ok(Exp::Num(x)),
        Err(e) => {
            if len < 5 {
                panic!("can't be valid if not a number\n");
            }
            match &content_trimmed[0..5] {
                "Add1(" => {
                    return Ok(Exp::Add1(Box::<Exp>::new(
                        str_to_exp(&content_trimmed[5..len - 1]).unwrap(),
                    )))
                }
                &_ => panic!("invalid expression{:#?}\n", &content.trim()[0..]),
            }
        }
    }
}

fn exp_to_instr(e: &Exp) -> Vec<Instr> {
    match e {
        Exp::Num(n) => vec![Instr::Mov(Reg::rax("rax".to_string()), n.to_owned())],
        Exp::Add1(e) => {
            let mut is = exp_to_instr(e);
            is.push(Instr::Add(Reg::rax("rax".to_string()), 1));
            is
        }
    }
}
fn compile_to_string(e: &Exp) -> String {
    format!(
        "\
    section .text
    global start_here
    start_here:
        {}
    ret \n",
        exp_to_instr(e).to_string()
    )
}

fn main() -> Result<(), String> {
    let argv: Vec<String> = std::env::args().collect();
    let content = std::fs::read_to_string(argv[1].clone()).unwrap();
    let num = str_to_exp(&content)?; //parse the number into Exp format
    let compiled_string = compile_to_string(&num);
    println!("{compiled_string}");
    Ok(())
}
