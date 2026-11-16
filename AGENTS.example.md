# AGENTS.md

## What this repo is
[One paragraph: product purpose, main user flows, tech stack.]

## Architecture
- `src/api/` - HTTP layer, no business logic here
- `src/services/` - all domain logic lives here
- `src/db/` - repository pattern, direct DB access only here

## Key constraints
- Auth is handled by `AuthMiddleware` - do not bypass or replicate it
- All external API calls go through `src/clients/` with retry logic

## Conventions
Before any change to the code, read the rules in the `.ai/rules` folder to ensure compliance with the user defined rules.

`docs/task.md`: Describes how to build/lint/test/format the current project

## Do not touch without asking
- `src/legacy/billing/` - scheduled for migration, fragile
- `infra/terraform/prod/` - requires human approval
