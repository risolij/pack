# mark

**mark** is a lightweight command-line tool for managing project scaffolds and reusable snippets.

It helps you quickly **see what you have**, **save what you like**, and **use what you need** — all from your terminal.

## ✨ Overview

`mark` combines two common workflows:

* 🏗️ **Project scaffolding** — bootstrap new projects instantly
* 📌 **Snippet management** — store and reuse useful code or text

## 🚀 Installation

```bash
cargo install mark
```

*or clone and build manually:*

```bash
git clone <repo-url>
cd mark
cargo build --release
```

---

## 🧠 Core Concepts

* **Owns** → What you have
* **Keeps** → What you save
* **Wants** → What you use

## 📚 Commands

### `mark owns`

List everything available to you.

```bash
mark owns
```

Displays:

* Available project scaffolds
* Saved snippets

---

### `mark keeps`

Save something new.

```bash
mark keeps TBD
```

Use this command to:

* Save a snippet
* Store a reusable project scaffold

---

### `mark wants`

Use what you've saved.

```bash
mark wants my-snippet
```

Use this command to:

* Apply a scaffold to a new project
* Output or insert a snippet

---

## 🔧 Examples
TBD

## 📁 Suggested Structure

`mark` may store data in a directory like:

```
~/.mark/
  scaffolds/
  snippets/
```

## 🔮 Future Ideas

* Tagging and search
* Remote sync
* Template variables for scaffolds
* Editor integrations

## 🤝 Contributing
Contributions, ideas, and feedback are welcome!

