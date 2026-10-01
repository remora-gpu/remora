# Wire protocol (draft v0.1)

::: warning Draft
The protocol is unstable until 1.0. Before 1.0 every minor version is a breaking change; from 1.0 peers interoperate within a major version.
:::

All integers are little-endian. The reference implementation is the `remora-proto` crate.

## Connection lifecycle
1. Transport connects (TCP/QUIC with TLS 1.3 by default; shared memory or RDMA set up through a TCP control connection).
2. **Handshake** (`Control` frames): `ClientHello` → `ServerHello` → authentication → `SessionReady`.
3. API traffic: `Request` / `Response` / `Event` frames.
4. `Goodbye` or transport close. A session may be resumed within a server-configured timeout after an unexpected disconnect.

## Frame header
Every frame starts with a fixed 24-byte header, followed by `inline_len` bytes of structured payload and `bulk_len` bytes of bulk data.

| Offset | Size | Field | Description |
|---:|---:|---|---|
| 0 | 4 | `inline_len` | Size of the structured payload, at most 16 MiB |
| 4 | 1 | `kind` | 1 Request, 2 Response, 3 Event, 4 Control |
| 5 | 1 | `flags` | See below; unknown bits are a protocol error |
| 6 | 2 | `channel` | Logical ordering channel (e.g. a CUDA stream) |
| 8 | 4 | `method` | `(api_family << 16) \| index`, assigned by the code generator |
| 12 | 4 | `bulk_len` | Size of the bulk section of this frame |
| 16 | 8 | `request_id` | Monotonic per connection; responses echo it |

### Flags
| Bit | Name | Meaning |
|---:|---|---|
| 0 | `ASYNC` | The client does not wait for a response; errors are deferred to the next synchronizing call |
| 1 | `COMPRESSED` | The bulk section is compressed with the negotiated algorithm |
| 2 | `BATCH_CONTINUES` | More frames of this batch follow; the server may defer execution |
| 3 | `BULK_CONTINUATION` | This frame continues a chunked bulk transfer |

### API families
| Id | Family |
|---:|---|
| 0 | Remora control |
| 1 | CUDA Driver API |
| 2 | NVML |
| 3 | cuBLAS / cuBLASLt |
| 4 | cuDNN |
| 5 | cuFFT |
| 6 | cuSPARSE |
| 7 | cuSOLVER |
| 8 | cuRAND |
| 9 | NVRTC / nvJitLink |
| 10 | NCCL |
| 11 | cuTENSOR |
| 12 | NPP |

## Handshake
`ClientHello`: magic `RMRA`, protocol version (`major: u16`, `minor: u16`), capability bitset (`u64`), client OS/arch, CUDA API version the application requested, supported compression algorithms, client nonce.
`ServerHello`: magic, version, capability bitset, server id, list of advertised GPUs (opaque ids), selected compression, server nonce.

Negotiation: same `major` (and, before 1.0, same `minor`) is required; the effective capability set is the intersection of both bitsets.

## Payload encoding
The inline payload of a call is a generated, fixed-order encoding of the function's arguments: scalars as little-endian values, handles as 64-bit session-scoped ids (never raw server pointers), device pointers as 64-bit virtual addresses validated by the worker, host input buffers moved into the bulk section. Output arguments travel back in the response the same way.

## Bulk transfers
Large host↔device copies are split into chunks (default 4 MiB) sent as consecutive frames with `BULK_CONTINUATION`, so that several transfers and calls can be multiplexed and the receiver can overlap network and PCIe copies.

## Errors
A `Response` carries the CUDA result code of the call. For `ASYNC` calls the server records the first error per channel and returns it with the next synchronizing call on that channel, matching CUDA's sticky/deferred error semantics.
