# 🐾 TRACKR

> stay pawsitive and organize your tasks like a pro ✨

a cute, fun, and actually useful CLI task tracker built with Rust 🦀

**📌 Built for:** [roadmap.sh Task Tracker Challenge](https://roadmap.sh/projects/task-tracker)

```
           /\_/\  
          ( o.o ) 
           > ^ <  

    ████████╗██████╗  █████╗  ██████╗██╗  ██╗██████╗ 
    ╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██╔══██╗
       ██║   ██████╔╝███████║██║     █████╔╝ ██████╔╝
       ██║   ██╔══██╗██╔══██║██║     ██╔═██╗ ██╔══██╗
       ██║   ██║  ██║██║  ██║╚██████╗██║  ██╗██║  ██║
       ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝
```

## ✨ why trackr?

- 😸 **cute af** - comes with an adorable ASCII cat
- ⚡ **super fast** - written in Rust, blazingly fast
- 🎨 **colorful vibes** - pink, purple, cyan aesthetics
- 🚀 **zero dependencies** - pure Rust stdlib only
- 📝 **simple JSON storage** - human-readable data
- 💯 **gen z energy** - messages that actually slap

## 🚀 installation

```bash
# clone this bad boy
git clone <your-repo-url>
cd trackr

# install globally
cargo install --path .

# now you can use it anywhere!
trackr --help
```

## 📖 how to use

### add a task
```bash
trackr add "finish that assignment"
# 😸 Task added successfully, slay!
```

### list your tasks
```bash
trackr list
# 🐾 Listing your vibes (tasks)...
```

### filter by status
```bash
trackr list todo
trackr list in-progress
trackr list done
```

### update a task
```bash
trackr update 1 "actually finish that assignment fr fr"
# ✨ Task updated, you're killing it!
```

### mark task status
```bash
trackr mark 1 in-progress  # you're working on it!
trackr mark 1 done         # yassss finished!
trackr mark 1 todo         # back to the grind
# ⚡ Task marked as in-progress! Keep going!
```

### delete a task
```bash
trackr delete 2
# 🗑️  Task deleted! Bye bye task #2
```

## 🎨 task statuses

| status | emoji | meaning |
|--------|-------|---------|
| `todo` | 📝 | haven't started yet |
| `in-progress` | ⚡ | working on it rn |
| `done` | ✨ | completed, period |

## 💾 where's my data?

all your tasks are saved in `tasks.json` in the same directory where you run trackr. it's just JSON, so you can edit it manually if you want (but why would you when trackr is this cute?)

## 🧪 testing

we've got **83 comprehensive tests** with **74.87% code coverage**! 

```bash
# run all tests
cargo test

# run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Stdout
```

### test files:
- ✅ `tests/task_tests.rs` - 14 tests
- ✅ `tests/storage_tests.rs` - 12 tests  
- ✅ `tests/storage_edge_cases_tests.rs` - 13 tests
- ✅ `tests/storage_comprehensive_tests.rs` - 12 tests
- ✅ `tests/commands_tests.rs` - 20 tests
- ✅ `tests/commands_edge_cases_tests.rs` - 12 tests

## 🏗️ architecture

```
trackr/
├── src/
│   ├── main.rs       → entry point, CLI parsing, cute cat banner
│   ├── lib.rs        → library exports
│   ├── task.rs       → Task struct & status logic (100% coverage!)
│   ├── storage.rs    → JSON read/write with pure stdlib
│   └── commands.rs   → all command implementations
├── tests/            → 83 integration tests
└── Cargo.toml        → no external dependencies!
```

**clean architecture vibes:**
- 🎯 separation of concerns
- ✨ zero external dependencies
- 🧪 comprehensive test coverage
- 🚀 production-ready code

## 🎭 messages you'll see

- `😸 Task added successfully, slay!`
- `✨ Task updated, you're killing it!`
- `🗑️  Task deleted! Bye bye task #X`
- `😿 Task not found, meow again!`
- `🐾 Listing your vibes (tasks)...`
- `⚡ Task marked as in-progress! Keep going!`

## 🤝 contributing

found a bug? want to add features? PRs are welcome! just make sure:
- ✅ tests pass (`cargo test`)
- ✅ code compiles (`cargo build`)
- ✅ vibes are immaculate
---

**stay pawsitive! 😸🐾**

