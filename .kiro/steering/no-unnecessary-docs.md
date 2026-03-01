---
inclusion: auto
---

# No Unnecessary Documentation Files

## Rule: Do NOT Create Documentation Files After Each Task

**CRITICAL:** Do not create markdown documentation files (.md) after completing tasks unless explicitly requested by the user.

### What NOT to Do

❌ Do NOT create files like:
- `*_GUIDE.md`
- `*_SUMMARY.md`
- `*_STATUS.md`
- `*_VERIFICATION.md`
- `TEST_*.md`
- `DEPLOYMENT_*.md`
- `COMPLETION_*.md`
- Any other documentation files after completing work

### When Documentation IS Allowed

✅ Only create documentation when:
1. User explicitly asks for documentation
2. It's part of the project requirements (README, API docs, etc.)
3. User says "document this" or "create a guide"

### What TO Do Instead

After completing tasks:
1. Provide a brief summary in the chat (2-3 sentences max)
2. List what was accomplished
3. State next steps if any
4. That's it - no files!

### Example Good Response

```
✅ Provisioning improvements complete:
- Added input validation and error handling
- Implemented auto-save for all settings
- Enhanced .gitignore for security
- Committed and pushed to both repos

The GUI is running and ready to use.
```

### Example Bad Response (DON'T DO THIS)

```
Let me create a comprehensive guide...
[Creates PROVISIONING_COMPLETION_GUIDE.md]
[Creates DEPLOYMENT_STATUS.md]
[Creates COMMIT_SUMMARY.md]
```

## Rationale

- Documentation files clutter the repository
- Most are never read or maintained
- Information is already in git history
- User can ask for docs if needed
- Wastes time and tokens

## Exceptions

The following are legitimate project files (not "documentation after task"):
- README.md (project overview)
- CONTRIBUTING.md (contribution guidelines)
- CHANGELOG.md (version history)
- API documentation (if part of project)
- User-facing guides (if requested)

## Summary

**Default behavior:** Complete task → Brief chat summary → Done
**No:** Task → Create 5 markdown files → Done
