# 🐾 TRACKR

> stay pawsitive and organize your tasks like a pro ✨

a cute, fun, and actually useful CLI task tracker built with Rust 🦀

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
- 💬 **motivational quotes** - 50 Gen Z slang quotes to keep you going
- 🚀 **zero dependencies** - pure Rust stdlib only 
- 📝 **simple JSON storage** - human-readable data

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
# 💯 No cap, you're crushing it today!
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
trackr mark 1 in-progress  
trackr mark 1 done        
trackr mark 1 todo        
# ⚡ Task marked as in-progress! Keep going!
```

### delete a task
```bash
trackr delete 2
# 🗑️  Task deleted! Bye bye task #2
```

### reset all tasks
```bash
trackr reset
# 🧹 All tasks cleared! Fresh start, bestie!
```

## 🎨 task statuses

| status | emoji | meaning |
|--------|-------|---------|
| `todo` | 📝 | haven't started yet |
| `in-progress` | ⚡ | working on it rn |
| `done` | ✨ | completed, period |

## 🎭 messages you'll see

- `😸 Task added successfully, slay!`
- `✨ Task updated, you're killing it!`
- `🗑️  Task deleted! Bye bye task #X`
- `🧹 All tasks cleared! Fresh start, bestie!`
- `😿 Task not found, meow again!`
- `🐾 Listing your vibes (tasks)...`
- `⚡ Task marked as in-progress! Keep going!`
- `💯 No cap, you're crushing it today!` (random motivational quote on list)

## 💾 where's my data?

all your tasks are saved in `tasks.json` in the same directory where you run trackr. it's just JSON, so you can edit it manually if you want (but why would you when trackr is this cute?)


## 🧪 testing

```bash
# run all tests
cargo test

# run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Stdout
```

## 🏗️ architecture

```
trackr/
├── src/
│   ├── main.rs       → entry point, CLI parsing, cute cat banner
│   ├── lib.rs        → library exports
│   ├── task.rs       → Task struct & status logic
│   ├── storage.rs    → JSON read/write with pure stdlib
│   ├── commands.rs   → all command implementations
│   └── quotes.rs     → 50 motivational Gen Z quotes
├── tests/            → integration tests
└── Cargo.toml        → minimal dependencies (rand only)
```

## 🤝 contributing

found a bug? want to add features? PRs are welcome! just make sure:
- ✅ tests pass (`cargo test`)
- ✅ code compiles (`cargo build`)
- ✅ vibes are immaculate
---

**stay pawsitive! 😸🐾**

