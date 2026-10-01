---
layout: home
hero:
  name: Remora
  text: Удалённая CUDA — прозрачно
  tagline: Запускайте PyTorch, llama.cpp, Blender и свой CUDA-код на GPU по сети. Linux и Windows.
  actions:
    - theme: brand
      text: Архитектура
      link: /ru/architecture
    - theme: alt
      text: Дорожная карта
      link: /ru/roadmap
features:
  - title: Без изменения кода
    details: Замена libcuda / nvcuda.dll, cuBLAS, cuDNN, NVML и других библиотек. Приложение видит удалённые GPU как локальные.
  - title: Скорость в основе
    details: Асинхронные батчи вызовов, кэш модулей, конвейерные копирования; транспорты TCP, QUIC, shared memory и RDMA.
  - title: Безопасная мультиарендность
    details: TLS 1.3, токены и mTLS, изолированные воркеры на клиента, квоты VRAM и времени, аудит.
  - title: Готово к эксплуатации
    details: Координатор GPU-пулов, обход NAT, метрики Prometheus, веб-панель и трассировка вызовов.
---
