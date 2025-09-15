# Project Backlog

## Quick Goals (<1 hr) ⚡️
- [ ] **Line Numbers**: Print the line number where the match occurs.
- [ ] **Match count**: Add a flag to only show how many matches were found.
- [ ] **Invert match**: Print all lines that don't contain the search term.
- [ ] **Highlight matches**: Wrap matche sin ANSI escape codes (So they show up in green/red).
- [ ] **Whole word only**: Only match if surrounded by whitespace or punctuation.


## Medium Goals (1-3 hrs) ⏳
- [ ] **Recursive search**: Accept dirs and explore them. Use walkdir crate.
- [ ] **Output modes**: e.g, --json flag to print structured JSON output.
- [ ] **Config file**: Let users define default serach options from config file.
- [ ] **Glob patterns**: Allow *.rs or *.txt to match groups of files.

## Stretch Goals 🚀
- [ ] **Regex Search**: Replace substring search with regex crate
- [ ] **Context Lines**: Show N lines before/after a match
- [ ] **Parallelism**: Use rayon to search multiple files concurrently.
- [ ] **Streaming search**: Instead of loading the whole file into memory, stream line by line (Useful for huge files).
- [ ] **Interactive TUI**: Browse matches inside a terminal UI (ratatui crate).
- [ ] **Organize Library crate**: Make it a suitable and callable interface.



# Current Sprint (Week 1)
_This is after initial project commits that fleshed out the OG design_

## In Progress

## Completed



# Completed Tasks
- [X] **Argument Flags**: Add a CLI flag system instead of relying only on env vars.
- [X] **Help Flag**: Added -h --help flag to output helpful tidbits about running program.
- [X] **Multiple Files**: Accept multiple paths, show results grouped by filename.

