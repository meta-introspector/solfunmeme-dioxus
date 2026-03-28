# Orphan Submodules Removed (2026-03-28)

Three gitlink entries were tracked in the git index without corresponding URLs in `.gitmodules`. They were added in commit `b5df7d9c` (2025-07-07, "wip") during a Cursor session and never had actual repos created.

## Removed Entries

| Path | Origin | Concept |
|------|--------|---------|
| `social_media_revolution` | Grok chat (founding_documents/vectors/grok-chat (3).md) | SocialMediaParsing (prime 367) — parse meme trends from X/Discord via vendor_scraper |
| `crates/task_manager` | Cursor chat (founding_documents/chat/cursor-chat.md:17328) | Turtle/RDF task manager with Sophia integration |
| `vendor/time` | Unknown | Likely time crate vendoring attempt |

## How to Reconstruct

The design specs live in the chat logs:

- **social_media_revolution**: Search `grok-chat (3).md` for `social_media_parsing` — contains the Concept Matrix entry, prime assignments, emoji sequences, and ZOS feature mapping.
- **task_manager**: Search `cursor-chat.md` for `task_manager` (line ~17328) — contains the Turtle/RDF migration plan, Sophia integration spec, and CLI interface design.

## Lesson

When creating new submodules locally:
1. Always create the repo first (`git init` or `gh repo create`)
2. Add URL to `.gitmodules` before committing
3. Register in `~/git/github.com/meta-introspector/<name>/impl/` per our convention
