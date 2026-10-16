<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <strong>🇹🇷 Türkçe</strong> · <a href="./de.md">🇩🇪 Deutsch</a> · <a href="./pt-BR.md">🇧🇷 Português</a>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Genel Bakış

`rdl-tray.exe`, [`rdl`](https://github.com/GHSFS/remote-dl) CLI istemcisiyle
eşleşen küçük yerel bir Windows uygulamasıdır. CLI, terminalden açık `rdl
<url>` çağrılarını işlerken, `rdl-tray` arka plan otomasyonunu sağlar —
panoya bir URL kopyaladığınızda, onu bulut depolamanıza indirmek isteyip
istemediğinizi soran bir bildirim gösterir.

Her iki ikili de aynı disk yapılandırma dosyasını
(`%APPDATA%\rdl\config.json`) paylaşır, bu nedenle CLI üzerinden bir kez
kimlik doğrulaması yapmak tepsi için de yeterlidir.

### Özellikler

- **Pano URL & Magnet algılama** — `AddClipboardFormatListener` ile değişiklikleri
  dinler ve ilk `http(s)://` veya `magnet:?xt=...` bağlantısını çıkarır
- **Etkileşimli balon tıklaması** — bildirime tıklayarak (`NIN_BALLOONUSERCLICK`)
  indirme görevini anında kuyruğa gönderin
- **Arayüzü dondurmayan asenkron kuyruk** — arka plan iş parçacığı sayesinde
  ağ gecikmelerinde tepsi asla donmaz
- **Çoklu başlatma koruması (Named Mutex)** — yinelenen simgeleri ve çakışan
  pano dinleyicilerini engeller
- **Windows ile başlatma** — tepsi sağ tık menüsünden başlangıç kaydını
  kolayca açıp kapatın (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **HTTP bağlantı havuzu** — `OnceLock` ile TLS el sıkışma gecikmesini sıfıra indirir
- **Yerel Win32 sistem tepsisi simgesi** — `Shell_NotifyIconW` + sağ tık menüsü
- **Balon bildirimleri** — onay istemleri, iş kimlikleri ve hata bildirimleri
  Windows Eylem Merkezi'nde gösterilir
- **Duraklat / devam ettir** — uygulamayı kapatmadan tepsi menüsünden
  dinleyiciyi açıp kapatın
- **Son URL'yi tekrar gönder** — son kopyalanan URL'yi tek tıkla yeniden
  kuyruğa alın
- **Tek ikili dosya** — statik bağlı, ~1.5 MB, DLL bağımlılığı yok

### Proje yapısı

```
rdl-tray/
├── Cargo.toml              Paket manifesti, bağımlılıklar, release profili
├── rust-toolchain.toml     Toolchain sabitleme (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Derleme hedefi + bağlayıcı bayrakları (statik CRT)
├── build.rs                Win32 manifest + sürüm bilgisi gömme
├── src/
│   ├── main.rs             Gizli pencere + Win32 mesaj döngüsü, bildirim tıklama ve asenkron kuyruk
│   ├── autostart.rs        Windows başlangıç kayıt defteri yönetimi
│   ├── tray.rs             Shell_NotifyIconW yaşam döngüsü yönetimi
│   ├── clipboard.rs        AddClipboardFormatListener + güvenli URL/Magnet çıkarma
│   ├── notify.rs           NIM_MODIFY ile bilgi/hata balon bildirimleri
│   ├── client.rs           Havuzlanmış reqwest istemcisi, POST /api/dl
│   ├── config.rs           Yapılandırma yükleme + DPAPI ile token çözme
│   └── error.rs            Crate genelinde hata türü
├── tests/integration.rs    assert_cmd tabanlı smoke testler
├── resources/              Win32 resource script + manifest
└── .github/workflows/      cargo build + fmt/clippy/test için CI
```

Ayrıntılı kurulum, yapılandırma ve CLI başvurusu için [English](#english)
bölümüne bakın.

### Lisans

MIT. [LICENSE](./LICENSE) dosyasına bakın.
