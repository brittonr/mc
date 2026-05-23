# Stevenarella → Valence protocol 763 gameplay smoke — 2026-05-23

Receipt BLAKE3: `afc679e6d7a6f3eb476a5dfd8c352c00304ba8c74a920c5ed030e589c003d9ff`

## What ran

Started Valence `ctf` from `/home/brittonr/git/mc/valence`, then ran two bounded headless Stevenarella probes from the user fork commit `4c5e89d` against `127.0.0.1:25565`:

- 90s normal probe: `timeout 90s xvfb-run -a cargo run --release -- --server 127.0.0.1:25565 --username steve763`
- 45s debug probe: `timeout 45s xvfb-run -a cargo run --release -- --network-debug --server 127.0.0.1:25565 --username steve763debug`

Both probes detected `Detected server protocol version 763` and reached the bounded timeout (`exit=124`) without a logged panic/error/unmapped-packet/decoder failure.

## Result

This is a bounded runtime smoke after the observed packet-ID boundary drain. It raises confidence that Stevenarella stays alive after protocol detection against Valence `ctf` protocol 763, but it did not surface a concrete next semantic/parser failure.

Artifacts hashed in the receipt:

- Normal log: `5e162ff70d8e5b926872992afc3a47d718ddd18a8f246e7f2cdb2d658e1f8528`
- Normal status: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`
- Debug log: `b4a267a834b14b7209c42e0a89455edf980474936f9f372148105e95aed2e963`
- Debug status: `f67d19a0fed77375c9ee600f70ff255957e708d6c0898a270d40151761401f0a`

## Non-claims

Does not prove full Minecraft 1.20.1 compatibility.
Does not prove full Stevenarella protocol 763 support.
Does not prove semantic packet parser correctness.
Does not prove in-world gameplay success.
Does not prove every protocol 763 packet is mapped.
