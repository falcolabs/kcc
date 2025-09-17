use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum ErrorType {
    TypeError,
    NotFoundError,
    SyntaxError,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub error_type: ErrorType,
    pub description: String,
    pub location: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScratchError {
    pub trace: Vec<Trace>,
}

impl std::error::Error for ScratchError {}

impl Display for ScratchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error occured. Traceback:\n")?;
        for t in &self.trace {
            write!(f, "at {}\n", t.location)?;
            write!(f, "    {:?}: {}\n", t.error_type, t.description)?;
        }
        write!(f, "See the above traceback for details.\n")
    }
}

impl ScratchError {
    pub fn syntax_error<T: ToString, Q: ToString>(description: T, location: Q) -> Self {
        Self {
            trace: vec![Trace {
                error_type: ErrorType::SyntaxError,
                description: description.to_string(),
                location: location.to_string(),
            }],
        }
    }

    pub fn type_error<T: ToString, Q: ToString>(description: T, location: Q) -> Self {
        Self {
            trace: vec![Trace {
                error_type: ErrorType::TypeError,
                description: description.to_string(),
                location: location.to_string(),
            }],
        }
    }

    pub fn not_found<T: ToString, Q: ToString>(description: T, location: Q) -> Self {
        Self {
            trace: vec![Trace {
                error_type: ErrorType::NotFoundError,
                description: description.to_string(),
                location: location.to_string(),
            }],
        }
    }

    pub fn internal<T: ToString, Q: ToString>(description: T, location: Q) -> Self {
        Self {
            trace: vec![Trace {
                error_type: ErrorType::Internal,
                description: description.to_string(),
                location: location.to_string(),
            }],
        }
    }

    pub fn push_syntax_error<T: ToString, Q: ToString>(
        mut self,
        description: T,
        location: Q,
    ) -> Self {
        self.trace.push(Trace {
            error_type: ErrorType::SyntaxError,
            description: description.to_string(),
            location: location.to_string(),
        });
        self
    }

    pub fn push_type_error<T: ToString, Q: ToString>(
        mut self,
        description: T,
        location: Q,
    ) -> Self {
        self.trace.push(Trace {
            error_type: ErrorType::TypeError,
            description: description.to_string(),
            location: location.to_string(),
        });
        self
    }

    pub fn push_not_found<T: ToString, Q: ToString>(mut self, description: T, location: Q) -> Self {
        self.trace.push(Trace {
            error_type: ErrorType::NotFoundError,
            description: description.to_string(),
            location: location.to_string(),
        });
        self
    }

    pub fn push_internal<T: ToString, Q: ToString>(mut self, description: T, location: Q) -> Self {
        self.trace.push(Trace {
            error_type: ErrorType::Internal,
            description: description.to_string(),
            location: location.to_string(),
        });
        self
    }
}
