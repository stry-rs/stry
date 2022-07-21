# Fenn

Fenn is a series of extension trait for core std types and general utilities.

Supports `#[no_std]`.

## Macros

### `capture`

The `capture` macro allow you to clone, reference or copy variables captured by a closure.

```rust
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

fn test() {
    let arc_atomic = Arc::new(AtomicBool::new(false));
    let arc_mutex = Arc::new(Mutex::new(Ordering::AcqRel));

    run({
        fenn::capture!(arc_atomic, arc_mutex => mutex);

        move || {
            let lock = mutex.lock().unwrap();

            arc_atomic.store(true, *lock);
        }
    });
}

fn run(f: impl FnOnce() -> ()) {
    f();
}
```

## Extensions

### String

  - `String.trim`
  - `String.trim_matches`
  - `String.trim_start`
  - `String.trim_start_matches`
  - `String.trim_end`
  - `String.trim_end_matches`

### Vec

  - `Vec.appended`
  - `Vec.cleared`
  - `Vec.deduped`
  - `Vec.deduped_by`
  - `Vec.deduped_by_key`
  - `Vec.resized`
  - `Vec.sorted`
  - `Vec.sorted_by`
  - `Vec.sorted_by_key`
  - `Vec.truncated`

## General

### Peep

  - `<T: Peep>.peep`
  - `<T: Peep>.peep_dbg`
  - `<T: Peep>.peep_mut`
  - `<T: Peep>.peep_mut_dbg`

### PeepOption

`PeepOption` is just `Peep` for `Option`s, with function for the `Some` and `None` variants.

  - `Option.peep_some`
  - `Option.peep_some_dbg`
  - `Option.peep_some_mut`
  - `Option.peep_some_mut_dbg`
  - `Option.peep_none`
  - `Option.peep_none_dbg`

### PeepResult

`PeepResult` is just `Peep` for `Result`s, with function for the `Ok` and `Err` variants.

  - `Result.peep_ok`
  - `Result.peep_ok_dbg`
  - `Result.peep_ok_mut`
  - `Result.peep_ok_mut_dbg`
  - `Result.peep_err`
  - `Result.peep_err_dbg`
  - `Result.peep_err_mut`
  - `Result.peep_err_mut_dbg`
