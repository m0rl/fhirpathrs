pub use crate::context::InterpreterContext;
pub use crate::value::Value;

use parser::{Expression, Invocation};

pub(crate) enum BinOp<'a> {
    Multiplicative(&'a parser::MultiplicativeOp),
    Additive(&'a parser::AdditiveOp),
    Union,
    Inequality(&'a parser::InequalityOp),
    Equality(&'a parser::EqualityOp),
    Membership(&'a parser::MembershipOp),
}

pub(crate) enum Frame<'a> {
    Invocation(&'a Invocation),
    IndexerEvalIndex(&'a Expression),
    IndexerCombine(Value),
    Polarity(&'a parser::PolarityOp),
    BinaryEvalRight(&'a Expression, BinOp<'a>),
    BinaryCombine(Value, BinOp<'a>),
    TypeApply(&'a parser::TypeOp, &'a parser::TypeSpecifier),
    AndAfterLeft(&'a Expression, InterpreterContext),
    AndAfterRight(Value, InterpreterContext),
    OrAfterLeft(&'a Expression, &'a parser::OrOp, InterpreterContext),
    OrAfterRight(Value, &'a parser::OrOp, InterpreterContext),
    ImpliesAfterLeft(&'a Expression, InterpreterContext),
    ImpliesAfterRight(Value, InterpreterContext),
    WhereApply {
        items: Vec<Value>,
        expr: &'a Expression,
        index: usize,
        results: Vec<Value>,
        saved_ctx: InterpreterContext,
    },
    SelectApply {
        items: Vec<Value>,
        expr: &'a Expression,
        index: usize,
        results: Vec<Value>,
        saved_ctx: InterpreterContext,
    },
    AllApply {
        items: Vec<Value>,
        expr: &'a Expression,
        index: usize,
        saved_ctx: InterpreterContext,
    },
    ExistsApply {
        items: Vec<Value>,
        expr: &'a Expression,
        index: usize,
        saved_ctx: InterpreterContext,
    },
    FuncArgs {
        base: Value,
        name: &'a str,
        args: &'a [Expression],
        index: usize,
        evaluated: Vec<Value>,
        saved_ctx: InterpreterContext,
    },
    RepeatApply {
        dedup: bool,
        to_process: Vec<Value>,
        index: usize,
        expr: &'a Expression,
        result: Vec<Value>,
        new_items: Vec<Value>,
        saved_ctx: InterpreterContext,
    },
    IifBranch {
        args: &'a [Expression],
        eval_ctx: InterpreterContext,
        saved_ctx: InterpreterContext,
    },
    IifRestore {
        saved_ctx: InterpreterContext,
    },
    TraceAfterName {
        base: Value,
        args: &'a [Expression],
        saved_ctx: InterpreterContext,
    },
    TraceLoop {
        base: Value,
        name: String,
        items: Vec<Value>,
        index: usize,
        projection: &'a Expression,
        saved_ctx: InterpreterContext,
    },
    AggregateInit {
        items: Vec<Value>,
        aggregator: &'a Expression,
        saved_ctx: InterpreterContext,
    },
    AggregateLoop {
        items: Vec<Value>,
        aggregator: &'a Expression,
        index: usize,
        saved_ctx: InterpreterContext,
    },
    SortEval {
        items: Vec<Value>,
        criteria: &'a Expression,
        index: usize,
        keyed: Vec<(Value, Value)>,
        descending: bool,
        saved_ctx: InterpreterContext,
    },
    DefineVarEvalName {
        base: Value,
        args: &'a [Expression],
        saved_ctx: InterpreterContext,
    },
    DefineVarEval {
        base: Value,
        name: String,
        saved_ctx: InterpreterContext,
    },
    CoalesceArgs {
        args: &'a [Expression],
        index: usize,
        saved_ctx: InterpreterContext,
    },
}
