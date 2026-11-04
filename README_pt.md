# CMR — Claude Model Router

**Roteamento automático de Haiku, Sonnet e Opus no Claude Code**, com uma CLI nativa em Rust para Windows, macOS e Linux.

[![CI](https://github.com/Weslley08/cmr/actions/workflows/ci.yml/badge.svg)](https://github.com/Weslley08/cmr/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](README.md) · [Instalação](INSTALL.md) · [Arquitetura](docs/contributing/ARCHITECTURE.md)

> **Código experimental 0.2.0:** o Mod utiliza a API oficial `turn.step` para modificar o modelo das requisições principais do Claude Code. Ainda precisamos verificar a integração em uma sessão real; não há publicação WinGet/Homebrew.

## Como funciona

1. Você envia uma tarefa pelo Claude Code (VS Code, CLI ou outro ambiente local compatível).
2. O Mod classifica localmente a complexidade da tarefa, sem chamada adicional a IA.
3. Antes de cada requisição do turno principal, aplica Haiku, Sonnet ou Opus via `turn.step`.
4. Se Claude utilizar Bash, **RTK** continua compactando os comandos de maneira independente.

O CMR **não usa um serviço intermediário**, não modifica sua autenticação nem troca o modelo dos subagentes. O seletor visual da IDE pode continuar indicando o modelo padrão mesmo quando a requisição efetiva for roteada.

| Perfil | Simples | Implementação | Arquitetura | Crítico |
| --- | --- | --- | --- | --- |
| economy | Haiku | Sonnet | Sonnet | Opus |
| balanced | Haiku | Sonnet | Opus | Opus |
| quality | Sonnet | Opus | Opus | Opus |

## Instalar diretamente no VS Code

Na extensão **Claude Code** abra `/plugin` e execute:

```text
/plugin marketplace add Weslley08/cmr
/plugin install model-router@cmr
```

Selecione escopo **user**, para disponibilizar em todos os projetos locais. Depois execute `/reload-plugins` ou reinicie a sessão e confirme com:

```text
/cmr status
/cmr mode balanced
/cmr auto
```

O Mod inicia com roteamento ativado após ser carregado; `/cmr off` o desativa na sessão.

**Marketplace** é apenas o catálogo do GitHub utilizado pelo gerenciador oficial do Claude Code; não é uma loja ou extensão separada do VS Code.

## CLI no estilo RTK (opcional)

```powershell
git clone https://github.com/Weslley08/cmr.git
cd cmr
cargo test --all-targets
cargo build --release
```

Com `target/release/` no PATH, você poderá executar `cmr init -g` para instalar o Mod pela CLI oficial do Claude Code. Se o comando `claude` não estiver disponível, a CLI exibirá as instruções de instalação pelo VS Code, sem fingir que conseguiu instalar.

O antigo hook que somente recomendava modelos continua disponível por compatibilidade, mas não é mais o mecanismo de instalação padrão.

## Comandos do Mod

```text
/cmr status
/cmr auto
/cmr off
/cmr mode economy
/cmr mode balanced
/cmr mode quality
/cmr force opus
/cmr force none
/cmr stats
```

### Estado

**Ainda não há release binário público nem pacote WinGet aprovado.** É necessário validar o Mod na sua extensão Claude Code 2.1.318 e conferir quais modelos responderam às requisições. Os números de uso não garantem economia total de tokens.

Licença [MIT](LICENSE).
