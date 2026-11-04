# CMR — Claude Model Router

<p align="center"><strong>Roteamento inteligente de modelos para Claude Code, com regras locais e ativação opcional.</strong></p>

[![CI](https://github.com/Weslley08/cmr/actions/workflows/ci.yml/badge.svg)](https://github.com/Weslley08/cmr/actions/workflows/ci.yml) [![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](README.md) · [Instalação](INSTALL.md) · [Guia rápido](docs/guide/getting-started/quick-start.md) · [Arquitetura](docs/contributing/ARCHITECTURE.md)

> **Versão 0.1.0 experimental.** Os testes locais e CI foram preparados, mas ainda falta comprovar a troca real de modelos no VS Code. **WinGet e Homebrew ainda não distribuem o CMR.**

## Objetivo

O CMR usa um componente de extensão do **Claude Code** para selecionar **Haiku, Sonnet ou Opus**, conforme a complexidade da tarefa. A CLI em Rust gerencia instalação e diagnóstico. O roteamento inicia **desativado**, até executar `/cmr auto`.

| Tipo de tarefa | Modelo no perfil equilibrado |
|---|---|
| Consulta simples, localizada | Haiku |
| Implementação e testes comuns | Sonnet |
| Arquitetura e alterações amplas | Opus |
| Segurança ou incidente de produção | Opus |

A classificação utiliza heurísticas locais. Economia de tokens, escalonamento automático em caso de erro e funcionamento no ambiente real ainda não foram demonstrados.

## Instalação

Depois que o pacote for publicado, queremos oferecer:

```powershell
winget install Weslley08.CMR
cmr init -g
cmr doctor
```

**Esses comandos ainda não estão disponíveis no WinGet.**

Para testar o Mod agora: no gerenciador de plugins do **Claude Code**, adicione a fonte `Weslley08/cmr` e instale `model-router@weslley08-cmr` no escopo de usuário. O chamado *marketplace* é apenas um catálogo de plugins do Claude Code — não é o Marketplace do VS Code.

```text
/cmr status
/cmr mode balanced
/cmr auto
/cmr stats
/cmr off
```

Veja [INSTALL.md](INSTALL.md) para mais opções. A [documentação principal em inglês](README.md) apresenta a estrutura completa.

## RTK e CMR

**RTK** compacta saídas dos comandos; **CMR** seleciona o modelo. São independentes e podem funcionar juntos. CMR não modifica os hooks do RTK nem suas permissões.

Código sob licença [MIT](LICENSE).
