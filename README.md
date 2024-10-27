Rust Dir-Scan is a lightweight and efficient directory scanner built in Rust. Designed for speed and security, it quickly scans directories, making it an ideal tool for web penetration testing and security assessments.

# Rust Dir-Scan

**Rust Dir-Scan** is a fast and efficient directory scanner built using Rust. It allows users to scan a given URL for potential directories or files, utilizing a wordlist for enumeration. The tool is designed with speed and security in mind, making it ideal for web penetration testing and security assessments.

## Features

- **Asynchronous Scanning:** Utilizes Rust's async capabilities for faster scanning.
- **Progress Bar:** Displays a real-time progress bar, indicating the scan's completion status.
- **Colored Output:** Easily distinguish valid URLs with color-coded output.
- **Input Validation:** Ensures valid URLs and file paths are specified by the user.

## Prerequisites

Before running the tool, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (1.67+)
- [Tokio](https://tokio.rs/) (for async capabilities)
- [Reqwest](https://docs.rs/reqwest/) (for HTTP requests)
- [Colored](https://docs.rs/colored/) (for color-coded console output)
- [Indicatif](https://docs.rs/indicatif/) (for the progress bar)

You can add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
colored = "2.0"
indicatif = "0.17"
```
## Installation

1. CloneThe Repository :

   ```bash
   git clone https://github.com/siinomega/RD-S.git
   ```
2. Accédez au répertoire du projet :

   ```bash
   cd RD-S
   ```
3. Run the scanner :

   ```bash
   cargo build --release
   ```
---
## Contribuer

If you would like to contribute to this project, feel free to submit a pull request or report issues. Any contribution is welcome!

---
## 📄 Licence

This project is licensed under the [MIT](LICENSE).
---

Thank you for checking out this project! Feel free to reach out to me if you have any questions or suggestions.
