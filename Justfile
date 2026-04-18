#!/usr/bin/env -S just --justfile

# Build project
build:
  #!/usr/bin/env sh
  cargo leptos build
  [[ -f target/site/pkg/blog.wasm && ! -f target/site/pkg/blog_bg.wasm ]] \
    && mv target/site/pkg/blog.wasm target/site/pkg/blog_bg.wasm \
    || echo "✅ blog_bg.wasm was already built by cargo-leptos"

# Check project prerequisites
check:
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
  yarn_exists=$(command -v yarn >/dev/null && echo 0 || echo 1)
  if [[ $yarn_exists -eq 1 ]]; then
    echo "❎ yarn - 'package manager for NodeJS' not found."
    echo "❎ serve - 'static file serving and directory listing' not found."
    echo "❎ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' not found."
  else
    echo "✅ yarn - 'package manager for NodeJS' found."
    yarn global list | grep '^info "serve@\d\+\.\d\+\.\d\+" has binaries:$' -q
    [[ $? -eq 1 ]] \
      && echo "❎ serve - 'static file serving and directory listing' not found." \
      || echo "✅ serve - 'static file serving and directory listing' found."
    yarn global list | grep '^info "@tailwindcss/cli@\d\+\.\d\+\.\d\+" has binaries:$' -q
    [[ $? -eq 1 ]] \
      && echo "❎ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' not found." \
      || echo "✅ @tailwindcss/cli - 'dedicated command-line interface for TailwindCSS' found."
  fi

# Serve
serve:
  #!/usr/bin/env sh
  yarn serve

# Watch
watch:
  #!/usr/bin/env sh
  [[ ! -e dist ]] && mkdir dist
  stty -echoctl # hide ^C
  stty intr ^C
  cleanup() {
    echo 'Cleaning up processes...'
    echo $LEPTOS_WATCH
    [[ -z $LEPTOS_WATCH ]] && pkill -P $LEPTOS_WATCH
    [[ $? -eq 0 ]] && echo "Terminated cargo-leptos watch process."
    echo $YARN_SERVE
    [[ -z $YARN_SERVE ]] && pkill -P $YARN_SERVE
    [[ $? -eq 0 ]] && echo "Terminated yarn serve process." \
    echo $YARN_WATCH
    [[ -z $YARN_WATCH ]] && pkill -P $YARN_WATCH
    [[ $? -eq 0 ]] && echo "Terminated yarn watch process."
    stty sane
    exit 0
  }
  trap 'cleanup' SIGINT
  yarn watch >/dev/null 2>&1 &
  YARN_WATCH=$!
  cargo leptos watch >/dev/null 2>&1 &
  LEPTOS_WATCH=$!
  yarn serve >/dev/null 2>&1 &
  YARN_SERVE=$!
  echo "Running processes: [$YARN_WATCH, $LEPTOS_WATCH, $YARN_SERVE]"
  wait
