use maud::{Markup, PreEscaped, html};

pub fn render_text_with_links(s: &str) -> Markup {
    let mut out = String::new();
    let mut i = 0;
    let bytes = s.as_bytes();

    while let Some(rel_start) = s[i..].find('[') {
        let start = i + rel_start;
        escape_html_text(&s[i..start], &mut out);

        let rel_close = match s[start + 1..].find(']') {
            Some(v) => v,
            None => {
                escape_html_text(&s[start..], &mut out);
                return html! { (PreEscaped(out)) };
            }
        };
        let close = start + 1 + rel_close;
        let after_close = close + 1;
        if after_close >= bytes.len() || bytes[after_close] != b'(' {
            escape_html_text(&s[start..=start], &mut out);
            i = start + 1;
            continue;
        }
        let rel_paren = match s[after_close + 1..].find(')') {
            Some(v) => v,
            None => {
                escape_html_text(&s[start..], &mut out);
                return html! { (PreEscaped(out)) };
            }
        };
        let close_paren = after_close + 1 + rel_paren;

        let text = &s[start + 1..close];
        let url = &s[after_close + 1..close_paren];
        let url_trim = url.trim();

        if text.trim().is_empty() || !is_safe_url(url_trim) {
            escape_html_text(&s[start..=close_paren], &mut out);
            i = close_paren + 1;
            continue;
        }

        out.push_str("<a href=\"");
        escape_html_attr(url_trim, &mut out);
        out.push_str("\">");
        escape_html_text(text, &mut out);
        out.push_str("</a>");

        i = close_paren + 1;
    }

    escape_html_text(&s[i..], &mut out);
    html! { (PreEscaped(out)) }
}

fn is_safe_url(url: &str) -> bool {
    if url.is_empty() {
        return false;
    }

    let lower = url.to_ascii_lowercase();
    if lower.starts_with("javascript:") || lower.starts_with("data:") {
        return false;
    }

    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("mailto:")
        || url.starts_with('/')
        || url.starts_with("./")
        || url.starts_with("../")
        || url.starts_with('#')
    {
        return true;
    }

    !url.contains(':')
}

fn escape_html_text(s: &str, out: &mut String) {
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
}

fn escape_html_attr(s: &str, out: &mut String) {
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_text_with_links_parses_markdown_link() {
        let html = render_text_with_links("See [Docs](https://example.com).").into_string();
        assert!(html.contains("See "));
        assert!(html.contains("<a href=\"https://example.com\">Docs</a>"));
        assert!(html.contains("."));
    }

    #[test]
    fn render_text_with_links_rejects_javascript_url() {
        let html = render_text_with_links("See [Docs](javascript:alert(1)).").into_string();
        assert!(html.contains("[Docs](javascript:alert(1))"));
        assert!(!html.contains("<a href="));
    }

    #[test]
    fn render_text_with_links_preserves_plain_text() {
        let html = render_text_with_links("No links here.").into_string();
        assert!(html.contains("No links here."));
        assert!(!html.contains("<a href="));
    }
}
