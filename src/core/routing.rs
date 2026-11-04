//! Model recommendations are intentionally NOT model switches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recommendation {
    pub model: &'static str,
    pub reason: &'static str,
}

fn normalize(input: &str) -> String {
    input.to_lowercase().chars().map(|c| match c {
        'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
        'é' | 'ê' | 'è' | 'ë' => 'e',
        'í' | 'î' | 'ì' | 'ï' => 'i',
        'ó' | 'ô' | 'õ' | 'ò' | 'ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        other => other,
    }).collect()
}

fn contains_any(text: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|term| text.contains(term))
}

pub fn recommend(prompt: &str) -> Recommendation {
    let text = normalize(prompt);
    if contains_any(&text, &["use haiku", "modelo haiku", "use o haiku"]) {
        return Recommendation { model: "haiku", reason: "explicit user preference" };
    }
    if contains_any(&text, &["use sonnet", "modelo sonnet", "use o sonnet"]) {
        return Recommendation { model: "sonnet", reason: "explicit user preference" };
    }
    if contains_any(&text, &["use opus", "modelo opus", "use o opus"]) {
        return Recommendation { model: "opus", reason: "explicit user preference" };
    }
    if contains_any(&text, &[
        "seguranca", "security", "producao", "production", "incidente",
        "incident", "deadlock", "race condition", "vazamento", "auth",
        "autenticacao", "autorizacao", "critical", "critico",
    ]) {
        return Recommendation { model: "opus", reason: "high-risk or critical task" };
    }
    if contains_any(&text, &[
        "arquitetur", "architecture", "distribuid", "distributed",
        "redesenh", "system design", "refatore tudo", "refactor entire",
        "escalabilidade", "consistencia", "multi-tenant",
    ]) {
        return Recommendation { model: "opus", reason: "complex system design" };
    }
    let simple = ["liste", "listar", "mostre", "mostrar", "onde", "encontre",
        "localize", "list ", "show ", "where ", "find "];
    let scoped = ["arquivo", "file", "pasta", "folder", "diretorio",
        "readme", "branch", "commit", "git status", "path", "caminho"];
    let starts_simple = simple.iter().any(|p| text.starts_with(p));
    if text.len() <= 240 && starts_simple && contains_any(&text, &scoped) {
        return Recommendation { model: "haiku", reason: "scoped read-only task" };
    }
    Recommendation { model: "sonnet", reason: "conservative development default" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recognizes_complexity() {
        assert_eq!(recommend("Liste arquivos do src").model, "haiku");
        assert_eq!(recommend("Implemente um endpoint com testes").model, "sonnet");
        assert_eq!(recommend("Refatore a arquitetura distribuida").model, "opus");
        assert_eq!(recommend("Corrija um deadlock em produção").model, "opus");
        assert_eq!(recommend("Explique o funcionamento de uma API").model, "sonnet");
        assert_eq!(recommend("Use o Haiku para localizar arquivos").model, "haiku");
    }
}
