<div align="center">

<p align="center">
  <a href="../../README.md">🇺🇸 English</a> · <a href="./ko.md">🇰🇷 한국어</a> · <a href="./zh-CN.md">🇨🇳 中文</a> · <a href="./ja.md">🇯🇵 日本語</a> · <a href="./es.md">🇪🇸 Español</a><br />
  <a href="./ru.md">🇷🇺 Русский</a> · <a href="./vi.md">🇻🇳 Tiếng Việt</a> · <a href="./tr.md">🇹🇷 Türkçe</a> · <a href="./de.md">🇩🇪 Deutsch</a> · <strong>🇧🇷 Português</strong>
</p>

# rdl-tray

[![Build](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/build.yml)
[![Tests](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml/badge.svg)](https://github.com/GHSFS/rdl-tray/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue)](#installation)

</div>

---

### Visão geral

`rdl-tray.exe` é um pequeno aplicativo nativo do Windows que faz par com o
cliente CLI [`rdl`](https://github.com/GHSFS/remote-dl). Enquanto a CLI lida
com invocações explícitas `rdl <url>` no terminal, o `rdl-tray` cobre o caso
ambiente: sempre que você copia uma URL para a área de transferência, uma
notificação pergunta se deseja baixá-la para seu armazenamento em nuvem.

Ambos os binários compartilham o mesmo arquivo de configuração em disco
(`%APPDATA%\rdl\config.json`), portanto autenticar-se uma vez pela CLI já é
suficiente para que o tray também funcione.

### Recursos

- **Detecção de URL e links Magnet** — escuta via `AddClipboardFormatListener`
  e extrai o primeiro link `http(s)://` ou `magnet:?xt=...`
- **Clique interativo no balão** — clique na notificação (`NIN_BALLOONUSERCLICK`)
  para despachar imediatamente o download
- **Enfileiramento assíncrono sem travamentos** — thread em segundo plano garante
  zero congelamentos na interface do Windows
- **Proteção contra múltiplas instâncias (Named Mutex)** — impede processos
  duplicados e conflitos de listeners
- **Iniciar com o Windows** — alterne o início automático no logon diretamente pelo menu
  (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`)
- **Pool de conexões HTTP** — cliente em cache (`OnceLock`) elimina o overhead de handshake TLS
- **Ícone nativo Win32 na bandeja** — `Shell_NotifyIconW` + menu de clique direito
- **Notificações em balão** — solicitações de confirmação, IDs de trabalho e alertas de erro
- **Pausar / retomar** — alternar o listener pelo menu da bandeja sem sair
- **Reenviar a última URL** — recolocar a URL copiada anteriormente na fila com um clique
- **Binário único** — linkado estaticamente, ~1.5 MB, sem dependências de DLL

### Estrutura do projeto

```
rdl-tray/
├── Cargo.toml              Manifesto do pacote, dependências, perfil release
├── rust-toolchain.toml     Fixação do toolchain (stable / x86_64-pc-windows-msvc)
├── .cargo/config.toml      Alvo de build + flags do linker (CRT estática)
├── build.rs                Embute o manifesto Win32 + informação de versão
├── src/
│   ├── main.rs             Janela oculta + loop de mensagens Win32, clique no balão e filas async
│   ├── autostart.rs        Gerenciamento do registro de inicialização do Windows
│   ├── tray.rs             Gerenciamento do ciclo de vida do Shell_NotifyIconW
│   ├── clipboard.rs        AddClipboardFormatListener + extração segura de URL/Magnet
│   ├── notify.rs           Notificações em balão via NIM_MODIFY (info / erro)
│   ├── client.rs           Cliente reqwest com pool de conexões, POST /api/dl
│   ├── config.rs           Carga de configuração + descriptografia DPAPI do token
│   └── error.rs            Tipo de error do crate
├── tests/integration.rs    Testes smoke baseados em assert_cmd
├── resources/              Script de recursos Win32 + manifesto
└── .github/workflows/      CI para cargo build + fmt/clippy/test
```

Para instruções completas de instalação, configuração e referência da CLI,
consulte a seção [English](#english).

### Licença

MIT. Veja [LICENSE](./LICENSE).
