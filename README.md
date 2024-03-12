# Deno Embed

This library is intended to provide a simple "batteries included" experience to embed Deno into a Rust application.

It repackages the `deno_cli` package and offers a programmatic API to access entry points like `deno run ./script.ts`

## Todo

There is a lot of extra stuff in here like linting, docs generation, benchmarking, etc that may not be important and come with the burden of additional unused dependencies.

Deno CLI is a little difficult to untangle so I have left them in there for now and will manually "tree shake" the package by hand later - perhaps enabling them using crate features.

## Usage

```rust
use deno_embed::deno_current_thread;
use deno_embed::DenoInitOptions;

fn main() {
  deno_current_thread(async move {
    let exit_code = deno_embed::run_script(
      deno_embed::DenoInitOptions{ // Combination of RunFlags and Flags
        script: "/home/dalsh/Development/alshdavid/mach/pkg/index.ts".to_string(),
        ..Default::default()
      })
      .await;

    println!("{:?}", exit_code);
  });
}
```
