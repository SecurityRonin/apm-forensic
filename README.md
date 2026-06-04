[![Crates.io](https://img.shields.io/crates/v/apm-forensic.svg)](https://crates.io/crates/apm-forensic)
[![docs.rs](https://img.shields.io/docsrs/apm-forensic)](https://docs.rs/apm-forensic)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/SecurityRonin/apm-forensic/actions/workflows/ci.yml/badge.svg)](https://github.com/SecurityRonin/apm-forensic/actions)
[![Sponsor](https://img.shields.io/badge/sponsor-h4x0r-ea4aaa?logo=github-sponsors)](https://github.com/sponsors/h4x0r)

**Pure-Rust forensic Apple Partition Map (APM) reader — Driver Descriptor Map and partition entries from a byte buffer.**

Reads the partition scheme on Apple hybrid optical discs and APM-formatted media, with no `unsafe`.

## Install

```toml
[dependencies]
apm-forensic = "0.1"
```

## Quick start

```rust
// `data` begins at the device's first byte (block 0 = Driver Descriptor Map).
let data: Vec<u8> = std::fs::read("disk.img")?;

if let Some(map) = apm_forensic::parse(&data) {
    println!("{}-byte blocks, {} partitions", map.block_size, map.partitions.len());
    for p in &map.partitions {
        println!("  {:<24} {}  start {} ({} blocks)", p.type_name, p.name, p.start_block, p.block_count);
    }
    if let Some(hfs) = map.hfs_partition() {
        println!("Apple_HFS at block {}", hfs.start_block);
    }
}
```

## What it parses

| Capability | Notes |
|---|---|
| Driver Descriptor Map | `ER` signature, device block size |
| Partition entries | `PM` entries: name, type, start block, block count |
| HFS lookup | `hfs_partition()` finds the first `Apple_HFS` slice |

## Validation

Tested against a **real `hdiutil`-created APM** (`Apple_partition_map` + `Apple_HFS` entries), so the layout is checked against genuine Apple output.

## Related

Part of the [Security Ronin](https://github.com/SecurityRonin) forensic toolkit. Sibling partition readers: [`gpt-forensic`](https://github.com/SecurityRonin/gpt-forensic), [`mbr-forensic`](https://github.com/SecurityRonin/mbr-forensic). Filesystems: [`hfsplus-forensic`](https://github.com/SecurityRonin/hfsplus-forensic), [`udf-forensic`](https://github.com/SecurityRonin/udf-forensic). Consumed by [`iso9660-forensic`](https://github.com/SecurityRonin/iso9660-forensic) for Apple hybrid discs.

---

[Privacy Policy](https://securityronin.github.io/apm-forensic/privacy/) · [Terms of Service](https://securityronin.github.io/apm-forensic/terms/) · © 2026 Security Ronin Ltd
