[![Crates.io](https://img.shields.io/crates/v/hbp100.svg)](https://crates.io/crates/hbp100)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# hummin-bird

hbp100 is a privacy system that sits between a user and a llm , it finds sensitive
values, swaps them with placeholders , before the text leaves the process,and restores 
the originals after the response comes back. 

---

## What's new

### Config based extractors


    see 

>ex*/syntax.md.

### Sessions


### Eval

| Metric    | Result  |
| --------- | ------: |
| Accuracy  | 91.34%  |
| Precision | 95.57%  |
| Recall    | 84.59%  |
| F1        | 89.75%  |

---

## Install

```bash
cargo add hbp100          
pip install      
```

From source (Rust toolchain + Python 3.8+):

```bash
git clone <repository> && cd humming-bird-v3
maturin build --release
python -m venv env
pip install ./target/wh*/*
```

> always use --release for speed

---

## Py api

```python
engine.process(text, intent=None, session_id=None)
engine.restore(text, session_id=None)
engine.restore_with_metadata(text, metadata)
engine.validate_response(llm_output)     # -> (bool, Optional[str])

engine.add_extractor(json_string)
engine.add_extractor_from_file(path)
engine.enable_extractor(name)            # -> bool
engine.disable_extractor(name)           # -> bool
engine.list_extractors()                 # -> list[str]
engine.list_enabled()                    # -> list[str]
engine.reset_extractors()                # clears all (no defaults)
```

The Python wrapper is *HBP100*

---

## Rust api


```rust

engine.process(text: &str, intent: Option<&str>) -> ProcessResult


engine.restore(text: &str) -> String
engine.restore_with_metadata(text: &str, metadata: HashMap<String, String>) -> String
engine.validate_response(text: &str) -> (bool, Option<String>)


engine.add_config_extractor(json: &str) -> Result<(), String>
engine.add_config_extractor_from_file(path: &str) -> Result<(), String>
engine.enable_extractor(name: &str) -> bool
engine.disable_extractor(name: &str) -> bool
engine.list_extractors() -> Vec<String>
engine.list_enabled_extractors() -> Vec<String>
engine.reset_extractors()
```

`ProcessResult` fields: `original_text`, `masked_text`, `metadata`,
`entities`, `decisions`, `has_pii` — same data as the Python dict.

Sessions from Rust go through `Sesman` (a session manager) with
`Crtses`, `prc_wses`, and `res_wses`; see `dev/tst.rs` for a working
example.

---

## Sessions

```python
engine.process("Patient John Doe, MRN: 123456", session_id="chat")


engine.process("Patient Jane Smith, MRN: 789012", session_id="chat")

engine.restore("[MRN_1] and [MRN_2]", session_id="chat")

```

Counters are per entity type per session. IDs are SHA256-derived.
Sessions live in a `HashMap` in memory; they do not survive process
exit.

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

---

## Performance

Release mode, Intel Core i5-1135G7, 3 extractors, 100,000 iterations × 5
rounds:

```
Mean:       45.2 µs/text
Throughput: 22,116 texts/sec
```

check the dev/test* and check the speed on your own

---

## Applications

- healthcare document processing 
- insurance workflows 
- ocr 
- ai assistants and llm front-ends 
- customer support systems 
- data pre processign.

---

## Limits

- Pyo3 overhead 
- only single thread optimized 
- model isnt absolute
- user made extractors depends heavily on the user .


## License

MIT

---

## Author

Dipanjan Dutta

---

## Version

>3.1.2
