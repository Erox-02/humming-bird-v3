```json
{
    "name":        "PAN_India",
    "entity_type": "PAN",
    "pattern":     "[A-Z]{5}[0-9]{4}[A-Z]",
    "confidence":  0.98
}
```

put it in the python code itself

```python
from hbp100 import HBP100

engine = HBP100()

engine.add_extractor("""
{
    "name": "PAN_India",
    "entity_type": "PAN",
    "pattern": "[A-Z]{5}[0-9]{4}[A-Z]",
    "confidence": 0.98
}
""")
```

or 

```python
engine.add_extractor_from_file("./ext.json")
```

import from file , both are avalible .