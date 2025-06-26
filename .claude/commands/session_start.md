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
- [ ] **Check CLAUDE.md** - Verify development commands and methodology alignment
- [ ] **Assess Documentation Drift** - Identify any gaps between docs and current implementation

### 3. Session Context
- [ ] **Check TODO.md** - Review TODO.md current iteration status, completed tasks, and next priorities
- [ ] **Review Current Session** - if TODO.md includes a file for the current session Log, then pull it into context
- [ ] **Identify Planned Work** - if `PAUSED.md` exists, review it to see if any work can be resumed

### 4. Critical Analysis (use the critical eye of an experienced staff engineer)
- [ ] **Schedule Reality Check** - Analyze if current TODO.md tasks are realistic for timeline
- [ ] **Architectural Consistency** - Verify planned work aligns with design principles
- [ ] **Technical Debt Assessment** - Identify any technical debt that could impact current iteration
- [ ] **Risk Identification** - Spot potential blockers or complexity that could derail progress

### 5. Session Planning
- [ ] **Prioritize Work** - Confirm next task from TODO.md or identify urgent work needed
- [ ] **Scope Validation** - Ensure selected work fits within session time constraints
- [ ] **Resource Check** - Verify all necessary tools/dependencies are available
- [ ] **Create Session Log** - Document findings and decisions in session_logs/{i}_{current_task_name}.log.md 
    - (name this session log appropriate to the selected task) 
    - (also, current session i should be > than the previous session i)

### 6. Decision Points
- [ ] **Schedule Adjustment Needed?** - Determine if TODO.md requires updates based on analysis
- [ ] **Architecture Concerns?** - Flag any design decisions that need addressing
- [ ] **Quality Issues?** - Identify immediate quality problems requiring attention
- [ ] **Blocking Issues?** - Document any blockers that need resolution

## Expected Outcomes
- Clear understanding of current project state
- Validated next steps with realistic scope
- Documented insights and decisions
- Confidence in development direction

## Time Estimate
5-10 minutes (investment pays dividends in session efficiency)

## Success Criteria
- [ ] All procedure items completed
- [ ] Session log created with key insights
- [ ] Next development task clearly identified
- [ ] No critical issues overlooked
