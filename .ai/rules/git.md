# Git usage guidelines

## Commit message convention

- **GIT-C-1 (MUST)** When committing, always include a body explaining:
  - Why this change was made
  - What problem it solves or risk it removes
  - Any tradeoffs or constraints that shaped the approach
- **GIT-C-2 (MUST)** Follow the format defined:
```
type(scope): short summary of what changed
Reasoning here - what was the previous behavior, why was it a
problem, and what should a future engineer know before touching this.
Closes #<issue>
```
