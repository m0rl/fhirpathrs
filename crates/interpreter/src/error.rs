#[derive(Debug)]
pub enum InterpreterError {
    InvalidOperation(String),
    TypeMismatch(String),
    UnknownFunction(String),
    UnknownConstant(String),
    DivisionByZero,
    InvalidRegex(String),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            InterpreterError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            InterpreterError::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            InterpreterError::UnknownConstant(name) => write!(f, "Unknown constant: {}", name),
            InterpreterError::DivisionByZero => write!(f, "Division by zero"),
            InterpreterError::InvalidRegex(msg) => write!(f, "Invalid regex: {}", msg),
        }
    }
}

impl std::error::Error for InterpreterError {}
