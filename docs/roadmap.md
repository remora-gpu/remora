# Roadmap to 1.0.0

Clients and servers: Linux and Windows. macOS is not supported. Games and graphics are a separate 2.x track.
Every stage ends with a public pre-release, a git tag and an acceptance run on a real GPU (rented cloud instances, Ampere/Ada class).

| Stage | Release | Scope | Done when |
|---|---|---|---|
| 0. Foundation | `v0.0.1` | Org and repo, Apache-2.0, DCO, governance, RFC process, Cargo workspace, CI (Linux + Windows), docs site EN/RU, architecture, protocol draft, threat model | CI green on both OSes, docs published |
| 1. Core | `v0.1.0` | Code generator (libclang → IR → annotations → stubs/dispatchers/codecs), protocol codec and handshake, TCP transport, minimal Driver API (device, context, memory, modules, launch, streams, events), `remora run`/`server` on Linux, mock server for CI | CUDA Samples `vectorAdd`, `matrixMul`, `deviceQuery`, `bandwidthTest` work Linux → Linux |
| 2. Full Driver API | `v0.2.0` | ≥ 95% of Driver API, `cuGetProcAddress`, export tables, static and dynamic cudart, module cache by hash, local property caches | ≥ 95% of CUDA Samples (except interop); CuPy and Numba basics |
| 3. Libraries & inference | `v0.3.0` | NVML, cuBLAS/cuBLASLt, cuDNN v8/v9, NVRTC, nvJitLink, cuRAND, cuFFT, cuSPARSE, cuSOLVER; `remora smi` | PyTorch inference, llama.cpp, Ollama, ComfyUI SDXL, ONNX Runtime |
| 4. Windows | `v0.4.0` | `nvcuda.dll` and library DLLs on the client, Windows server as a service, LLP64 ABI handling | Stage 3 scenarios in all 4 client/server OS combinations |
| 5. Performance & tooling | `v0.5.0` | Async batching, deferred errors, pipelined chunked copies, shared memory, QUIC, io_uring / RIO, compression; `remora bench`, `remora trace` (Perfetto), perf regression CI | Performance targets met on LAN; benchmarks published |
| 6. Advanced CUDA | `v0.6.0` | CUDA Graphs, virtual memory management, managed memory (userfaultfd / guard pages), multi-GPU, P2P, IPC, multiple servers per process, NCCL | Full framework matrix: PyTorch training + `torch.compile`, DDP, TensorFlow, JAX, vLLM, RAPIDS, Blender Cycles (CUDA/OptiX), Hashcat |
| 7. Multi-tenancy & security | `v0.7.0` | Worker per client, sandboxing (seccomp/cgroups/namespaces, Job Objects/AppContainer), VRAM/time quotas, exclusive/time-slicing/MPS/MIG, TLS 1.3, mTLS, tokens, pairing, audit log, fuzzing | 4+ concurrent clients on one GPU with quotas; 24 h fuzzing clean |
| 8. Coordinator & networking | `v0.8.0` | Coordinator (registry, allocation, accounting, HA, RBAC, OIDC), mDNS, NAT traversal and relay, reconnect with session resume, Prometheus, OpenTelemetry, web dashboard | Client behind NAT reaches a GPU server over the internet; dashboard shows clients, VRAM, load |
| 9. RDMA | `v0.9.0` | ibverbs transport (RoCE v2 / InfiniBand), GPUDirect RDMA where available, Soft-RoCE in CI | < 30 µs p50 round-trip on an RDMA testbed |
| 10. Product | `v1.0.0-rc.1` | `.deb`/`.rpm`/AUR/tarball/`curl \| sh`, MSI/winget/Scoop, Docker, Helm, pip; signed releases, SBOM; tray app, `remora top`, `remora doctor`; full docs EN+RU | One-command install on clean Windows and Ubuntu runs PyTorch on a remote GPU in < 5 minutes |
| 11. Stabilisation | `v1.0.0` | Protocol v1 freeze, cross-version tests, 72 h soak, network chaos tests, security checklist, LTS 1.x | Release candidates without blockers |

## Performance targets (LAN 10G, relative to local)
- Compute-bound (large-model training, LLM inference with weights in VRAM): ≥ 90%.
- Typical PyTorch training (ResNet-50, BERT): ≥ 75%.
- Synchronous call round-trip: < 30 µs p50 on 10G/RDMA, < 100 µs on 1G.

## Testing without owning a GPU
- Every PR: GitHub Actions on Linux and Windows; unit, protocol and codegen tests, end-to-end tests against a **mock server** (a deterministic fake GPU).
- Stage acceptance and nightly runs: `cargo xtask gpu-e2e` rents a GPU instance through a provider API, deploys the server, runs the test matrix and tears the instance down.
- Special configurations (multi-GPU, MIG, RDMA, 10G pairs) are rented for the stages that need them.

## After 1.0 — track 2.x
- **Games and graphics.** Not CUDA: start with frame streaming (NVENC → network → decoder, input back, Moonlight/Sunshine style), reusing Remora's transport, auth, coordinator and dashboard; Vulkan/D3D API remoting as research.
- TensorRT, NVENC/NVDEC as CUDA APIs, ROCm/HIP server backend, live session migration, GPU state checkpointing.
