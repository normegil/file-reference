# Dockerfile coding guidelines

These rules ensure maintainability, safety, and developer velocity.
**MUST** rules are enforced by CI; **SHOULD** rules are strongly recommended.

## Documentation Best Practices

- **Dockerfile-D-1 (SHOULD)** Document base images, environment variables (ENV), and ports (EXPOSE).
- **Dockerfile-D-2 (SHOULD)** Add comments explaining the purpose of each build stage.