# Architecture

Remora implements **CUDA API remoting**: the client process loads Remora's replacements of the CUDA libraries, every call is serialized and executed by a worker on a GPU server that runs the real NVIDIA driver.

```
 Client (Linux/Windows, no NVIDIA driver)             Server (Linux/Windows, NVIDIA driver)
 ┌──────────────────────────────────────────┐        ┌──────────────────────────────────────┐
 │ Application (PyTorch, llama.cpp, .cu ...) │        │ remora-server (daemon / service)     │
 │   │ libcudart / cuBLAS / cuDNN ...        │        │   ├─ auth, sessions, scheduler, quota│
 │   ▼                                       │        │   └─ spawns remora-worker per client │
 │ remora-shim: libcuda.so.1 / nvcuda.dll,   │ wire   │        ├─ dispatcher (generated)     │
 │   libnvidia-ml, libcublas*, libcudnn* ... │◄──────►│        ├─ module cache (by hash)     │
 │   ├─ API stubs (generated)                │ TCP /  │        └─ real libcuda + libraries   │
 │   ├─ async batching, local caches         │ QUIC / │                                      │
 │   └─ transport + TLS                      │ SHM /  │ /metrics, dashboard, audit log       │
 └──────────────────────────────────────────┘ RDMA   └──────────────────────────────────────┘
                     ▲                                               ▲
                     └──── remora-coordinator (optional): registry, GPU allocation, RBAC, relay ──┘
```

## Interception levels
- **CUDA Driver API** (`libcuda.so.1`, `nvcuda.dll`) is the core. Everything above it — the CUDA Runtime (static or dynamic), PTX JIT, framework allocators — works through it, including `cuGetProcAddress` and the undocumented export tables used by `cudart`.
- **CUDA libraries** (cuBLAS/cuBLASLt, cuDNN, cuFFT, cuSPARSE, cuSOLVER, cuRAND, NVRTC, nvJitLink, NVML, NCCL, cuTENSOR, NPP) are remoted **as whole calls**. Forwarding a single `cublasGemmEx` is far cheaper than forwarding the dozens of driver calls it would make internally, and these libraries use private driver entry points.

## Components
| Crate | Role |
|---|---|
| `remora-codegen` | libclang → IR → annotations (TOML: pointer direction, buffer sizes, sync/async, handle types) → client stubs, server dispatchers, codecs, tracing, coverage tables |
| `remora-proto` | Frame layout, codec, versioning, handshake. Normative spec: [Wire protocol](./protocol) |
| `remora-transport` | `Transport` trait with TCP, QUIC, shared-memory and RDMA implementations |
| `remora-shim` | `cdylib`s exported with the exact symbol names (and symbol versions on Linux) of the NVIDIA libraries |
| `remora-server` | Daemon/Windows service: authentication, session management, scheduling, quotas, metrics |
| `remora-worker` | One process per client session, optionally sandboxed; owns the CUDA contexts of that client |
| `remora-coordinator` | Optional: server registry, GPU allocation, accounting, RBAC/OIDC, NAT relay, HA |
| `remora-cli` | `remora run / server / doctor / bench / trace / top / pair` |
| `remora-sdk` | Rust, C and Python API for managing sessions programmatically |

## Key design decisions
- **Rust everywhere**, C ABI only at the exported symbol boundary. The shim uses no async runtime inside the host process; servers use tokio.
- **Generated, not hand-written.** Hundreds of API functions are generated from headers plus annotations. Hand-written code is reserved for modules, graphs, export tables, virtual and managed memory.
- **No NVIDIA bits in the repository.** Headers are read from a locally installed CUDA Toolkit during generation; only annotations and generated signature descriptions are committed.
- **Asynchrony preserved.** Calls that are asynchronous in CUDA (launches, async copies, event records) are sent without waiting for a reply and batched; errors surface at the next synchronizing call, exactly as CUDA specifies.
- **Local caches.** Device properties, attributes and function metadata are fetched once per session. Modules are uploaded once per server, keyed by content hash.
- **Server never trusts the client.** Every handle, length and device pointer is validated against the session's own tables before use.
- **Process per client.** A crashing or misbehaving client cannot take down others; quotas and sandboxing are enforced per worker.

## Platforms
Clients and servers run on Linux (x86_64, aarch64) and Windows 10/11 x64 in any combination. ABI differences (LLP64 `long`, struct alignment, `HANDLE` vs file descriptors) are normalised by the code generator into a platform-independent wire representation. macOS is not supported.
