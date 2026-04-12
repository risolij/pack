# pocket

**pocket** is a lightweight command-line tool for managing project scaffolds and reusable snippets.

It helps you quickly **see what you have**, **save what you like**, and **use what you need** — all from your terminal.

## ✨ Overview

`pocket` combines two common workflows:

* 🏗️ **Project scaffolding** — bootstrap new projects instantly
* 📌 **Snippet management** — store and reuse useful code or text

## 🚀 Installation

```bash
cargo install pocket
```

*or clone and build manually:*

```bash
git clone <repo-url>
cd pocket
cargo build --release
```
---

## 🧠 Core Concepts

* **dump** → What you have
* **stash** → What you save
* **reach** → What you use

## 📚 Commands

### `pocket owns`

List everything available to you.

```bash
pocket owns
```

Displays:

* Available project scaffolds
* Saved snippets

---

### `pocket keeps`

Save something new.

```bash
pocket keeps TBD
```

Use this command to:

* Save a snippet
* Store a reusable project scaffold

---

### `pocket wants`

Use what you've saved.

```bash
pocket wants my-snippet
```

Use this command to:

* Apply a scaffold to a new project
* Output or insert a snippet

---

## 🔧 Examples
TBD

## 📁 Suggested Structure
TDB

## 🔮 Future Ideas

* Tagging and search
* Remote sync
* Template variables for scaffolds
* Editor integrations

## 🤝 Contributing
Contributions, ideas, and feedback are welcome!
