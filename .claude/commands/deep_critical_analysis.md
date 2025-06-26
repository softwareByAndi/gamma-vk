# Critical Analysis Workflow

## Objective
Perform deep critical analysis of specific tasks or implementations with the perspective of an experienced staff engineer, identifying potential issues, improvements, and architectural considerations.

## Usage Pattern
This workflow is designed for iterative analysis:
1. First round: Initial critical analysis
2. Second round: Deeper analysis after initial findings
3. Follow-up: Implementation or TODO.md updates

## Procedure

### Phase 1: Initial Critical Analysis

**Prompt Template:**
```
analyze {task} with the critical eye of an experienced staff engineer
```

**Analysis Framework:**
- [ ] **Architectural Impact** - How does this task affect overall system design?
- [ ] **Performance Implications** - What are the performance characteristics and bottlenecks?
- [ ] **Security Considerations** - Are there any security implications or vulnerabilities?
- [ ] **Maintainability** - How will this affect long-term maintenance and evolution?
- [ ] **Testing Strategy** - What testing approach is needed for confidence?
- [ ] **Error Handling** - How robust is the error handling and recovery?
- [ ] **Dependencies** - What dependencies are introduced and are they justified?
- [ ] **Complexity Assessment** - Is the complexity appropriate for the problem being solved?

### Phase 2: Iterative Deepening

**Prompt Template:**
```
in the spirit of iterative development, again, analyze {task} with the critical eye of an experienced staff engineer
```

**Deeper Analysis Focus:**
- [ ] **Alternative Approaches** - What other solutions were considered and why was this chosen?
- [ ] **Technical Debt** - What technical debt is being introduced or resolved?
- [ ] **Scalability** - How will this scale with increased load or complexity?
- [ ] **Integration Points** - How does this interact with existing systems and future components?
- [ ] **Resource Management** - Are resources being managed efficiently and safely?
- [ ] **Documentation Impact** - What documentation needs to be updated or created?
- [ ] **Team Knowledge** - What knowledge gaps might this create for the team?
- [ ] **Monitoring/Observability** - How will we detect issues in production?

### Phase 3: Action Planning

**Decision Points:**
- [ ] **Immediate Implementation** - Should fixes/improvements be implemented now?
- [ ] **TODO.md Updates** - What items should be added to the backlog?
- [ ] **Documentation Needs** - What documentation requires updates?
- [ ] **Testing Requirements** - What additional tests are needed?

**Documentation:**
- [ ] **Session Log** - Record key insights in session_logs/{i}_critical_analysis_{task}.log
- [ ] **TODO.md Updates** - Add actionable items discovered during analysis
- [ ] **Architecture Notes** - Update design docs if architectural decisions were made

## Output Templates

### Critical Analysis Report Structure
```
## Critical Analysis: {Task Name}
### Date: {Date}
### Scope: {Brief description}

### Key Findings
- [ ] **Positive Aspects**: What's working well
- [ ] **Concerns**: Areas of concern or risk
- [ ] **Recommendations**: Specific actionable recommendations

### Architectural Impact
- [ ] **System Design**: How this affects overall architecture
- [ ] **Performance**: Performance implications and optimizations
- [ ] **Security**: Security considerations and mitigations

### Action Items
- [ ] **Immediate**: Tasks to address now
- [ ] **Short-term**: Items for current iteration
- [ ] **Long-term**: Strategic items for future iterations
```

### Session Log Template
```
# Critical Analysis Session Log

## Session: {i}_critical_analysis_{task}
## Date: {Date}
## Duration: {Duration}

## Analysis Summary
{High-level summary of findings}

## Key Insights
{Most important discoveries}

## Architectural Decisions
{Any architectural decisions made}

## Action Items
- [ ] Immediate: {items}
- [ ] TODO.md: {items}
- [ ] Documentation: {items}

## Next Steps
{What should happen next}
```

## Best Practices

### When to Use This Workflow
- [ ] **Before major implementation** - Validate approach before significant work
- [ ] **Code review scenarios** - Deep analysis of proposed changes
- [ ] **Architectural decisions** - Evaluate system design choices
- [ ] **Performance optimization** - Analyze performance-critical code paths
- [ ] **Security review** - Examine security-sensitive implementations

### Quality Standards
- [ ] **Objectivity** - Maintain professional objectivity, not just criticism
- [ ] **Actionability** - Ensure recommendations are specific and actionable
- [ ] **Prioritization** - Clearly indicate which issues are most critical
- [ ] **Context Awareness** - Consider project constraints and timeline realities

### Follow-up Protocols
- [ ] **Implementation Sessions** - Schedule focused sessions for critical fixes
- [ ] **TODO.md Maintenance** - Regularly review and prioritize discovered items
- [ ] **Documentation Updates** - Ensure insights are captured in appropriate docs
- [ ] **Team Communication** - Share critical findings with team members

## Integration with Other Workflows

### Combines Well With:
- **session_start.workflow** - Use for initial session analysis
- **task_planning.workflow** - Use for complex task breakdown
- **iteration_review.workflow** - Use for mid-iteration health checks
- **quality_gate.workflow** - Use for pre-commit critical validation

### Workflow Sequencing:
1. Start with **session_start.workflow**
2. Use **critical_analysis.workflow** for deep dives
3. Follow with **task_planning.workflow** for implementation planning
4. End with **quality_gate.workflow** before commits