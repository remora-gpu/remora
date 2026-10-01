# Remora

**Use NVIDIA GPUs over the network — transparently.**

Remora is an open-source remote CUDA system. Run PyTorch, llama.cpp, Ollama, Blender or your own CUDA programs on a machine without a GPU (or with a weak one) while the work executes on a GPU server elsewhere on your network — no code changes:

```sh
# on the GPU machine
remora server

# on your laptop
remora run -- python train.py
```

> **Status: pre-alpha.** The foundation is in place; the first working CUDA path lands in `v0.1.0`. See the [roadmap](docs/roadmap.md).

## Goals
- **Transparent.** Replaces `libcuda` / `nvcuda.dll` and CUDA libraries (cuBLAS, cuDNN, NVML, …) on the client; applications see remote GPUs as local devices.
- **Cross-platform.** Linux and Windows clients and servers, in any combination.
- **Fast.** Asynchronous batched protocol, module caching, pipelined memory copies; TCP, QUIC, shared-memory and RDMA transports.
- **Secure and multi-tenant.** TLS 1.3, tokens and mTLS, per-client sandboxed workers, VRAM and time quotas.
- **Operable.** Optional coordinator for GPU pools, NAT traversal, Prometheus metrics, web dashboard, call tracing.

## Documentation
- [Architecture](docs/architecture.md)
- [Wire protocol (draft)](docs/protocol.md)
- [Threat model](docs/threat-model.md)
- [Roadmap](docs/roadmap.md)
- Русская документация: [docs/ru](docs/ru/index.md)

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md). Design changes go through [RFCs](rfcs/README.md).

## License
[Apache-2.0](LICENSE). NVIDIA and CUDA are trademarks of NVIDIA Corporation; Remora is not affiliated with NVIDIA.
