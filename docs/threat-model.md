# Threat model

Remora lets a remote party execute arbitrary GPU code on a server. This page lists what we protect, from whom, and how.

## Assets
- GPU server host: OS, files, credentials, other workloads.
- Data and models of each client (in transit and in GPU memory).
- Availability of GPUs for legitimate users.
- Coordinator: user accounts, tokens, accounting data.

## Adversaries
| Adversary | Capabilities |
|---|---|
| Network attacker | Observes and modifies traffic between client, server and coordinator |
| Unauthenticated peer | Can reach the server/coordinator port |
| Authenticated malicious client | Sends arbitrary, malformed or adversarial protocol messages and GPU kernels |
| Curious co-tenant | Another legitimate client on the same GPU |
| Compromised server | Operator or attacker controlling a GPU server |

## Mitigations
- **Transport security.** TLS 1.3 (rustls) by default on TCP and QUIC; mTLS and API tokens; pairing codes for first contact. The `insecure` mode must be enabled explicitly and prints a warning.
- **Strict validation.** The server decodes every frame defensively: bounded lengths, known kinds and flags, handles resolved only through per-session tables, device pointers checked against the session's allocations. Decoders are fuzzed continuously.
- **Isolation.** One worker process per client with minimal privileges; on Linux seccomp, cgroups and namespaces, on Windows Job Objects and AppContainer. A worker crash affects only its client.
- **Resource limits.** VRAM quotas enforced at allocation time, time slicing and priorities, per-client rate limits on connections and requests.
- **Co-tenancy.** GPU memory is not shared between sessions; freed memory is scrubbed before reuse when strict mode is enabled. For strong isolation use exclusive mode or MIG.
- **Accountability.** Audit log of sessions, authentication events and administrative actions.

## Out of scope / residual risks
- Vulnerabilities in the NVIDIA driver or GPU hardware reachable by crafted kernels. Run untrusted tenants in exclusive mode or MIG and keep drivers updated.
- A compromised server can read the data of clients connected to it. Clients must trust the servers they use.
- Timing and side channels between co-tenants on the same GPU.
