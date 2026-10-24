<div align="center">

<img src="assets/logo-round.png"
     alt="alycia-send logo"
     width="160"
     height="160">

<h2>alycia-send</h2>

<p>
  <strong>A lightweight CLI tool for automated message sending.</strong>
</p>

</div>


---

**alycia-send** is a lightweight command-line utility written in Rust that automates message sending through keyboard input and the system clipboard.

It is designed to work with messaging applications that accept standard keyboard input, without requiring a dedicated API or integration.

---

## Features

- Native Rust application with a lightweight executable
- CLI-first interface powered by `clap`
- Multi-language message dictionaries
- Randomized message ordering
- Non-repeating cycles — messages are not repeated until the current dictionary cycle is exhausted
- Clipboard-based sending for reliable Unicode and emoji support
- Keyboard automation through `enigo`
- Configurable message delays
- Configurable startup countdown
- Argument validation and error handling
- Optimized release builds

---

## Supported Languages

alycia-send currently supports **12 languages**:

| Code | Language |
|------|----------|
| `fr` | 🇫🇷 French |
| `en` | 🇬🇧 English |
| `de` | 🇩🇪 German |
| `ru` | 🇷🇺 Russian |
| `es` | 🇪🇸 Spanish |
| `it` | 🇮🇹 Italian |
| `pl` | 🇵🇱 Polish |
| `ar` | 🇸🇦 Arabic |
| `ja` | 🇯🇵 Japanese |
| `tr` | 🇹🇷 Turkish |
| `pt` | 🇵🇹 Portuguese |
| `zh` | 🇨🇳 Chinese |


More languages may be added in future updates.