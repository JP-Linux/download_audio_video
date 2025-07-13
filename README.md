# 🎵Universal Downloader

**Baixe áudio e vídeo de qualquer site!**  
Um utilitário simples e eficiente em Rust para download de conteúdo multimídia usando o poderoso yt-dlp como backend.

[![Rust](https://img.shields.io/badge/Made_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Arch Linux](https://img.shields.io/badge/For-Arch_Linux-blue?logo=arch-linux)](https://archlinux.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## ✨ Recursos

- ✅ Download de **áudio** ou **vídeo** de qualquer site suportado pelo yt-dlp
- 🎚️ Suporte a múltiplos formatos:
  - Áudio: MP3, Opus, FLAC, WAV
  - Vídeo: MP4, MKV, WEBM, AVI
- 🖼️ Adição automática de metadados e thumbnails
- 📁 Escolha de diretório de saída
- 📶 Seleção de qualidade customizável
- 🧩 Interface intuitiva passo-a-passo

## ⚙️ Pré-requisitos

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) instalado
- FFmpeg (para conversão de formatos)
- Rust 1.70+ (para compilação)

### Instalação de dependências no Arch Linux
```bash
sudo pacman -S yt-dlp ffmpeg
```

## 🚀 Instalação e Uso

### Passo 1: Clonar e compilar
```bash
git clone https://github.com/JP-Linux/download_audio_video.git
cd download_audio_video
cargo build --release
```

### Passo 2: Executar
```bash
./target/release/download_audio_video
```

### Passo 3: Seguir o fluxo interativo
1. Cole a URL do conteúdo
2. Escolha entre áudio ou vídeo
3. Selecione formato e qualidade
4. (Opcional) Especifique diretório de saída
5. Aguarde o download completar!

## 🖥️ Demonstração

```text
=== Universal Downloader ===

URL do conteúdo: https://www.youtube.com/watch?v=-BLPtkGw4ug

Tipo de download:
1) Somente Áudio
2) Vídeo + Áudio
> 2

Formato de vídeo:
1) MP4 (padrão)
2) MKV
3) WEBM
4) AVI
> 1

Qualidade (ex: best, 1080, 720, ou formato customizado): 1080

Diretório de saída (Enter para atual): /videos

Iniciando download...

[yt-dlp] Downloading video...
[ffmpeg] Merging formats...

✅ Download concluído com sucesso!
```

## 📁 Estrutura do Projeto
```
.
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
└── src
    └── main.rs
```

## 📜 Licença
Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---
Desenvolvido por [Jorge Paulo Santos](mailto:jorgepsan7@gmail.com)  
Suporte: [Arch Linux](https://archlinux.org/) | [Rust](https://www.rust-lang.org/) | [yt-dlp](https://github.com/yt-dlp/yt-dlp)