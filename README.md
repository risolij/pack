## pack

**pack** is a lightweight command-line tool for managing project scaffolds and reusable snippets.

It helps you quickly **see what you have**, **save what you like**, and **use what you need** — all from your terminal.

## ✨ Overview

`pack` combines two common workflows:

* 🏗️ **Project scaffolding** — bootstrap new projects instantly
* 📌 **Snippet management** — store and reuse useful code or text

## 🚀 Installation

```bash
cargo install pack
```

*or clone and build manually:*

```bash
git clone <repo-url>
cd pack
cargo build --release
```
---

## 🧠 Core Concepts

* **dump** → dump all gear
* **stash** → stash gear
* **fish** → fish for gear
* **ditch** → ditch gear

## 📚 Commands

### pack dump
```bash
pack dump
```
List everything available to you.

### pack stash

```bash
pack stash -f src/main.rs -n "gear 1"
```

```bash
pack stash -s "one two three" -n "gear 1"
```

### pack ditch
```bash
pack ditch -n "gear 1"
```

### pack fish
```bash
pack fish -n "gear 1"
```


## 🔮 Future Ideas

* Tagging and search
* Remote sync
* Template variables for scaffolds
* Editor integrations

## 🤝 Contributing
Contributions, ideas, and feedback are welcome!
