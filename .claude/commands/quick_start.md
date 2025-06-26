# Session Start Workflow

## Objective
Perform comprehensive session initialization with staff engineer-level analysis to ensure optimal development focus and catch potential issues early.

## Procedure

### 1. Current State Assessment
- [ ] **Verify Application Health** - Run `cargo run --example hello_world` to confirm baseline functionality
- [ ] **Run Test Suite** - Execute `cargo test` to validate current codebase integrity
- [ ] **Quality Check** - Run `cargo clippy` and `cargo fmt --check` to ensure code quality

### 2. Documentation Review
- [ ] **Review Key Docs** - Read DESIGN_PRINCIPLES.md, PROJECT_STRUCTURE.md, STYLE_GUIDE.md for current architectural understanding

### 4. Session Planning
- [ ] **Prioritize Work** - Confirm next task from TODO.md or identify urgent work needed
- [ ] **Review Current Session** - if TODO.md includes a file for the Current Session Log, then pull it into context
- [ ] **Identify Planned Work** - if `PAUSED.md` exists, review it to see if any work can be resumed
- [ ] **Scope Validation** - Ensure selected work fits within session time constraints
- [ ] **Resource Check** - Verify all necessary tools/dependencies are available
- [ ] **Create Session Log** - Any notes for this session will go into session_logs/{i}_{current_task_name}.log.md 
    - (name this session log appropriate to the selected task) 
    - (also, current session i should be > than the previous session i)

## Success Criteria
- [ ] All procedure items completed
- [ ] Session log created with key insights
- [ ] Next development task clearly identified
- [ ] No critical issues overlooked
