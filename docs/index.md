---
layout: home
hero:
  name: Remora
  text: Remote CUDA, transparently
  tagline: Run PyTorch, llama.cpp, Blender and your own CUDA code on GPUs across the network. Linux and Windows.
  actions:
    - theme: brand
      text: Architecture
      link: /architecture
    - theme: alt
      text: Roadmap
      link: /roadmap
features:
  - title: No code changes
    details: Drop-in replacements for libcuda / nvcuda.dll, cuBLAS, cuDNN, NVML and more. Applications see remote GPUs as local devices.
  - title: Fast by design
    details: Asynchronous batched calls, module caching, pipelined copies; TCP, QUIC, shared memory and RDMA transports.
  - title: Secure multi-tenancy
    details: TLS 1.3, tokens and mTLS, sandboxed per-client workers, VRAM and time quotas, audit log.
  - title: Built to operate
    details: Coordinator for GPU pools, NAT traversal, Prometheus metrics, web dashboard and call tracing.
---
