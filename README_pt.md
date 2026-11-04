# CMR — Claude Model Router

**Um executável nativo em Rust, sem plugin ou marketplace.** Funciona com hooks locais do Claude Code no terminal e na extensão oficial do VS Code.

[![CI](https://github.com/Weslley08/cmr/actions/workflows/ci.yml/badge.svg)](https://github.com/Weslley08/cmr/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](README.md) · [Instalação](INSTALL.md) · [Arquitetura](docs/contributing/ARCHITECTURE.md)

## CMR + RTK

- **Primeiro o CMR:** o hook `UserPromptSubmit` recebe a tarefa e recomenda Haiku, Sonnet ou Opus.
- **Depois o Claude Code:** processa a solicitação com o modelo efetivamente selecionado na interface.
- **Se executar Bash, entra o RTK:** o hook `PreToolUse` intercepta os comandos compatíveis e compacta suas saídas.

Ambos funcionam independentemente, sem sobrescrever os hooks existentes.

### Limitação importante

**Os hooks externos do Claude Code não podem forçar uma troca automática de modelo por prompt.** O CMR indica qual modelo recomenda, mas isso não garante que ele foi utilizado. Para uma troca efetiva, use o seletor de modelos do VS Code, `/model` ou o comando `cmr launch`, que executa `claude --model ...` ao iniciar uma nova sessão no terminal.

Não utilizaremos proxy, API privada, alteração de credenciais nem integração obrigatória com plugin.

## Comandos

```powershell
cmr init -g --dry-run
cmr init -g
cmr doctor
cmr status
cmr recommend "Liste arquivos do src"
cmr launch "Implemente um endpoint com testes"
cmr uninstall -g
```

O `cmr init -g` só adiciona `cmr hook prompt` ao evento `UserPromptSubmit` do `~/.claude/settings.json`. Não altera o `rtk hook claude` em `PreToolUse`.

## Instalação

**Versão experimental 0.1.0**: ainda não há pacote oficial no WinGet/Homebrew. Até a primeira publicação, é necessário compilar a CLI em Rust ou usar os artefatos de CI para testes. O plano de distribuição é:

```powershell
winget install Weslley08.CMR  # Planejado, ainda não disponível
cmr init -g
```

Confira [INSTALL.md](INSTALL.md). Licença [MIT](LICENSE).
