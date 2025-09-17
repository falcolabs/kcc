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
pub struct VMError {
    pub trace: Vec<Trace>,
}

impl VMError {
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
