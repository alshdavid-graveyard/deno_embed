# Deno Embed

This library is intended to provide a simple "batteries included" experience to embed Deno into a Rust application

## Usage

```rust
use deno_embed::DenoContext;
use deno_embed::DenoContextOptions;

fn main() {
  let ctx = DenoContext::new(DenoContextOptions::default());

  // Will print "42" to the console
  ctx.run_code("cached_module_name.js", "console.log(42)")
    .recv()
    .unwrap()
    .unwrap();
}
```
