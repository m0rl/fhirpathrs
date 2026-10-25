# fhirpathrs

A high-performance [FHIRPath](https://hl7.org/fhirpath/) engine written in Rust with Python bindings.

FHIRPath is a path-based navigation and extraction language for [FHIR](https://hl7.org/fhir/) (Fast Healthcare Interoperability Resources) data, widely used in healthcare systems for querying clinical data, validating resources, and implementing decision support logic.

## Key Features

- **Fast** — Rust-powered parser and interpreter, 10-600x faster than pure-Python alternatives
- **Safe** — Fully iterative evaluation (no call-stack recursion)
- **Complete** — 100+ FHIRPath functions including collections, math, strings, dates, types, and higher-order functions
- **Python-native** — Install with pip, pass dicts and lists, get dicts and lists back

## Installation

```bash
pip install fhirpathrs
```

### Build from source

```bash
# Rust toolchain required
pip install maturin
maturin develop --release --manifest-path crates/python/Cargo.toml
```

## Quick Start

```python
from fhirpathrs import evaluate

patient = {
    "resourceType": "Patient",
    "id": "pat-42",
    "active": True,
    "name": [
        {"family": "Jetson", "given": ["George", "Astro"]},
        {"family": "Jetson-Spacely", "given": ["Elroy"]},
    ],
    "address": [
        {"city": "Orbit City", "state": "Space"}
    ],
}

# Navigate fields
evaluate("name.family", patient)
# → ["Jetson", "Jetson-Spacely"]

# Filter collections
evaluate("name.where(family = 'Jetson').given", patient)
# → ["George", "Astro"]

# Boolean logic
evaluate("name.exists() and active", patient)
# → True

# Count results
evaluate("name.count()", patient)
# → 2
```

## Python API

```python
evaluate(expression: str, data: Any = None, constants: dict | None = None) -> Any
```

| Parameter | Description |
|---|---|
| `expression` | FHIRPath expression string |
| `data` | Input data — any combination of dicts, lists, strings, numbers, bools, and None |
| `constants` | Optional external constants, accessible as `%name` in expressions |

Returns the evaluation result as a Python object. Raises `ValueError` on parse errors and `RuntimeError` on evaluation errors.

## Examples

### Path Navigation

```python
data = {"a": {"b": {"c": 42}}}
evaluate("a.b.c", data)  # → 42
```

### Filtering with `where()`

```python
bundle = {
    "entry": [
        {"resource": {"resourceType": "Observation", "status": "final", "value": 120}},
        {"resource": {"resourceType": "Condition", "status": "active"}},
        {"resource": {"resourceType": "Observation", "status": "final", "value": 80}},
    ]
}

evaluate(
    "entry.resource.where(resourceType = 'Observation').value",
    bundle,
)
# → [120, 80]
```

### QuestionnaireResponse Navigation

```python
data = {
    "QuestionnaireResponse": {
        "resourceType": "QuestionnaireResponse",
        "status": "completed",
        "item": [
            {
                "linkId": "blood-pressure",
                "item": [
                    {"linkId": "systolic", "answer": [{"valueInteger": 120}]},
                    {"linkId": "diastolic", "answer": [{"valueInteger": 80}]},
                ],
            },
            {
                "linkId": "heart-rate",
                "answer": [{"valueInteger": 72}],
            },
        ],
    }
}

# %QuestionnaireResponse resolves from the data context
evaluate(
    "%QuestionnaireResponse.repeat(item).where(linkId = 'systolic').answer.valueInteger",
    data,
)
# → [120]

evaluate(
    "%QuestionnaireResponse.repeat(item).where(linkId = 'heart-rate').answer.valueInteger",
    data,
)
# → [72]

# Collect all answers across nesting levels
evaluate("%QuestionnaireResponse.repeat(item).answer.valueInteger", data)
# → [72, 120, 80]
```

### External Constants

```python
data = {"items": [{"value": 10}, {"value": 50}, {"value": 90}]}

evaluate(
    "items.where(value > %threshold).count()",
    data,
    constants={"threshold": 25},
)
# → 2
```

### Arithmetic and String Functions

```python
evaluate("1 + 2 * 3")               # → 7
evaluate("name.length()", {"name": "hello"})  # → 5
evaluate("name.upper()", {"name": "hello"})   # → "HELLO"
```

### Aggregation

```python
data = {"scores": [10, 20, 30, 40, 50]}

evaluate("scores.sum()", data)   # → 150
evaluate("scores.avg()", data)   # → 30.0
evaluate("scores.min()", data)   # → 10
evaluate("scores.max()", data)   # → 50

# Custom aggregation
evaluate("scores.aggregate($total + $this, 0)", data)  # → 150
```

### FHIR Date and Quantity Literals

```python
evaluate("@2024-01-15")           # → "2024-01-15"
evaluate("@T14:30:00")            # → "14:30:00"
evaluate("10 'mg'")               # → {"value": 10.0, "unit": "mg"}
```

### Pydantic Models

```python
from pydantic import BaseModel

class Patient(BaseModel):
    resourceType: str = "Patient"
    id: str
    active: bool

patient = Patient(id="pat-42", active=True)
evaluate("active", patient.model_dump())  # → True
```

### Type Testing and Conversion

```python
evaluate("(1).is(Integer)")              # → True
evaluate("(1.5).is(Decimal)")            # → True
evaluate("42.toString()")               # → "42"
evaluate("'123'.toInteger()")           # → 123
```

## Supported Functions

### Collection
`empty()` `exists()` `count()` `first()` `last()` `single()` `tail()` `take()` `skip()` `combine()` `distinct()` `isDistinct()` `intersect()` `exclude()` `subsetOf()` `supersetOf()` `not()` `hasValue()` `union()` `children()` `descendants()` `ofType()`

### Higher-Order
`where()` `select()` `all()` `repeat()` `repeatAll()` `sort()` `iif()` `coalesce()` `aggregate()` `defineVariable()`

### Math
`abs()` `ceiling()` `floor()` `round()` `truncate()` `sqrt()` `exp()` `ln()` `log()` `power()`

### String
`indexOf()` `substring()` `startsWith()` `endsWith()` `contains()` `upper()` `lower()` `replace()` `matches()` `matchesFull()` `replaceMatches()` `length()` `toChars()` `split()` `join()` `trim()` `lastIndexOf()` `encode()` `decode()` `escape()` `unescape()`

### Type Conversion
`toString()` `toInteger()` `toDecimal()` `toBoolean()` `toDate()` `toDateTime()` `toTime()` `toQuantity()` `toLong()` `convertsToInteger()` `convertsToDecimal()` `convertsToBoolean()` `convertsToString()` `convertsToDate()` `convertsToDateTime()` `convertsToTime()` `convertsToQuantity()` `convertsToLong()`

### Aggregate
`allTrue()` `anyTrue()` `allFalse()` `anyFalse()` `sum()` `avg()` `min()` `max()`

### Utility
`now()` `today()` `timeOfDay()` `type()` `precision()` `lowBoundary()` `highBoundary()` `trace()`

### Date/Time Components
`year()` `month()` `day()` `hour()` `minute()` `second()` `millisecond()` `timezone()` `duration()` `difference()` `comparable()`

### Operators
`+` `-` `*` `/` `div` `mod` `=` `!=` `~` `!~` `<` `>` `<=` `>=` `and` `or` `xor` `implies` `is` `as` `in` `contains` `|` `&`

## Performance

Benchmarked against [fhirpathpy](https://github.com/beda-software/fhirpathpy) (pure Python, ANTLR-based):

| Benchmark | fhirpathrs | fhirpathpy | Speedup |
|---|---|---|---|
| Field access (small object) | 1.6 us | 73 us | **47x** |
| Nested path (`a.b.c.d.e`) | 2.4 us | 268 us | **103x** |
| Arithmetic (`1 + 2 * 3 - 4`) | 2.3 us | 1,411 us | **627x** |
| `where()` filter (10 items) | 14 us | 745 us | **54x** |
| `where()` filter (1000 items) | 1,578 us | 15,504 us | **10x** |
| Patient field navigation | 3.8 us | 130 us | **34x** |
| Patient `where().given` | 6.3 us | 389 us | **61x** |
| Bundle filter+count (100 entries) | 191 us | 2,482 us | **13x** |
| 100-chained `.where(true)` | 102 us | 50,099 us | **489x** |
| 100-nested `iif()` | 152 us | crashes | -- |

## Architecture

```
crates/
├── parser/        # FHIRPath string → AST (nom + iterative Pratt parser)
├── interpreter/   # AST + data → result (iterative dispatch loop)
└── python/        # PyO3 bindings exposing evaluate()
```

All three layers are fully iterative — no call-stack recursion anywhere.

## Fuzzing

Coverage-guided fuzzing with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) (libFuzzer + AddressSanitizer) lives in `fuzz/`, a standalone crate outside the Cargo workspace.

| Target | Input | Exercises |
|---|---|---|
| `parse` | arbitrary string | `parser::parse`, formatting parse errors |
| `interpret` | arbitrary string | `parse` + `interpret` against a fixed Patient fixture at a fixed evaluation time; walks every result and trace value, formats errors; unparseable inputs are rejected from its corpus |

Prerequisites: a nightly toolchain and `cargo install cargo-fuzz`.

```bash
cargo +nightly fuzz run parse fuzz/corpus/parse fuzz/seeds -- \
  -dict=fuzz/dictionary.txt -create_missing_dirs=1 -max_len=65536 -timeout=10 -rss_limit_mb=2048

cargo +nightly fuzz run interpret fuzz/corpus/interpret fuzz/seeds -- \
  -dict=fuzz/dictionary.txt -create_missing_dirs=1 -timeout=10 -rss_limit_mb=2048
```

- `fuzz/seeds/` holds the committed seed expressions; the corpus directory must come first so libFuzzer writes new inputs into the gitignored `fuzz/corpus/<target>/`, never into the seeds.
- `fuzz/dictionary.txt` lists FHIRPath keywords, functions, operators and literal shapes.
- Add `-max_total_time=<seconds>` to stop after a fixed time.
- Crashes, timeouts and OOMs are written to `fuzz/artifacts/<target>/`; reproduce one with `cargo +nightly fuzz run <target> <artifact>`.
- `cargo +nightly fuzz cmin <target>` minimizes a grown corpus.
- CI (`fuzz` job in `.github/workflows/ci.yml`) runs each target for 60 seconds from the seeds on every push to `main`; any crash, timeout or OOM fails the job and uploads `fuzz/artifacts`.

Coverage of a corpus (needs `rustup component add --toolchain nightly llvm-tools-preview`):

```bash
cargo +nightly fuzz coverage interpret fuzz/corpus/interpret fuzz/seeds
llvm-cov report target/<host>/coverage/<host>/release/interpret \
  -instr-profile=fuzz/coverage/interpret/coverage.profdata \
  -ignore-filename-regex='(\.cargo|rustc|fuzz_targets)'
```

`<host>` is the host triple (e.g. `aarch64-apple-darwin`); `llvm-cov` lives in `$(rustc +nightly --print sysroot)/lib/rustlib/<host>/bin/`. Use `llvm-cov show` with the same arguments for line-level detail; uncovered code usually needs a new seed in `fuzz/seeds/`.

## License

MIT
