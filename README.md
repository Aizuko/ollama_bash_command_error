# Ollama for your bash terminal!

This is a silly repository providing fun feedback from a local LLM right in your
bash terminal.

![Australian Ollama telling the user to fix their bash
command](https://cdn.discordapp.com/attachments/1084216976186024026/1198525661426491492/screenshot_shm.png?ex=65bf3901&is=65acc401&hm=83b42a9c324cd8c9bb971805c92c811e009f232f8add9564ec167f7eb39b66e1&)

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
