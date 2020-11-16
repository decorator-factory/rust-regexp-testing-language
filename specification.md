# Language specification

```js
Document = { string: [Spec, ...] }

Spec = Detects | DoesNotDetect | InsideFinds | ReplacesWith

Detects = {
    "it": string,
    "detects": [string, ...]
}

DoesNotDetect = {
    "it": string,
    "doesNotDetect": [string, ...]
}

InsideFinds = {
    "it": string,
    "insideFinds": { string: Match }
}

ReplacesWith = {
    "it": string,
    "given": string,
    "replaces": [[string, string], ...]
}

Match = { string: string }
```

# Example
```json
{
    "signed_integer": [
        {
            "it": "Detects signed decimal integers",
            "detects": ["42", "-5", "0", "-0", "+7"]
        },
        {
            "it": "Does not detect non-decimal integers or floats",
            "doesNotDetect": ["3.14", "0.0", "0,0", "0xf00d"]
        }
    ],
    "integer_pair": [
        {
            "it": "Detects a comma-separated parenthesized pair of integers",
            "detects": ["(1,2)", "( -3, +5  )"]
        },
        {
            "it": "Extracts the integers from the pair",
            "insideFinds": {
                "(3, -7)": { "1": "3", "2": "-7" },
                "(  +2, 5  )": { "1": "+2", "2" : "5" }
            }
        }
    ]
}
```