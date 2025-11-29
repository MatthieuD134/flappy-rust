#!/usr/bin/env bash
# bootstrap.sh
# Safe developer setup for Rust template repository
# Installs required CLI tools only if missing

set -e

# Color definitions - only use colors if output is to a terminal
if [ -t 1 ]; then
  RED='\033[0;31m'
  GREEN='\033[0;32m'
  YELLOW='\033[0;33m'
  ORANGE='\033[38;5;208m'
  BLUE='\033[0;34m'
  MAGENTA='\033[0;35m'
  CYAN='\033[0;36m'
  BOLD='\033[1m'
  RESET='\033[0m'
else
  RED=''
  GREEN=''
  YELLOW=''
  ORANGE=''
  BLUE=''
  MAGENTA=''
  CYAN=''
  BOLD=''
  RESET=''
fi

echo -e ""
echo -e "${BOLD}${GREEN}========== [BOOTSTRAP] ==========${RESET}"
echo -e ""

# List of tools: "<name>:<crate>:<semver_version>:<check_command>"
# semver_version uses ^major.minor, e.g., ^0.37 means >=0.37.0 <1.0.0
TOOLS=(
  "cargo-make:cargo-make:^0.37:cargo make --version"
  "git-cliff:git-cliff:^2.10:git-cliff --version"
  "taplo:taplo-cli:^0.10:taplo --version"
  "rumdl:rumdl:^0.0.183:rumdl --version"
  "cargo-nextest:cargo-nextest:^0.9:cargo nextest --version"
  "cargo-audit:cargo-audit:^0.22:cargo audit --version"
  "cargo-deny:cargo-deny:^0.18:cargo deny --version"
  "commitlint:commitlint-rs:^0.2:commitlint --version"
  "cargo-bundle:cargo-bundle:^0.9:cargo bundle --version"
)

# Function to convert ^semver to minimal version
semver_min() {
  # input: ^0.37 -> output: 0.37.0
  local v="$1"
  v="${v#^}"  # strip ^
  [[ "$v" =~ ^([0-9]+)\.([0-9]+)$ ]] && echo "${BASH_REMATCH[1]}.${BASH_REMATCH[2]}.0" || echo "$v"
}

# SemVer comparison: returns 0 if v1 >= v2, 1 otherwise
version_ge() {
  local ver1="${1%%-*}"  # strip pre-release
  local ver2="${2%%-*}"

  IFS='.' read -r -a ver1_parts <<< "$ver1"
  IFS='.' read -r -a ver2_parts <<< "$ver2"

  # Fill shorter version with zeros
  local len=${#ver1_parts[@]}
  [ ${#ver2_parts[@]} -gt $len ] && len=${#ver2_parts[@]}

  for ((i=0; i<len; i++)); do
    local v1=${ver1_parts[i]:-0}
    local v2=${ver2_parts[i]:-0}
    if ((10#$v1 > 10#$v2)); then
      return 0
    elif ((10#$v1 < 10#$v2)); then
      return 1
    fi
  done

  # If versions equal numerically, release > pre-release
  local pre1="${1#${ver1}-}"
  local pre2="${2#${ver2}-}"
  if [ "$pre1" != "$1" ] && [ "$pre2" = "$2" ]; then
    return 1
  fi
  return 0
}

echo -e "Bootstrapping developer tools..."

for entry in "${TOOLS[@]}"; do
  IFS=":" read -r name crate semver check_command <<< "$entry"
  min_version=$(semver_min "$semver")
  if [ -z "$check_command" ]; then
    check_command="$name --version"
  fi

  echo -e "Checking ${BOLD}${CYAN}$name${RESET}..."
  installed_version=""
  if command -v "$name" &>/dev/null; then
    # Get first number-like token from output (allow pre-release)
    installed_version=$(eval "$check_command" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+(\.[0-9]+)?(-[0-9A-Za-z.-]+)?' | head -n1)
  fi

  if [ -z "$installed_version" ]; then
    echo -e "${YELLOW}Installing${RESET} ${BOLD}${CYAN}$name${RESET} ${YELLOW}version${RESET} ${BOLD}${MAGENTA}$semver${RESET}..."
    cargo install "$crate" --version "$semver"
  else
    if version_ge "$installed_version" "$min_version"; then
      echo -e "${GREEN}✓${RESET} ${BOLD}${CYAN}$name${RESET} version ${BOLD}${ORANGE}$installed_version${RESET} is compatible (>= ${BOLD}${MAGENTA}$min_version${RESET})"
    else
      echo -e "${YELLOW}Reinstalling${RESET} ${BOLD}${CYAN}$name${RESET} ${YELLOW}(${RESET}${BOLD}${ORANGE}$installed_version${RESET} → ${BOLD}${MAGENTA}$semver${RESET}${YELLOW})${RESET}..."
      cargo install "$crate" --version "$semver" --force
    fi
  fi
done

echo -e ""
echo -e "${BOLD}Setting up git hooks with cargo-husky...${RESET}"
rm -f .git/hooks/pre-commit .git/hooks/pre-push .git/hooks/commit-msg .git/hooks/post-merge
cargo clean -p cargo-husky && cargo test
echo -e ""


echo -e ""
echo -e "${BOLD}${GREEN}========== [BOOTSTRAP COMPLETE] ==========${RESET}"
echo -e ""