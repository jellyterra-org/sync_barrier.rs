# Sync Barrier

Block thread until counter reached zero.

# Usage

```toml
# Cargo.toml
[dependencies]
sync_barrier = "1.0.0"
```

```rust
let barrier = Arc::new(SyncBarrier::new());

// barrier.add(queue.len())
for item in queue {
    barrier.add(1);

    let barrier = barrier.clone();
    tokio::spawn(async move {
        // Do something.
        barrier.done();
    });
}

barrier.wait();
```

## Licensing

Public domain.
