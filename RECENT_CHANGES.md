# Mudanças Recentes no Jaman

## Resumo das Melhorias Implementadas

### 🎯 1. Auto-adição ao PATH na Primeira Execução

**O que foi feito:**
- Jaman agora se adiciona automaticamente ao PATH do sistema na primeira vez que é executado
- Não é mais necessário configurar manualmente o PATH
- Funciona tanto no Windows quanto em sistemas Unix

**Arquivos modificados:**
- `src/path_manager.rs`: Adicionadas funções `add_jaman_to_path()` e `is_jaman_in_path()`
- `src/main.rs`: Adicionada função `ensure_jaman_in_path()` que é chamada no início de cada execução

**Como funciona:**
- No Windows: Adiciona o diretório do executável ao registro do Windows (HKEY_CURRENT_USER\Environment\Path)
- No Unix/Linux: Adiciona uma linha de export ao `.bashrc`, `.bash_profile`, `.zshrc` e `.profile`
- Verifica se já está no PATH antes de adicionar novamente

### 📁 2. Diretório de Download Configurável

**O que foi feito:**
- Adicionado campo `download_dir` na configuração
- Permite configurar onde os arquivos Java serão baixados
- Separação entre diretório de instalação e diretório de download

**Arquivos modificados:**
- `src/config.rs`: 
  - Adicionado campo `download_dir: PathBuf` na struct `Config`
  - Atualizado método `new()` para receber `download_dir`
  - Atualizado método `default_config()` para criar diretório de download padrão

**Configuração padrão:**
- Windows: `%LOCALAPPDATA%\jaman\downloads`
- Unix/Linux: `~/.local/share/jaman/downloads`

**Como usar:**
```bash
# Ver configuração atual
jaman config --show

# Configurar diretório de download customizado
jaman config --set-download-dir "D:\Java\Downloads"
```

### 🔍 3. Scan Melhorado - Busca em Todos os Discos

**O que foi feito:**
- Scan agora busca em **todos os discos disponíveis** no Windows (C:, D:, E:, etc.)
- Não se limita mais apenas ao disco C:
- Busca mais completa em todo o sistema

**Arquivos modificados:**
- `src/detector.rs`:
  - Adicionada função `get_available_drives()` (Windows only)
  - Modificada função `get_search_paths()` para iterar por todos os discos
  - Para cada disco, busca em:
    - `Program Files\Java`
    - `Program Files\Eclipse Adoptium`
    - `Program Files\Amazon Corretto`
    - `Program Files\Zulu`
    - `Program Files\BellSoft`
    - `Program Files\Microsoft`
    - `Program Files\GraalVM`
    - `Program Files\Azul`
    - `Program Files\Liberica`
    - E também em `Program Files (x86)` onde aplicável

### 🔎 4. Detecção via Comandos Java e PATH

**O que foi feito:**
- Adicionada detecção de Java através de comandos do sistema
- Usa `where java` no Windows e `which java` no Unix para encontrar Java no PATH
- Executa `java -version` para verificar se há Java instalado no sistema
- Verifica a variável de ambiente `JAVA_HOME`

**Arquivos modificados:**
- `src/detector.rs`:
  - Adicionada função `detect_from_path()` que:
    - Executa `where java` (Windows) ou `which java` (Unix)
    - Extrai o caminho do executável Java
    - Localiza o diretório raiz do JDK
    - Detecta a versão através de `java -version`
    - Verifica `JAVA_HOME` se disponível
  - Modificada função `scan_system()` para chamar `detect_from_path()` primeiro

**Ordem de detecção:**
1. Primeiro: Busca via PATH e comandos do sistema
2. Depois: Busca em diretórios de instalação comuns
3. Deduplicação: Remove entradas duplicadas pelo caminho

### 📝 5. Documentação Atualizada

**Arquivos atualizados:**
- `README.md`:
  - Atualizada seção de instalação explicando a auto-adição ao PATH
  - Atualizada seção do comando `config` com nova opção `--set-download-dir`
  - Atualizada seção do comando `scan` com novas funcionalidades

- `QUICKSTART.md`:
  - Adicionada nota sobre auto-adição ao PATH
  - Atualizada seção sobre comando scan

- `CHANGELOG.md`:
  - Adicionadas todas as novas funcionalidades na seção `[Unreleased]`

## Benefícios

### Para o Usuário:
✅ **Instalação mais fácil**: Não precisa configurar PATH manualmente
✅ **Detecção mais completa**: Encontra Java em qualquer lugar do sistema
✅ **Mais flexibilidade**: Pode configurar onde baixar e instalar
✅ **Melhor experiência**: Funciona "out of the box"

### Para o Sistema:
✅ **Mais robusto**: Detecta Java de múltiplas formas
✅ **Mais inteligente**: Verifica PATH antes de varrer diretórios
✅ **Mais rápido**: Deduplicação evita processamento redundante

## Como Testar

### 1. Testar Auto-adição ao PATH
```bash
# Primeiro, remova jaman do PATH se já estiver lá
# Depois execute:
jaman --help

# Abra um novo terminal e teste:
jaman --version
```

### 2. Testar Configuração de Download Dir
```bash
# Ver configuração atual
jaman config --show

# Configurar novo diretório
jaman config --set-download-dir "D:\Temp\JavaDownloads"

# Verificar mudança
jaman config --show
```

### 3. Testar Scan Melhorado
```bash
# Execute o scan
jaman scan

# Deve encontrar Java em:
# - Todos os discos (não só C:)
# - PATH do sistema
# - Instalações detectadas por 'where java'
```

## Compatibilidade

✅ Windows 10/11
✅ macOS (Intel e Apple Silicon)
✅ Linux (x64 e ARM64)

## Próximos Passos Sugeridos

1. **Testar compilação**: `cargo build --release`
2. **Testar funcionalidades**: Executar o executável e testar cada comando
3. **Atualizar versão**: Considerar incrementar para `0.2.0` no `Cargo.toml`
4. **Criar release**: Gerar binários para todas as plataformas
5. **Atualizar instaladores**: Garantir que `install.ps1` e `install.sh` funcionem

## Notas Técnicas

### Segurança
- Modificação do PATH é feita apenas para o usuário atual (não requer admin)
- No Windows, usa `HKEY_CURRENT_USER` em vez de `HKEY_LOCAL_MACHINE`
- Verifica existência antes de adicionar para evitar duplicatas

### Performance
- Detecção via PATH é feita primeiro (mais rápido)
- Deduplicação por caminho evita processar mesmo JDK múltiplas vezes
- Scan de diretórios tem profundidade máxima de 3 níveis

### Manutenibilidade
- Código modularizado em funções específicas
- Suporte a Windows e Unix separado com `#[cfg()]`
- Funções bem documentadas com comentários
