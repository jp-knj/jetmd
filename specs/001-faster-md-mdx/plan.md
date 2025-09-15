# Implementation Plan: faster-md & mdx

**Branch**: `001-faster-md-mdx` | **Date**: 2025-09-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-faster-md-mdx/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
4. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
5. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, or `GEMINI.md` for Gemini CLI).
6. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
7. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
8. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
High-performance Markdown core (Rust) with MDX compiler layer. WASM-first architecture targeting 200 MB/s native and 50 MB/s WASM throughput. CommonMark 0.30 compliant with opt-in GFM extensions. No environment variables - all configuration via options. Monorepo structure with separate packages for core, WASM bindings, Node wrapper, and MDX compiler.

## Technical Context
**Language/Version**: Rust (latest stable) for core, TypeScript/JavaScript for Node packages  
**Primary Dependencies**: wasm-bindgen, serde, nom/winnow (parser), acorn (JS parser for MDX)  
**Storage**: N/A (stateless processor)  
**Testing**: cargo test (Rust), vitest (JS packages), CommonMark/GFM conformance suites  
**Target Platform**: WASM (primary), Native (secondary via future Neon binding)
**Project Type**: single (monorepo with multiple packages)  
**Performance Goals**: ≥200 MB/s native, ≥50 MB/s WASM, <3ms p50 for 50KB docs  
**Constraints**: Memory ≤1.5× input (typical), ≤3× (worst case), zero runtime dependencies  
**Scale/Scope**: Core ~10k LOC Rust, bindings ~2k LOC, full CommonMark + GFM + MDX support

**User-Provided Technical Details**:
- WASM-first with wasm-bindgen (no napi-rs)
- Future native support via Neon (N-API) as separate path
- No ENV variables - all config via options
- Node ≥20 with ESM support
- Monorepo with pnpm workspaces
- Incremental parsing with ≥90% node reuse
- Streaming output support
- Plugin system for syntax/transform/render hooks
- Security by default (sanitization enabled, raw HTML disabled)
- MDX compiles to ESM modules with JSX, no runtime execution during compilation

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Simplicity**:
- Projects: 2 (crates for Rust, packages for Node) ✓
- Using framework directly? Yes - direct wasm-bindgen, no wrappers ✓
- Single data model? Yes - FMDAST for all operations ✓
- Avoiding patterns? Yes - functional approach, no unnecessary abstractions ✓

**Architecture**:
- EVERY feature as library? Yes - all functionality in separate crates/packages ✓
- Libraries listed:
  - fmd-core: Lexer, parser, AST operations
  - fmd-gfm: GFM extension support
  - fmd-html: HTML rendering with sanitization
  - fmd-slug: Stable slug generation
  - fmd-cli: Rust command-line interface
  - fmd-wasm: WASM bindings via wasm-bindgen
  - faster-md: Node.js wrapper for WASM
  - mdx-core: MDX token detection
  - mdx-compiler: MDX to ESM transformation
- CLI per library: fmd CLI planned with --help/--version/--format ✓
- Library docs: README.md + API docs planned ✓

**Testing (NON-NEGOTIABLE)**:
- RED-GREEN-Refactor cycle enforced? Yes - conformance tests first ✓
- Git commits show tests before implementation? Will enforce ✓
- Order: Contract→Integration→E2E→Unit strictly followed? Yes ✓
- Real dependencies used? N/A (no external services) ✓
- Integration tests for: CommonMark/GFM conformance, plugin API ✓
- FORBIDDEN: Implementation before test, skipping RED phase ✓

**Observability**:
- Structured logging included? Yes - diagnostic system with severity levels ✓
- Frontend logs → backend? N/A (library, not service) ✓
- Error context sufficient? Yes - line/column/offset in all errors ✓

**Versioning**:
- Version number assigned? 0.1.0 for MVP ✓
- BUILD increments on every change? Via changesets ✓
- Breaking changes handled? Semver + migration guides planned ✓

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure]
```

**Structure Decision**: Monorepo structure (modified Option 1 for multi-package project)
```
crates/           # Rust packages
├── fmd-core/
├── fmd-gfm/
├── fmd-html/
├── fmd-slug/
├── fmd-cli/
└── fmd-wasm/

packages/         # Node packages
├── faster-md/
├── faster-md-cli/
├── mdx-core/
├── mdx-compiler/
└── mdx-runtime/

tests/
├── conformance/  # CommonMark/GFM test suites
├── fixtures/     # Shared test documents
└── benchmarks/   # Performance tests
```

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - ✅ All technical decisions clarified in user specification
   - ✅ No NEEDS CLARIFICATION items remain

2. **Generate and dispatch research agents**:
   - ✅ Researched parser architectures (dual-pass with SIMD)
   - ✅ Evaluated WASM strategies (wasm-bindgen selected)
   - ✅ Analyzed incremental parsing approaches (rope-based)
   - ✅ Determined MDX integration strategy (delegated JS parsing)

3. **Consolidate findings** in `research.md` using format:
   - ✅ Decisions documented with rationales
   - ✅ Alternatives evaluated and rejected with reasons
   - ✅ Risk assessment completed

**Output**: ✅ research.md created with all decisions documented

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - ✅ Core entities defined (Document, FMDAST, ProcessorOptions)
   - ✅ Validation rules specified for all entities
   - ✅ State transitions documented (parsing, incremental, plugin)

2. **Generate API contracts** from functional requirements:
   - ✅ Rust API contract (rust-api.md)
   - ✅ WASM API contract (wasm-api.md)
   - ✅ Node.js API contract (node-api.md)
   - ✅ Contract tests included in each file

3. **Generate contract tests** from contracts:
   - ✅ Parse tests (empty, paragraph, GFM, position tracking)
   - ✅ Render tests (sanitization, dangerous HTML)
   - ✅ Error tests (malformed MDX, diagnostics)
   - ✅ Tests designed to fail initially (TDD approach)

4. **Extract test scenarios** from user stories:
   - ✅ Quickstart guide with 10 usage examples
   - ✅ Verification tests included
   - ✅ Common patterns documented

5. **Update agent file incrementally** (O(1) operation):
   - ✅ Created CLAUDE.md with project context
   - ✅ Tech stack and structure documented
   - ✅ Recent changes tracked
   - ✅ Under 150 lines for efficiency

**Output**: ✅ data-model.md, ✅ /contracts/*, ✅ quickstart.md, ✅ CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Core parser tasks: lexer, block scanner, inline parser [P]
- AST tasks: node types, position tracking, validation [P]
- WASM binding tasks: wasm-bindgen setup, API exports [P]
- Node wrapper tasks: initialization, API surface
- MDX compiler tasks: JSX parsing, ESM generation
- Testing tasks: conformance setup, benchmarks

**Ordering Strategy**:
- TDD order: Tests before implementation
- Dependency order: 
  1. Core Rust crates (fmd-core, fmd-html)
  2. WASM bindings (fmd-wasm)
  3. Node packages (faster-md, mdx-compiler)
  4. Integration tests
- Mark [P] for parallel execution within each layer

**Estimated Output**: 40-50 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none needed)

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*