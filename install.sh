#!/usr/bin/env bash
declare -r script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$script_dir"
cargo build --release

cat <<EOF >> ~/.bashrc

# Inserted by ollama-bash-command-error #########
command_not_found_handle() {
    if ! "${script_dir}/target/release/ollama-bash-command-error" "\$1" 2>/dev/null; then
      echo "bash: Command not found \"$1\""
    fi
}
#################################################
EOF
