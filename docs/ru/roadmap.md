# Дорожная карта до 1.0.0

Клиенты и серверы: Linux и Windows. macOS не поддерживается. Игры — трек 2.x.
Все реальные прогоны — на **арендованных облачных GPU** (по часам: Vast.ai / RunPod / Lambda и т.п.). Базовая карта для приёмки — класса RTX 3060/3090/4090 (Ampere/Ada).

---

## Этапы

Каждый этап заканчивается публичным pre-release, тегом в git и прогоном приёмочных тестов на арендованном GPU.

### Этап 0 — Фундамент → `v0.0.1`
- GitHub-организация `remora-gpu`, репозиторий `remora`, Apache-2.0, DCO, CODE_OF_CONDUCT, SECURITY.md, шаблоны issue/PR, процесс RFC (`rfcs/`).
- Cargo workspace, `cargo xtask`, rustfmt/clippy (deny warnings), cargo-deny (лицензии/CVE), pre-commit.
- CI (GitHub Actions): Linux + Windows сборка, тесты, линт. Каркас VitePress-доков (EN+RU).
- Документы: ARCHITECTURE.md, черновик спецификации протокола v1, модель угроз.
- **Готово, когда:** CI зелёный на обеих ОС, доки публикуются на GitHub Pages.

### Этап 1 — Ядро: кодоген, протокол, TCP, минимальный Driver API → `v0.1.0`
- `remora-codegen`: парсинг `cuda.h` (12.x и 11.8), IR, формат аннотаций, генерация клиентских и серверных частей, таблица покрытия.
- `remora-proto`: framing, кодек (zero-copy для буферов), request-id, async/sync флаги, handshake с согласованием версий и capabilities.
- TCP-транспорт (tokio на сервере, лёгкий блокирующий/свой reactor в shim — без тяжёлого рантайма внутри чужого процесса).
- Driver API: init, device/context, memory (alloc/copy/memset), module/function (`cuModuleLoadData`, fatbin), `cuLaunchKernel`, streams, events, errors.
- `remora run` (Linux, LD_PRELOAD + подмена `libcuda.so.1`), `remora server` (Linux).
- Мок-сервер (fake GPU) для CI без железа.
- **Готово, когда:** CUDA Samples `vectorAdd`, `matrixMul`, `deviceQuery`, `bandwidthTest` работают Linux→Linux на арендованном GPU.

### Этап 2 — Полный Driver API + cudart → `v0.2.0`
- Driver API ≥ 95%: `cuGetProcAddress` (v1/v2), export tables, primary context, occupancy, texture/surface objects, pointer attributes, `cuLaunchKernelEx`, библиотеки модулей (`cuLibrary*`).
- Работа как со статическим, так и с динамическим cudart. Регистрация фатбинарников, кэш модулей по хэшу.
- Локальные кэши свойств и атрибутов (одна поездка по сети на сессию).
- **Готово, когда:** полный прогон CUDA Samples (кроме interop) ≥ 95%, CuPy и Numba работают с базовыми ядрами.

### Этап 3 — Библиотеки и ML-фреймворки (инференс) → `v0.3.0`
- Удалённые вызовы библиотек целиком: NVML, cuBLAS/cuBLASLt, cuDNN (v8 и v9 backend API), NVRTC, NVJitLink, cuRAND, cuFFT, cuSPARSE, cuSOLVER.
- Обработка handle-типов, workspace-буферов, callback'ов (cuFFT callbacks — через серверный JIT).
- `nvidia-smi`-совместимый `remora smi` через удалённый NVML.
- **Готово, когда:** на арендованном GPU работают инференс PyTorch (ResNet-50, BERT), llama.cpp и Ollama (Llama 3.1 8B Q4), ComfyUI SDXL, ONNX Runtime CUDA EP.

### Этап 4 — Windows → `v0.4.0`
- Windows-клиент: `nvcuda.dll`, `nvml.dll`, `cublas64_*.dll`, … Подстановка через `remora run` (инжект/redirect) и через системный режим.
- Windows-сервер как служба (SCM), установка драйверных путей, тот же воркер.
- Различия ABI (LLP64), `HANDLE`/`fd` в IPC, пути; матрица Win↔Linux в CI (Windows-runner с моком + арендованный GPU).
- **Готово, когда:** все сценарии этапа 3 проходят во всех 4 комбинациях клиент/сервер.

### Этап 5 — Производительность и инструменты → `v0.5.0`
- Батчинг async-вызовов, отложенные ошибки (семантика CUDA сохраняется), pipelining больших копий чанками, перекрытие сети и вычислений.
- Транспорты: shared memory (тот же хост/VM), QUIC (quinn), TCP-оптимизации (io_uring/MSG_ZEROCOPY на Linux, RIO на Windows), сжатие LZ4/Zstd по эвристике.
- `remora bench` (latency/bandwidth/фреймворки), `remora trace` (Perfetto), перф-CI с порогом регрессии 5%.
- **Готово, когда:** целевые показатели производительности достигнуты на LAN (10G — облачная пара машин), бенчмарки опубликованы.

### Этап 6 — Продвинутые возможности CUDA → `v0.6.0`
- CUDA Graphs (capture + явное построение + update), VMM (`cuMemCreate`/`Map`/`SetAccess`) для expandable segments PyTorch.
- Managed memory: userfaultfd (Linux) и guard pages/VEH (Windows), prefetch/advise.
- Multi-GPU на сервере, P2P, IPC; несколько серверов в одном процессе (единый список устройств, `REMORA_DEVICES`); NCCL (в т.ч. между серверами).
- PyTorch: обучение + `torch.compile`/Triton, DDP; TensorFlow, JAX, vLLM, RAPIDS, Blender Cycles (CUDA + OptiX), Hashcat.
- **Готово, когда:** вся матрица фреймворков проходит (PyTorch обучение + `torch.compile`, DDP, TensorFlow, JAX, vLLM, RAPIDS, Blender Cycles, Hashcat); multi-GPU/NCCL — на арендованном сервере с 2+ GPU.

### Этап 7 — Мультиарендность и безопасность → `v0.7.0`
- Процесс-воркер на каждого клиента; sandbox: seccomp + cgroups + namespaces (Linux), Job Objects + AppContainer (Windows).
- Квоты VRAM/времени, приоритеты, режимы эксклюзив / time-slicing / MPS / MIG.
- TLS 1.3 (rustls), mTLS, API-токены, pairing по коду, аудит-лог.
- Строгая валидация всех входных данных на сервере, cargo-fuzz декодера и диспетчера в CI.
- **Готово, когда:** 4+ клиента одновременно на арендованном GPU с квотами; падение/злоупотребление одного не влияет на других; фаззинг 24 ч без падений.

### Этап 8 — Координатор, сеть, наблюдаемость → `v0.8.0`
- `remora-coordinator`: реестр серверов, выдача GPU, учёт GPU-часов, мультиарендность (организации), HA (Raft), RBAC + OIDC.
- Обнаружение: mDNS/DNS-SD; NAT: hole punching через QUIC + relay.
- Переподключение при обрыве сети с сохранением сессии (в пределах тайм-аута).
- Prometheus `/metrics`, OpenTelemetry, JSON-логи; веб-дашборд (сервер и координатор).
- **Готово, когда:** клиент за NAT через интернет подключается к арендованному GPU-серверу через relay или напрямую; дашборд показывает клиентов, VRAM и загрузку.

### Этап 9 — RDMA → `v0.9.0`
- Транспорт ibverbs (RoCE v2/InfiniBand), регистрация памяти, zero-copy; GPUDirect RDMA на серверах, где он доступен (дата-центровые GPU).
- Функциональные тесты в CI — Soft-RoCE (rxe); перф — арендованные машины с RDMA.
- **Готово, когда:** латентность < 30 мкс p50 на RDMA-стенде.

### Этап 10 — Продукт и дистрибуция → `v1.0.0-rc.1`
- Установщики: `.deb`/`.rpm`/AUR/tarball/`curl | sh`, MSI (WiX)/winget/Scoop, Docker (GHCR + Hub), Helm, `pip install remora`.
- Подписи (cosign/Sigstore), SBOM, воспроизводимые сборки, автоматический release-pipeline.
- Трей-приложение (Windows/Linux), TUI `remora top`, `remora doctor`.
- Полная документация EN+RU: quickstart, гайды по каждому фреймворку, протокол, авто-таблица покрытия API, бенчмарки, безопасность.
- **Готово, когда:** с нуля на чистой Windows и чистой Ubuntu ставится одной командой и запускает PyTorch на удалённом GPU за < 5 минут.

### Этап 11 — Стабилизация → `v1.0.0`
- Заморозка протокола v1, тесты совместимости версий клиент/сервер.
- 72-часовой soak-тест (непрерывное обучение) без утечек, chaos-тесты сети (потери, задержки, обрывы).
- Security-чеклист, политика CVE, LTS-ветка 1.x, CHANGELOG, roadmap 2.x.
- RC-цикл: `rc.1` → `rc.N`, сбор фидбека, релиз **1.0.0**.


---

## Тестирование без своей видеокарты
- **Каждый PR:** GitHub Actions на Linux и Windows. Юнит-тесты, тесты протокола и кодогенерации, e2e против **мок-сервера** (fake GPU: эмулирует память, модули, запуски и библиотеки детерминированно). Это ~90% проверок, GPU для них не нужен.
- **Приёмка этапа и ночные прогоны:** скрипт `cargo xtask gpu-e2e` арендует GPU через API провайдера (ключ в секретах), разворачивает сервер, гоняет матрицу тестов, собирает отчёт и уничтожает инстанс. Ориентировочно $0.2–0.5/час за карту уровня 3090; один прогон приёмки — порядка 1–2 часов.
- **Особые конфигурации:** арендуются точечно. Multi-GPU/NCCL — этап 6, A100/H100 с MIG — этап 7, машины с RDMA — этап 9, пара машин с 10G — этап 5.
- До конца этапа 1 всё работает на моке.

## Трек 2.x — игры и графика (после 1.0)
Это **не CUDA**: нужен удалённый рендеринг графических API. Два подхода, выберем RFC в начале 2.x:
1. **Стриминг кадров** (как Moonlight/Sunshine/Parsec): игра идёт на сервере, на клиент приходит видео (NVENC → сеть → декодер), а обратно — ввод. Надёжно, работает с любыми играми, задержка 5–20 мс в LAN. Сюда же ляжет поддержка NVENC/NVDEC.
2. **API remoting для Vulkan/D3D** (команды рисования по сети): теоретически «настоящий удалённый GPU», но огромный трафик и задержки, античиты. Высокий риск, исследовательский режим.
Рекомендация: начать с (1), переиспользуя транспорт, авторизацию, координатор и дашборд Remora.

Прочее после 1.0: TensorRT, NVENC/NVDEC как CUDA-API, ROCm/HIP-бэкенд сервера (AMD), live-миграция сессий, чекпоинтинг GPU-состояния.

---
