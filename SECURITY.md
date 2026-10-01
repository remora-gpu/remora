# Security Policy

Remora executes code supplied by remote clients on GPU servers, so we take security reports seriously.

## Reporting a vulnerability
Report privately via [GitHub Security Advisories](https://github.com/remora-gpu/remora/security/advisories/new). Please do not open public issues for vulnerabilities.

Include affected versions, configuration, reproduction steps and impact. We aim to acknowledge reports within 3 days and to ship a fix or mitigation for critical issues within 30 days, coordinating disclosure with the reporter. Fixed issues get a CVE when applicable.

## Supported versions
Before 1.0 only the latest release receives security fixes. From 1.0 on, the latest minor release of every supported major version is maintained.

## Scope
In scope: protocol parsing, authentication, authorization, isolation between clients on a server, coordinator and relay, installers and release artifacts.
Out of scope: vulnerabilities in the NVIDIA driver or CUDA libraries themselves (report those to NVIDIA), and deployments with the explicit `insecure` transport mode on untrusted networks.
