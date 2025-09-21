use colored::Colorize;
use log::debug;

use crate::vm::internals::{Expression, StackExpression, VMEvaluable, VMValuePointer};

impl std::fmt::Display for StackExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>2}{}{}{}{}{}",
            self.original_block.obj_id.cyan(),
            ".".black(),
            format!("{:?}", self.opcode).bright_green(),
            "(".black(),
            {
                let mut output = Vec::new();

                for (name, val) in &self.dependencies {
                    output.push(format!(
                        "{}{}{}",
                        name.to_lowercase().black(),
                        ": ".black(),
                        match val {
                            VMEvaluable::Bare(rv) =>
                                serde_json::to_string(rv).unwrap().bright_cyan().to_string(),
                            VMEvaluable::Block(b) => format!(
                                "{}{}{}",
                                b.original_block.obj_id.cyan(),
                                ".".black(),
                                format!("{:?}", b.opcode).bright_green(),
                            ),
                            VMEvaluable::Pointer(VMValuePointer::List { name, id }) => format!(
                                "{}{}{} {}{}{}",
                                "(".black(),
                                "list".yellow(),
                                ")".black(),
                                id.to_string().cyan(),
                                ".".black(),
                                name.bright_yellow()
                            ),
                            VMEvaluable::Pointer(VMValuePointer::Variable { name, id }) => format!(
                                "{}{}{} {}{}{}",
                                "(".black(),
                                "var".yellow(),
                                ")".black(),
                                id.to_string().cyan(),
                                ".".black(),
                                name.bright_yellow()
                            ),
                            VMEvaluable::Pointer(VMValuePointer::Broadcast { name, id }) =>
                                format!(
                                    "{}{}{} {}{}{}",
                                    "(".black(),
                                    "var".yellow(),
                                    ")".black(),
                                    id.to_string().cyan(),
                                    ".".black(),
                                    name.bright_yellow()
                                ),
                            VMEvaluable::Field(f) => format!(
                                "{}{}{}{}",
                                "[".black(),
                                f.display_value.bright_cyan(),
                                match &f.pointer {
                                    Some(VMValuePointer::List { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "list".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    Some(VMValuePointer::Variable { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "var".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    Some(VMValuePointer::Broadcast { name, id }) => format!(
                                        "{} {}{}{} {}{}{}",
                                        ",".black(),
                                        "(".black(),
                                        "var".yellow(),
                                        ")".black(),
                                        id.to_string().cyan(),
                                        ".".black(),
                                        name.bright_yellow()
                                    ),
                                    None => "".to_string(),
                                },
                                "]".black(),
                            ),
                            VMEvaluable::Default => "default".black().to_string(),
                        }
                    ));
                }

                output.join(&", ".black().to_string())
            },
            ")".black()
        )
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stack(s) => write!(f, "{}", s),
            _ => todo!(),
        }
    }
}

pub fn show_code(code: &Vec<Expression>) {
    let mut output = String::from("Pretty-printed code:\n");
    for c in code {
        output.push_str(format!("{c}\n").as_str());
    }
    debug!("{output}");
}
