# Cold-start Friction V1

Temporary notes for the first zero-mount agent task run.

## Observed Friction Points

1. `install.sh` is naturally attempted with `sh`, but currently requires `bash` because it uses `set -o pipefail`.
2. After install succeeds, the agent still lacks a working authenticated path to clone the private `powerformer/consensus` repository.
3. The task container does not yet inject the minimum `COWORK_*` environment needed for a natural cowork-first path.
4. `cowork clone --help` still describes `preview` as an execution command, which conflicts with the new guide-only semantics.

## Why These Matter

- They create avoidable cold-start ambiguity before the real task begins.
- They blur whether the blocker is product design, environment setup, or repository access.
- They prevent the task from reaching the intended draft-PR validation stage.

## Next Iteration Focus

1. Make the shell expectation around `install.sh` explicit or resilient.
2. Provide a minimal authenticated repo access path.
3. Inject the smallest viable cowork environment contract.
4. Align all preview-related help text with guide-only behavior.
