[![Crates.io](https://img.shields.io/crates/v/hbp100.svg)](https://crates.io/crates/hbp100)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# hummin-bird

hbp100 is a privacy system , it is placed between a user and a llm , it finds sensitive
values, swaps them with placeholders , before the text leaves the local device ,and restores 
the originals after the response comes back. 

---

## New changes

### Config based extractors

see 

>ex*/syntax.md.

### Sessions

Sessions added and now working , check the api section .


### Eval

| Metric    | Result  |
| --------- | ------: |
| Accuracy  | 91.34%  |
| Precision | 95.57%  |
| Recall    | 84.59%  |
| F1        | 89.75%  |

---

## Tree

.
├── assets
│   └── hbp100-v3.lgb
├── Cargo.lock
├── Cargo.toml
├── dataset.json
├── dev
│   ├── session.rs
│   ├── speed.py
│   ├── test.py
│   └── tst.rs
├── LICENSE
├── README.md
└── src
    ├── api.rs
    ├── core
    │   ├── engine.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   ├── pipeline.rs
    │   └── session_manager.rs
    ├── extractors
    │   ├── config.rs
    │   ├── manager.rs
    │   ├── mod.rs
    │   └── syntax.md
    ├── interfaces
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   └── predictor.rs
    ├── lib.rs
    ├── ml
    │   ├── dataset.rs
    │   ├── features.rs
    │   ├── model.rs
    │   └── mod.rs
    ├── placeholders
    │   ├── generator.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   ├── restore.rs
    │   ├── session_aware_generator.rs
    │   └── validator.rs
    ├── policy_engine
    │   ├── context_builder.rs
    │   ├── mod.rs
    │   └── predictor.rs
    ├── pyproject.toml
    ├── schemas
    │   ├── decision.rs
    │   ├── entity.rs
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   ├── result.rs
    │   └── session.rs
    └── utils
        ├── helpers.rs
        ├── logger.rs
        └── mod.rs

12 directories, 47 files
---

## Install

```bash
cargo add hbp100          
pip install (the github download link for 3.2.1 release)
```

```bash
git clone && cd humming-bird-v3
python -m venv env
source env/bin/act*
maturin develop --release
```

> always use --release for speed , using build only no release decreases speed by a lot likely 10x times

---

## Py api

```python
engine.process(text, intent=None)              # -> dict
engine.restore(text)                            # -> str
engine.res_wmd(text, metadata)                  # -> str
engine.vald_res(response)                       # -> (bool, Optional[str])
engine.add_extractor(config_json)               # -> None
engine.add_extractor_from_file(path)            # -> None
engine.enex(name)                               # -> bool
engine.dis_ex(name)                             # -> bool
engine.ls_ex()                                  # -> list[str]
engine.lsen()                                   # -> list[str]
engine.rstex()                                  # -> None
```

## Rust api

```rust
engine.process(text: &str, intent: Option<&str>) -> ProcessResult
engine.restore(text: &str) -> String
engine.restore_with_metadata(text: &str, metadata: HashMap<String, String>) -> String
engine.validate_response(text: &str) -> (bool, Option<String>)

engine.add_conjson(json: &str) -> Result<(), String>
engine.add_conf(path: &str) -> Result<(), String>
engine.add_conex(config: Excon) -> Result<(), String>
engine.enex(name: &str) -> bool
engine.dis_ex(name: &str) -> bool
engine.ls_ex() -> Vec<String>
engine.lsen() -> Vec<String>
engine.rst_def()
```

Excon is the config struct:

```rust

pub struct Excon {
    pub name: String,
    pub entity_type: String,
    pub pattern: String,
    pub confidence: Option<f32>,
}

```rust
use hbp100::HBP100;
use hbp100::extractors::Excon;

let mut engine = HBP100::new();

engine.add_conjson(r#"{
    "name": "PAN_India",
    "entity_type": "PAN",
    "pattern": "[A-Z]{5}[0-9]{4}[A-Z]",
    "confidence": 0.98
}"#)?;

let result = engine.process("My PAN is ABCDE1234F", None);
```

Or build the config in rust:

```rust
engine.add_conex(Excon {
    name: "EmployeeNumber".into(),
    entity_type: "EMPLOYEE_ID".into(),
    pattern: "EMP-[0-9]{6}".into(),
    confidence: Some(0.95),
})?;
```

---

## Example

```python
from hbp100 import HBP100

engine = HBP100()
for cfg in [
    r'{"name":"Name","entity_type":"NAME","pattern":"\\b[A-Z][a-z]+ [A-Z][a-z]+\\b"}',
    r'{"name":"MRN", "entity_type":"MRN", "pattern":"\\b\\d{6}\\b"}',
]:
    engine.add_extractor(cfg)

result = engine.process(
    "Patient John Doe, MRN: 123456, prescribed Metformin 500mg daily.",
    intent="hospital_discharge",
)

print(result["masked_text"])

for d in result["decisions"]:
    print(f"{d['entity_type']} -> {d['decision']} ({d['confidence']})")
```

## Performance

Release mode, Intel Core i5-1135G7, 3 extractors, 100,000 iterations × 5
rounds:

```
Mean:       45.2 µs/text
Throughput: 22,116 texts/sec
```

check the dev/test* and check the speed on your own


## Applications

- healthcare document processing 
- insurance workflows 
- ocr 
- ai assistants and llm front-ends 
- customer support systems 
- data pre processign.


## Limits

- Pyo3 overhead 
- only single thread optimized 
- model isnt absolute
- user made extractors depends heavily on the user .


## License

MIT


## Author

Dipanjan Dutta


## Version

>3.2.1
