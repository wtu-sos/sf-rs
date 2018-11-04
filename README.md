# sf-rs
reimiplement snowflake by rust

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
snowflake = "0.1"
```

and this to your create root:

```rust
extern create snowflake_rs;
```

### Generated Code Example

single thread example
```rust
	let mut id_gen = SnowFlakeId::new(1, snowflake::STANDARD_EPOCH);
	println!("{:?}",&id_gen);
	let now = Instant::now();
	for _ in 1..1000 {
		let t  = &mut id_gen;
		assert!(t.generate_id().is_ok());
	}
	let elapsed = now.elapsed();
	println!("single thread generate 1000 ids cost {}.{:09} s",elapsed.as_secs(), elapsed.subsec_nanos());

```

multi thread example
```rust
    let id_gen = SnowFlakeId::new_multi_thread(2, STANDARD_EPOCH);
    let mut ths = Vec::new();
    for _i in 1 .. 10{
        let t = id_gen.clone();
        ths.push(thread::spawn(move || {
            for _ in 1..1000 {
                let mut gen = t.lock().unwrap();
                let id = gen.generate_id();
                assert!(id.is_ok());
                println!("{:?}",id.unwrap());
            }
        }));
    }

    for t in ths {
        t.join();
    }

```

## License
`sf-rs` is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.

Copyright 2017 wtu-sos
