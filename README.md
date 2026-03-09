[![progress-banner](https://backend.codecrafters.io/progress/shell/b89903a1-f3ba-47b7-92fc-6a89bebe42df)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`. Study and
uncomment the relevant code, then run the command below to execute the tests on
our servers:

```sh
codecrafters submit
```

Time to move on to the next stage!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.93)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Rust Lessons Learned - Log

## Stage 1 — Print prompt, read input, print "command not found"

- `read_line()` includes the trailing `\n` in the string — always `.trim()` before using the input
- `print!()` (no newline) doesn't flush stdout automatically. You must call `io::stdout().flush().unwrap()` to ensure the prompt appears before blocking on input. `println!()` flushes automatically on most systems, `print!()` does not
- Block expressions (`let x = { ... };`) let you scope mutable variables. The `mut` binding lives and dies inside the block, and only the final value escapes. This is idiomatic Rust for keeping mutability contained — coming from Python/Java, think of it as a mini-function without the function
- `unwrap()` panics on `Err`/`None`. Fine for exercises, but in production you'd use `?` or `match` to handle errors gracefully
- `#[allow(unused_imports)]` suppresses compiler warnings — only use it intentionally. Rust's compiler warnings are your friend, especially as a beginner

## Stage 2 — The REPL

- `loop {}` is Rust's infinite loop — preferred over `while true` (the compiler warns against the latter). Coming from Python/Java, think `while True:` / `while (true)` but more idiomatic
- `read_line()` returns `Ok(n)` where `n` is bytes read. On EOF it returns `Ok(0)` — worth checking later to break out of the loop gracefully
