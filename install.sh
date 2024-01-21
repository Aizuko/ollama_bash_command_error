#!/usr/bin/env bash
declare -r script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$script_dir"
cargo build --release

cat <<EOF >> ~/.bashrc
command_not_found_handle() {
    ${script_dir}/target/release/ollama-bash-command-error "\$1"
}
EOF
