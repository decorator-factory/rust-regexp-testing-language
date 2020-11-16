# Language specification

```js
Document = { string: [Spec, ...] }

Spec = Detects | DoesNotDetect | InsideFinds | ReplacesWith

Detects = {
    "detects": [string, ...]
}

DoesNotDetect = {
    "doesNotDetect": [string, ...]
}

InsideFinds = {
    "insideFinds": { string: Match }
}

ReplacesWith = {
    "given": string,
    "replaces": [[string, string], ...]
}

Match = { string: string }
```

# Example
```json
{
    "signed_integer": [
        {"detects": ["42", "-5", "0", "-0", "+7"]},
        {"doesNotDetect": ["3.14", "0.0", "0,0", "0xf00d"]}
    ],
    "integer_pair": [
        {"detects": ["(1,2)", "( -3, +5  )"]},
        {"insideFinds": {
            "(3, -7)": { "1": "3", "2": "-7" },
            "(  +2, 5  )": { "1": "+2", "2" : "5" }
        }}
    ]
}
```