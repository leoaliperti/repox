# repox ⚡

`repox` è uno strumento da riga di comando (CLI) estremamente veloce e nativo, sviluppato in Rust. È progettato specificamente per impacchettare l'intero contesto di una cartella o di un progetto locale in un unico file di testo o direttamente negli appunti di sistema, pronto per essere inviato ai Large Language Models (LLM) come ChatGPT, Claude o Gemini.

A differenza dei tool web che richiedono una repository GitHub pubblica, `repox` opera interamente offline sul tuo computer, garantendo la totale privacy del tuo codice e offrendo opzioni di ottimizzazione avanzate per ridurre al minimo lo spreco di token.

---

## ✨ Funzionalità principali

* 🚀 **Velocità e Sicurezza (Rust):** Analizza ed elabora migliaia di file in frazioni di secondo grazie alle prestazioni native e alla gestione sicura della memoria di Rust.
* 📂 **Rispetto Nativo del `.gitignore`:** Sfrutta le stesse librerie industriali alla base di *ripgrep* per escludere automaticamente file nascosti, file ignorati dal controllo di versione (come `node_modules`, `target`, `build`, `.git`) o configurazioni sensibili (come file `.env`).
* 🧹 **Ottimizzazione dei Token (Token Saver):** Rimuove di default tutte le righe vuote o composte solo da spazi dal codice sorgente. Questo riduce la dimensione del prompt del 20-30% senza intaccare l'indentazione logica del codice, preservandone la perfetta leggibilità per l'AI.
* 📋 **Copia Diretta negli Appunti (Clipboard Mode):** Tramite il flag `-c`, il contesto formattato viene copiato direttamente nella memoria di sistema. Non c'è bisogno di aprire o creare file fisici sulla scrivania: basta lanciare il comando e premere `Incolla` (`Ctrl+V` o `Cmd+V`) nella chat dell'AI.
* ⚙️ **Rilevamento Binario Intelligente:** Analizza i primi byte di ogni file prima di leggerlo. Se rileva un formato binario (immagini, audio, video, PDF o file compilati), lo scarta automaticamente evitando di corrompere l'output testuale.
* 📐 **Struttura ad Albero Professionale:** Genera una mappa gerarchica del progetto testuale, pulita, priva di emoji e ottimizzata per i parser delle AI.
* 📊 **Statistiche del Prompt:** Mostra un resoconto dettagliato nel terminale al termine dell'esecuzione, indicando il numero di file letti, i caratteri totali e una stima accurata dei token effettivi generati.

---

## 🛠️ Installazione

Assicurati di avere [Rust e Cargo installati](https://www.rust-lang.org/tools/install) sul tuo sistema.

1.  Clona questa repository o copia i file sorgente sul tuo computer:
    ```bash
    git clone [https://github.com/tuo-username/repox.git](https://github.com/tuo-username/repox.git)
    cd repox
    ```

2.  Compila e installa il tool globalmente sul tuo sistema operativo tramite Cargo:
    ```bash
    cargo install --path .
    ```

> **Nota:** Assicurati che la cartella dei binari di Cargo (solitamente `~/.cargo/bin` su Unix/macOS o `%USERPROFILE%\.cargo\bin` su Windows) sia presente nel PATH del tuo terminale.

---

## 🚀 Guida all'uso

Una volta installato, puoi invocare `repox` da **qualsiasi cartella** del tuo computer. Naviga nel tuo progetto ed esegui uno dei seguenti comandi:

### 1. Utilizzo Standard (Genera File)
```bash
repox

```

Genera un file fisico chiamato `ai_project_context.md` nella cartella corrente, strutturato con l'albero del progetto e il codice sorgente di tutti i file di testo.

### 2. Copia Rapida (Consigliato)

```bash
repox -c

```

Analizza tutto il progetto, ottimizza le righe vuote e **salva tutto negli appunti**. Non crea nessun file fisico nella cartella. Ti basta andare sul browser o sull'app dell'AI e incollare.

### 3. Esclusioni Personalizzate

Se vuoi escludere file pesanti o inutili per la comprensione del codice (come i file di lock delle dipendenze):

```bash
repox -c -e package-lock.json,cargo.lock,pnpm-lock.yaml

```

### 4. Mantenere le Righe Vuote

Se per motivi di formattazione o di stile hai assoluto bisogno di preservare la struttura vuota originale del codice:

```bash
repox --keep-empty-lines

```

---

## 📋 Opzioni e Flag (CLI Options)

Puoi scoprire tutti i comandi disponibili digitando `repox --help`. Di seguito la tabella riassuntiva:

| Flag / Opzione | Alternativa Estesa | Descrizione | Default |
| --- | --- | --- | --- |
| `-c` | `--clipboard` | Copia l'output direttamente negli appunti invece di salvare il file. | `false` |
| `-e <valori>` | `--exclude <valori>` | Esclude file o cartelle personalizzate (separati da virgola). | Nessuno |
| `-o <file>` | `--output <file>` | Specifica un nome personalizzato per il file generato. | `ai_project_context.md` |
| `-m <num>` | `--max-size-kb <num>` | Dimensione massima in KB per includere un singolo file. | `500` |
|  | `--keep-empty-lines` | Disattiva l'ottimizzazione e mantiene le righe vuote nel codice. | `false` |
|  | `--no-gitignore` | Ignora le regole dei file `.gitignore` locali e globali. | `false` |
| `-h` | `--help` | Mostra la schermata di aiuto con le istruzioni. | - |
| `-V` | `--version` | Mostra la versione corrente di repox. | - |

---

---

## 📂 Esempio di Output Generato

L'output (sia nel file che nella clipboard) è formattato in **Markdown Nativo**, lo standard preferito dalle AI.

Immaginiamo un progetto demo chiamato `repox_demo`. Di seguito, come appare il file generato da `repox`. Nota come nel codice sorgente **sono state rimosse tutte le righe vuote** originali per risparmiare spazio, mantenendo però l'indentazione.

````markdown
# PROJECT STRUCTURE

```text
repox_demo/
  src/
    main.rs
    utils.rs
  frontend/
    config.ts
    types.ts
  Cargo.toml
  README.md
```

# FILE CONTENTS

## File: `Cargo.toml`
```toml
[package]
name = "repox_demo"
version = "0.1.0"
edition = "2021"
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## File: `config.ts`
```typescript
interface AppConfig {
  apiUrl: string;
  timeout: number;
}
const config: AppConfig = {
  apiUrl: "https://api.example.com",
  timeout: 5000,
};
export default config;
```

## File: `src/main.rs`
```rust
mod utils;
fn main() {
    println!("Starting Repox Demo...");
    // Initialize configuration
    let config = utils::load_config();
    if config.is_ok() {
        println!("Config loaded successfully.");
    } else {
        eprintln!("Error loading config.");
    }
}
```

## File: `src/utils.rs`
```rust
use std::io;
pub fn load_config() -> io::Result<()> {
    // Simulating config load logic
    println!("Loading configuration from file...");
    Ok(())
}
```