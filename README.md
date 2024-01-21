# Ollama for your bash terminal!

This is a silly repository providing fun feedback from a local LLM right in your
bash terminal.

## Setup

Requirements:
 * Bash
 * Rust
 * Ollama (optionally any LLM backend)

You will need to install and run [Ollama](https://ollama.ai). Using other
backends with a REST API should be pretty easy, just modify the request in
`main.rs`. It'll work with Ollama out of the box.

Next run the install script:

```
bash install.sh
```

Now `source .bashrc` and type a wrong command!
