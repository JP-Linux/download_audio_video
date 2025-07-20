use std::process::{Command, Stdio, exit};
use std::io::{self, Write};

fn main() {
    println!("=== Universal Downloader ===");

    // Verificação portável do yt-dlp
    if Command::new("which")
        .arg("yt-dlp")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        eprintln!("Erro: yt-dlp não está instalado!");
        eprintln!("Instale com:");
        eprintln!("  Linux (Debian/Ubuntu): sudo apt install yt-dlp");
        eprintln!("  Linux (Arch): sudo pacman -S yt-dlp");
        eprintln!("  macOS: brew install yt-dlp");
        eprintln!("  Windows: winget install yt-dlp");
        exit(1);
    }

    // Obter URL (com validação básica)
    let url = loop {
        print!("\nURL do conteúdo: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let trimmed = input.trim();
        
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            break trimmed.to_string();
        }
        println!("URL inválida! Deve começar com http:// ou https://");
    };

    // Selecionar tipo de download
    let download_type = loop {
        print!("\nTipo de download:\n1) Somente Áudio\n2) Vídeo + Áudio\n> ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro ao ler entrada");
        
        match choice.trim() {
            "1" => break "audio",
            "2" => break "video",
            _ => println!("Opção inválida! Digite 1 ou 2"),
        }
    };

    // Configurar parâmetros do yt-dlp
    let mut cmd = Command::new("yt-dlp");
    
    // Configurações gerais
    cmd.arg("--no-playlist")
       .arg("--add-metadata")
       .arg("--newline");  // Mostra progresso em novas linhas
    
    // Diretório de saída
    print!("\nDiretório de saída (Enter para atual): ");
    io::stdout().flush().unwrap();
    let mut output_dir = String::new();
    io::stdin().read_line(&mut output_dir).expect("Erro ao ler entrada");
    
    if !output_dir.trim().is_empty() {
        cmd.arg("-o").arg(format!("{}/%(title)s.%(ext)s", output_dir.trim()));
    }

    // Configurações específicas por tipo
    match download_type {
        "audio" => {
            let audio_format = loop {
                print!("\nFormato de áudio:\n1) MP3 (padrão)\n2) Opus\n3) FLAC\n4) WAV\n> ");
                io::stdout().flush().unwrap();
                
                let mut format_choice = String::new();
                io::stdin().read_line(&mut format_choice).expect("Erro ao ler entrada");
                
                match format_choice.trim() {
                    "1" | "" => break "mp3",
                    "2" => break "opus",
                    "3" => break "flac",
                    "4" => break "wav",
                    _ => println!("Opção inválida! Usando MP3 como padrão"),
                }
            };

            cmd.args(["-x", "--audio-format", audio_format])
               .arg("--embed-thumbnail");
        },
        "video" => {
            let video_format = loop {
                print!("\nFormato de vídeo:\n1) MP4 (padrão)\n2) MKV\n3) WEBM\n4) AVI\n> ");
                io::stdout().flush().unwrap();
                
                let mut format_choice = String::new();
                io::stdin().read_line(&mut format_choice).expect("Erro ao ler entrada");
                
                match format_choice.trim() {
                    "1" | "" => break "mp4",
                    "2" => break "mkv",
                    "3" => break "webm",
                    "4" => break "avi",
                    _ => println!("Opção inválida! Usando MP4 como padrão"),
                }
            };

            cmd.arg("--merge-output-format").arg(video_format);
            
            // Correção crítica: seleção de qualidade
            let quality = loop {
                print!("\nQualidade (ex: 1080, 720, 480 ou 'best'): ");
                io::stdout().flush().unwrap();
                
                let mut qual_input = String::new();
                io::stdin().read_line(&mut qual_input).expect("Erro ao ler entrada");
                let qual_trimmed = qual_input.trim();
                
                if qual_trimmed.is_empty() {
                    println!("Usando qualidade padrão (1080p)");
                    break "1080".to_string();
                }
                
                if qual_trimmed.parse::<u32>().is_ok() || qual_trimmed == "best" {
                    break qual_trimmed.to_string();
                }
                println!("Qualidade inválida! Use números (720) ou 'best'");
            };

            // Formato corrigido para altura máxima
            if quality != "best" {
                cmd.args(["-S", &format!("res:{}", quality)]);
            }
        },
        _ => unreachable!(),
    };

    // Executar o download com tratamento de Ctrl+C
    println!("\nIniciando download... (Ctrl+C para cancelar)");
    cmd.arg(&url);
    
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Erro ao executar: {}", e);
            exit(1);
        }
    };

    if status.success() {
        println!("\n✅ Download concluído com sucesso!");
    } else {
        eprintln!("\n❌ Erro durante o download! Código: {}", status);
        exit(1);
    }
}