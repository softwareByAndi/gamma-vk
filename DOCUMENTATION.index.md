# Documentation Index

This index provides a comprehensive map of all documentation in the Gamma-VK project, organized by purpose and use case. Use this guide to quickly find the right documentation for your needs.

## 🚀 Quick Start References

**New to the project?**
1. Start with [CLAUDE.md](CLAUDE.md) for project overview
2. Read [docs/DESIGN_PRINCIPLES.md](docs/DESIGN_PRINCIPLES.md) for core philosophy
3. Check [TODO.md](TODO.md) for current status and roadmap
4. Review [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) for codebase organization

**Starting a development session?**
- Check [TODO.md](TODO.md) for current tasks
- Review relevant debug notes (see trigger patterns below)

## 📚 Documentation by Category

### Architecture & Design
- **[docs/DESIGN_PRINCIPLES.md](docs/DESIGN_PRINCIPLES.md)** - Core principles: Safety, Performance, Extensibility
- **[docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)** - Module organization and file structure
- **[debug/debug_architecture.md](debug/debug_architecture.md)** - Architectural decision rationale
- **[docs/RAII_PATTERN.md](docs/RAII_PATTERN.md)** - Resource management patterns

### Development Process
- **[docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md)** - 2-week iteration methodology
- **[TODO.md](TODO.md)** - Current iteration status and tasks

### Code Standards
- **[docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)** - Naming conventions, formatting, organization
- **[docs/TESTING_PATTERNS.md](docs/TESTING_PATTERNS.md)** - Testing methodology and patterns
- **[CLAUDE.md](CLAUDE.md)** - AI assistant instructions with code examples

### API & Implementation Guidance
- **[debug/debug_api_patterns.md](debug/debug_api_patterns.md)** - API design patterns
- **[debug/debug_vulkano_api.md](debug/debug_vulkano_api.md)** - Vulkano library usage
- **[debug/debug_buffer.md](debug/debug_buffer.md)** - Buffer implementation specifics
- **[debug/debug_rust_types.md](debug/debug_rust_types.md)** - Type system gotchas

### Test Documentation
- **[tests/context.tests.md](tests/context.tests.md)** - VulkanContext test specifications
- **[tests/buffer.tests.md](tests/buffer.tests.md)** - Buffer management test plans

### Examples & Demos
- **[examples/hello_world/](examples/hello_world/)** - Working Vulkan demo application

### Historical Context
- **[session_logs/](session_logs/)** - Development session logs with insights:
  - Bug fixes and their solutions
  - Architectural decisions not documented elsewhere
  - Implementation challenges and resolutions

## 🔍 Documentation by Use Case

### "I need to understand..."

| Topic | Primary Documentation | Additional References |
|-------|----------------------|----------------------|
| Overall architecture | [docs/DESIGN_PRINCIPLES.md](docs/DESIGN_PRINCIPLES.md) | [debug/debug_architecture.md](debug/debug_architecture.md) |
| Module organization | [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) | [CLAUDE.md](CLAUDE.md#module-organization) |
| Resource management | [docs/RAII_PATTERN.md](docs/RAII_PATTERN.md) | [debug/debug_buffer.md](debug/debug_buffer.md) |
| Error handling | [CLAUDE.md](CLAUDE.md#error-handling-strategy) | [debug/debug_api_patterns.md](debug/debug_api_patterns.md) |
| Testing approach | [docs/TESTING_PATTERNS.md](docs/TESTING_PATTERNS.md) | [tests/*.tests.md](tests/) |
| Development workflow | [docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md) | |

### "I'm working on..."

| Task | Key Documentation | Debug Notes |
|------|------------------|-------------|
| Buffers | [tests/buffer.tests.md](tests/buffer.tests.md) | [debug/debug_buffer.md](debug/debug_buffer.md), [debug/debug_vulkano_api.md](debug/debug_vulkano_api.md) |
| Context/Device | [tests/context.tests.md](tests/context.tests.md) | [debug/debug_architecture.md](debug/debug_architecture.md) |
| Shaders | [session_logs/5_shader_system.log.md](session_logs/5_shader_system.log.md) | [debug/debug_api_patterns.md](debug/debug_api_patterns.md) |
| API Design | [debug/debug_api_patterns.md](debug/debug_api_patterns.md) | [session_logs/6_api_consistency.log.md](session_logs/6_api_consistency.log.md) |
| Type Issues | [debug/debug_rust_types.md](debug/debug_rust_types.md) | - |

### "I encountered..."

| Issue | Check These Documents |
|-------|---------------------|
| "Unused parameter" warning | [debug/debug_architecture.md](debug/debug_architecture.md) |
| Type mismatch (u64 vs usize) | [debug/debug_rust_types.md](debug/debug_rust_types.md) |
| Vulkano API confusion | [debug/debug_vulkano_api.md](debug/debug_vulkano_api.md) |
| Buffer creation issues | [debug/debug_buffer.md](debug/debug_buffer.md), [session_logs/4_buffer_test_failure_analysis.log.md](session_logs/4_buffer_test_failure_analysis.log.md) |
| Test failures | [docs/TESTING_PATTERNS.md](docs/TESTING_PATTERNS.md), relevant session logs |

## 📝 Documentation Maintenance

### When to Update Documentation

1. **Debug Notes** - Update when discovering:
   - API gotchas or unexpected behavior
   - Architectural decisions that might seem unclear
   - Type system issues and resolutions
   - Wrong assumptions corrected during development

2. **Session Logs** - Create new logs for:
   - Complex debugging sessions
   - Major architectural decisions
   - Implementation of new systems
   - Significant refactoring efforts

3. **Core Docs** - Update when:
   - Design principles evolve
   - New patterns are established
   - Standards change
   - Project structure is modified

### Documentation Update Checklist
- [ ] Update this index when adding new documentation
- [ ] Cross-reference related documents
- [ ] Keep debug notes focused on insights, not tutorials
- [ ] Ensure session logs capture key decisions
- [ ] Maintain consistency with CLAUDE.md instructions

## 🎯 Quick Reference Commands

```bash
# Find all documentation files
find . -name "*.md" -type f | grep -E "(docs|debug|plans|tests|session_logs)" | sort

# Search documentation for a topic
grep -r "buffer" docs/ debug/ --include="*.md"

# View current tasks
head -50 TODO.md

# Check latest session insights
ls -la session_logs/ | tail -5
```

---

*Last updated: Check git history for latest updates*
*Maintained as part of Gamma-VK project documentation*