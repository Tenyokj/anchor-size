# Anchor Size

[![Crates.io](https://img.shields.io/crates/v/anchor-size)](https://crates.io/crates/anchor-size)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

![LOGO](asset.png)

**Anchor Size** is a CLI tool for automatically calculating account sizes in Anchor (Solana).  
No more manual `space = 8 + ...` calculations or worrying about account overflow.

The CLI analyzes your code, finds `#[account]` structs, and calculates:
- Exact account size
- Recommended size with padding
- Rent cost in SOL
- JSON report for CI/CD integration

---

## ✨ Features

### - 🔎 Automatically detects Anchor workspace
### - 📦 Analyzes all `#[account]` structs
### - 🧠 Supports nested structs and complex types
### - 🧵 Handles `String`, `Vec`, `Option`
### - 💰 Calculates rent in SOL (mainnet/devnet)
### - 📄 Generates JSON reports for CI/CD
### - 🚀 Works as a global CLI tool

---

## 📥 Installation

### Method #1 (Recommended)

Install globally from the local repository:

```bash
cargo install --path cli
```

After installation, the command is available globally:
bash

```bash
anchor-size --help
```

### Method #2 (Run without installation)
```bash
cargo run --bin anchor-size -- <PATH>
```

---

## 🚀 Quick Start

### Navigate to your Anchor project root:
```bash
cd my-anchor-project
```

Run analysis for all programs:
```bash
anchor-size . --all
```

---

## 📝 Example Output
```text

Detected Anchor workspace

Account: UserProfile
  authority → 32 bytes
  stats → 9 bytes
  name → 34 bytes

Exact size: 83 bytes
Recommended size: 96 bytes
Rent (mainnet): 0.001559 SOL
```

---

## 📊 JSON Report

### Generate a JSON report for CI/CD pipelines:

Print JSON to terminal
```bash
anchor-size . --all --json
```

Save report to file
```bash
anchor-size . --all --json --out-dir reports
```

This creates reports/anchor-size-report.json:
```json

[
  {
    "name": "UserProfile",
    "exact_size": 83,
    "recommended_size": 96,
    "rent_sol": 0.001559
  }
]
```

### Perfect for pre-deployment checks in CI/CD.

---

## ⚙️ Usage with a Single File

You can also analyze a specific file:
```bash
anchor-size programs/my_program/src/lib.rs
```

---

## 🧠 How It Works

### 1️⃣ CLI detects the Anchor workspace
### 2️⃣ Searches for programs/*/src directories
### 3️⃣ Parses Rust code using syn
### 4️⃣ Finds structs with #[account] attribute
### 5️⃣ Recursively calculates size for all fields

---

## 📚 Supported Types

| Type          | Size Calculation           | Notes                            |
|---------------|---------------------------|----------------------------------|
| `Pubkey`      | 32 bytes                  |                                  |
| `u8`          | 1 byte                    |                                  |
| `u16`         | 2 bytes                   |                                  |
| `u32`         | 4 bytes                   |                                  |
| `u64`         | 8 bytes                   |                                  |
| `bool`        | 1 byte                    |                                  |
| `String`      | 4 + `max_len`             | Prompts for max string length    |
| `Vec<T>`      | 4 + `max_items × size(T)` | Prompts for max items            |
| `Option<T>`   | 1 + `size(T)`             |                                  |
| Nested struct | Sum of fields             | Automatically calculated         |


### For dynamic types (String, Vec), the CLI will prompt:
``` text
Enter max string length for field 'name':
Enter max items for field 'items':
```

---

## 🧠 Why Recommended Size?

Anchor accounts cannot be resized after creation.
The tool automatically adds:

    +15% buffer for future updates

    8-byte alignment (Anchor accounts must be 8-byte aligned)

This protects against future account migrations and updates.

---

## 🛠 CLI Arguments

| Argument         | Description                       |
|-----------------|-----------------------------------|
| `<PATH>`         | Path to project or file           |
| `--all`          | Analyze all programs in workspace |
| `--json`         | Output JSON format                |
| `--out-dir <dir>`| Save JSON report to directory     |
| `--help`         | Show help                         |


---

## 💡 Who Is This For?

    Solana developers working with Anchor

    Projects with multiple programs

    DAOs and DeFi teams

    CI/CD pipelines needing pre-deployment validation

---

## ❤️ Why Use This?

Account size mistakes lead to:
### ❌ Failed deployments
### ❌ Overpaying for rent
### ❌ Inability to migrate accounts

### This CLI eliminates manual calculations and reduces human error.

---

## 📄 Example Anchor Account
``` rust

#[account]
pub struct UserProfile {
    pub authority: Pubkey,        // 32 bytes
    pub stats: UserStats,         // nested struct
    pub name: String,             // dynamic
}

#[account]
pub struct UserStats {
    pub level: u8,                // 1 byte
    pub xp: u64,                  // 8 bytes
    pub achievements: Vec<u8>,     // dynamic
}
```

### Running anchor-size on this will prompt for:

    Max length of name string

    Max items in achievements vector

---

## 📜 License

### MIT

---

## 🤝 Contributing

PRs and issues are welcome! Feel free to contribute to make Anchor development even better.

