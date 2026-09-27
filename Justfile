#!/usr/bin/env -S just --justfile

# Build project
build:
  #!/usr/bin/env sh
  cargo leptos build --split
  pnpm run build:dev

# Preflight check project prerequisites
preflight:
  #!/usr/bin/env sh
  echo "📝 Status check on project prerequisites"
  cargo_exists=$(command -v cargo >/dev/null && echo 0 || echo 1)
  [[ $cargo_exists -eq 1 ]] \
    && echo "❎ cargo - 'package manager for Rust' not found." \
    || echo "✅ cargo - 'package manager for Rust' found."
  cargo_leptos_exists=$(cargo leptos --version >/dev/null && echo 0 || echo 1)
  [[ $grep_exists -eq 1 ]] \
    && echo "❎ cargo-leptos - 'build tool for Leptos (Rust) ' not found." \
    || echo "✅ cargo-leptos - 'build tool for Leptos (rust) ' found."
  pnpm_exists=$(command -v pnpm >/dev/null && echo 0 || echo 1)
  if [[ $pnpm_exists -eq 1 ]]; then
    echo "❎ pnpm - 'Fast, disk space efficient package manager' not found."
    echo "❎ serve - 'static file serving and directory listing' not found."
    echo "❎ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' not found."
  else
    echo "✅ pnpm - 'Fast, disk space efficient package manager' found."
    pnpm ls | grep '^[└├]── serve@\d\+\.\d\+\.\d\+$' -q
    [[ $? -eq 1 ]] \
      && echo "❎ serve - 'static file serving and directory listing' not found." \
      || echo "✅ serve - 'static file serving and directory listing' found."
    pnpm ls | grep '^[└├]── @tailwindcss/cli@\d\+\.\d\+\.\d\+$' -q
    [[ $? -eq 1 ]] \
      && echo "❎ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' not found." \
      || echo "✅ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' found."
  fi

# Serve
serve:
  #!/usr/bin/env sh
  pnpm run serve:dev
