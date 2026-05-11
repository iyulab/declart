# Declart Grammar Principles

These principles govern every declaration file format in Declart, across all diagram kinds and all implementations.

1. **Design exclusion.** Declaration files contain no visual information: no colors, coordinates, fonts, sizes, margins, or layout hints. These are the engine's domain.

2. **Semantics only.** Fields express meaning, not appearance. `emphasis: primary` is allowed; `color: red` is not. The engine decides how emphasis looks.

3. **LLM-predictable.** Field names are single English words. Structure is flat or exactly one level deep (`[[items]]`). A language model must be able to generate a correct declaration without examples.

4. **Minimal and concise.** Each kind defines only its required fields. Optional fields are added only when proven necessary across multiple use cases. When in doubt, leave it out.

5. **One representation per kind.** There is exactly one way to express each diagram. No aliases, no alternate structures, no flags that change interpretation. Two valid declarations that express the same diagram must be byte-for-byte identical except for whitespace.
