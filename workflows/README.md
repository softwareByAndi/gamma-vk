# Workflow Usage Guide

This document provides practical examples and guidance for using the Gamma-VK workflow system effectively.

## Overview

The workflow system consists of four core workflows designed to ensure consistent, high-quality development:

1. **[session_start.workflow](session_start.workflow)** - Comprehensive session initialization
2. **[iteration_review.workflow](iteration_review.workflow)** - Mid-iteration health checks
3. **[task_planning.workflow](task_planning.workflow)** - Complex task breakdown and planning
4. **[quality_gate.workflow](quality_gate.workflow)** - Pre-commit quality validation

## When to Use Each Workflow

### Session Start Workflow
**Use when**: Beginning any development session
**Frequency**: Every development session (daily)
**Time**: 5-10 minutes

**Example scenarios**:
- Starting work on a new iteration
- Returning to development after a break
- Beginning work on a specific task
- Joining the project for the first time

### Iteration Review Workflow
**Use when**: Checking iteration health and progress
**Frequency**: Mid-iteration (typically after 3-5 days of work)
**Time**: 10-15 minutes

**Example scenarios**:
- Halfway through a 1-2 week iteration
- When feeling uncertain about progress
- Before major scope decisions
- When encountering unexpected complexity

### Task Planning Workflow
**Use when**: Breaking down complex tasks
**Frequency**: As needed for non-trivial tasks
**Time**: 15-30 minutes

**Example scenarios**:
- Implementing a new major feature
- Refactoring significant code sections
- Adding complex new APIs
- Tasks that will take more than 2-3 hours

### Quality Gate Workflow
**Use when**: Before committing code changes
**Frequency**: Before every commit
**Time**: 5-10 minutes

**Example scenarios**:
- Completing any development task
- Before creating pull requests
- After fixing bugs or issues
- Before iteration completion

## Practical Usage Examples

### Example 1: Starting Iteration 2 Development

**Context**: Beginning Buffer Management implementation for Iteration 2

**Workflow sequence**:
1. **Session Start** → Check project health, verify Iteration 1 completion
2. **Task Planning** → Break down Buffer Management into subtasks
3. **Quality Gate** → Validate each subtask before commit
4. **Iteration Review** → Mid-iteration check on progress

```bash
# Follow session_start.workflow procedures
# 1. Check TODO.md for current iteration status
# 2. Run cargo run --example hello_world
# 3. Run cargo test
# 4. Run cargo clippy && cargo fmt --check
# 5. Create session log: workflow_sessions/2_buffer_management_start.log

# Follow task_planning.workflow for Buffer Management
# 1. Break down into: Buffer struct, RAII cleanup, type-safe creation, error handling
# 2. Define success criteria for each subtask
# 3. Estimate time for each component
# 4. Update TODO.md with detailed tasks
```

### Example 2: Mid-Iteration Health Check

**Context**: 4 days into Iteration 2, completed Buffer and Shader modules

**Workflow sequence**:
1. **Iteration Review** → Assess progress and identify risks

```bash
# Follow iteration_review.workflow procedures
# 1. Check completed tasks: Buffer ✅, Shader ✅, Pipeline ❌, Renderer ❌, Example ❌
# 2. Velocity assessment: 2/5 major tasks complete in 4/10 days = good pace
# 3. Quality check: Run full test suite, verify examples still work
# 4. Risk assessment: Pipeline creation might be more complex than planned
# 5. Decision: Keep current scope, add extra day buffer for Pipeline complexity
```

### Example 3: Complex Feature Implementation

**Context**: Adding texture system with multiple image formats and sampling options

**Workflow sequence**:
1. **Task Planning** → Comprehensive breakdown of texture system
2. **Session Start** → Initialize each development session
3. **Quality Gate** → Validate each component
4. **Iteration Review** → Check if complexity is manageable

```bash
# Follow task_planning.workflow procedures
# 1. Requirements: Load PNG/JPEG, create samplers, integrate with shaders
# 2. Decomposition: 
#    - Image loading (2 hours)
#    - Texture creation (3 hours)  
#    - Sampler management (2 hours)
#    - Shader integration (4 hours)
#    - Example implementation (2 hours)
# 3. Success criteria: Textured quad renders correctly, memory managed properly
# 4. Risk: Image format complexity might require additional time
```

### Example 4: Pre-Commit Validation

**Context**: Completed Buffer struct implementation, ready to commit

**Workflow sequence**:
1. **Quality Gate** → Comprehensive validation before commit

```bash
# Follow quality_gate.workflow procedures
# 1. Code quality: cargo fmt --check ✅, cargo clippy ✅
# 2. Tests: cargo test ✅ (added 3 new unit tests for Buffer)
# 3. Documentation: Added rustdoc comments for all public APIs ✅
# 4. Examples: hello_world still works ✅
# 5. Integration: No breaking changes to existing code ✅
# 6. Commit with confidence: "Add Buffer struct with RAII resource management"
```

## Workflow Integration Patterns

### Daily Development Cycle
```
Morning:
1. Session Start Workflow → Assess project state, plan day's work

During Development:
2. Task Planning Workflow → (As needed for complex tasks)
3. Quality Gate Workflow → Before each commit

End of Day:
4. Quality Gate Workflow → Final validation
5. Update session log with progress and insights
```

### Iteration Development Cycle
```
Iteration Start:
1. Session Start Workflow → Comprehensive iteration kickoff
2. Task Planning Workflow → Break down iteration goals

Mid-Iteration:
3. Iteration Review Workflow → Health check and course correction

Iteration End:
4. Quality Gate Workflow → Final validation
5. Iteration Review Workflow → Retrospective and planning
```

## Session Logging Best Practices

### Log Naming Convention
Use format: `{i}_{task_name}.log` where:
- `i` = Sequential number (higher = more recent)
- `task_name` = Descriptive task identifier

**Examples**:
- `1_workflow_system_implementation.log`
- `2_buffer_management_start.log`
- `3_iteration_2_midpoint_review.log`
- `4_shader_system_completion.log`

### Log Content Structure
Each session log should include:
- **Objective**: What was the session's main goal
- **Current State Analysis**: Project health assessment
- **Actions Taken**: What was accomplished
- **Key Decisions**: Important architectural or process decisions
- **Next Steps**: What should happen in the next session
- **Session Conclusion**: Overall assessment and confidence level

## Common Workflow Scenarios

### Scenario: Discovering Unexpected Complexity
**Trigger**: Task is taking much longer than estimated
**Response**: 
1. **Iteration Review Workflow** → Assess impact on iteration goals
2. **Task Planning Workflow** → Re-break down the complex task
3. **Decision**: Reduce scope or extend timeline

### Scenario: Quality Issues Found
**Trigger**: Quality Gate workflow finds issues
**Response**:
1. **Fix immediately** → Never compromise on quality
2. **Root cause analysis** → Why did issues occur?
3. **Process improvement** → Update workflows if needed

### Scenario: Iteration Behind Schedule
**Trigger**: Iteration Review shows significant delays
**Response**:
1. **Priority re-evaluation** → Move nice-to-haves to future iterations
2. **Scope reduction** → Focus on MVP features only
3. **Update TODO.md** → Reflect revised plans

## Integration with Existing Tools

### TODO.md Integration
- Session Start → Review TODO.md current status
- Task Planning → Update TODO.md with detailed subtasks
- Iteration Review → Adjust TODO.md priorities and scope

### Git Integration
- Quality Gate → Always run before commits
- Session logging → Document decisions for future reference
- Branch management → Create feature branches for major tasks

### Documentation Integration
- All workflows → Keep docs current with implementation
- Quality Gate → Verify documentation completeness
- Session logs → Capture architectural decisions

## Customization Guidelines

### Adapting Workflows
- **Time constraints**: Adjust checklists based on available time
- **Project phase**: Emphasize different aspects based on project maturity
- **Team size**: Scale procedures for individual vs team development

### Adding Custom Workflows
When creating new workflows:
1. **Clear objective** → Define specific purpose
2. **Structured procedure** → Break into logical steps
3. **Time estimate** → Provide realistic time investment
4. **Success criteria** → Define measurable outcomes
5. **Integration points** → Show how it connects to existing workflows

## Troubleshooting

### Common Issues
- **Workflow feels too heavy**: Start with shorter checklists, expand as needed
- **Inconsistent usage**: Set up reminders or integrate with development tools
- **Quality issues slip through**: Strengthen Quality Gate procedures
- **Poor time estimates**: Track actual time and improve estimation

### Warning Signs
- Skipping workflows due to time pressure
- Quality Gate failures becoming common
- Session logs not being maintained
- Iteration reviews revealing consistent surprises

## Success Metrics

### Individual Session Level
- All workflow checklists completed
- Session objectives achieved
- Quality standards maintained
- Clear next steps identified

### Iteration Level
- Iteration goals met on schedule
- Quality standards consistently maintained
- Minimal technical debt accumulation
- Predictable development velocity

### Project Level
- Consistent code quality across all iterations
- Architectural decisions well-documented
- Development process continuously improving
- Team confidence in delivery capabilities

---

**Remember**: Workflows are tools to enhance development quality and consistency. They should feel helpful, not burdensome. Adjust and customize based on your project's specific needs and constraints.