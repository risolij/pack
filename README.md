# pack

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

* **dump** → What you have
* **stash** → What you save
* **reach** → What you use

## 📚 Commands

### `pack owns`

List everything available to you.

```bash
pack owns
```

Displays:

* Available project scaffolds
* Saved snippets

---

### `pack keeps`

Save something new.

```bash
pack keeps TBD
```

Use this command to:

* Save a snippet
* Store a reusable project scaffold

---

### `pack wants`

Use what you've saved.

```bash
pack wants my-snippet
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
