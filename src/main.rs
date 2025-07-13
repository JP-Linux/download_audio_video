use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    println!("=== Universal Downloader ===");

    // Verificar se o yt-dlp está instalado
    if Command::new("yt-dlp")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        eprintln!("Erro: yt-dlp não está instalado!");
        eprintln!("Instale com: sudo pacman -S yt-dlp");
        std::process::exit(1);
    }

    // Obter URL do usuário
    let url = loop {
        print!("\nURL do conteúdo: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let trimmed = input.trim();
        
        if !trimmed.is_empty() {
            break trimmed.to_string();
        }
        println!("URL não pode ser vazia!");
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
    cmd.arg("--no-playlist");  // Baixa apenas vídeo único
    cmd.arg("--add-metadata"); // Inclui metadados
    
    // Diretório de saída
    print!("\nDiretório de saída (Enter para atual): ");
    io::stdout().flush().unwrap();
    let mut output_dir = String::new();
    io::stdin().read_line(&mut output_dir).expect("Erro ao ler entrada");
    
    if !output_dir.trim().is_empty() {
        cmd.arg("-P").arg(output_dir.trim());
    }

    // Configurações específicas por tipo
    match download_type {
        "audio" => {
            // Permitir escolha de formato de áudio
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

            cmd.args(["-x", "--audio-format", audio_format]);
            cmd.arg("--embed-thumbnail"); // Adiciona thumbnail
        },
        "video" => {
            // Permitir escolha de formato de vídeo
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
            
            // Seleção de qualidade flexível
            let quality = loop {
                print!("\nQualidade (ex: best, 1080, 720, ou formato customizado): ");
                io::stdout().flush().unwrap();
                
                let mut qual_input = String::new();
                io::stdin().read_line(&mut qual_input).expect("Erro ao ler entrada");
                let qual_trimmed = qual_input.trim();
                
                if !qual_trimmed.is_empty() {
                    break qual_trimmed.to_string();
                }
                println!("Qualidade não pode ser vazia!");
            };

            // Usar formato customizado se fornecido
            cmd.args(["-f", &format!("bestvideo[height<={}]+bestaudio/best", quality)]);
        },
        _ => unreachable!(),
    };

    // Executar o download
    println!("\nIniciando download...");
    let mut child = cmd
        .arg(&url)
        .spawn()
        .expect("Falha ao executar comando");

    let status = child.wait().expect("Falha ao aguardar processo");

    if status.success() {
        println!("\n✅ Download concluído com sucesso!");
    } else {
        eprintln!("\n❌ Erro durante o download!");
        eprintln!("Código de saída: {}", status);
        std::process::exit(1);
    }
}