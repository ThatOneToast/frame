use frame_core::{Diagnostic, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedFrameBlock<'a> {
    pub content: &'a str,
    pub content_start: usize,
    pub content_end: usize,
}

pub fn frame_block_at(source: &str, offset: usize) -> Option<EmbeddedFrameBlock<'_>> {
    frame_blocks(source)
        .into_iter()
        .find(|block| offset >= block.content_start && offset <= block.content_end)
}

pub fn frame_blocks(source: &str) -> Vec<EmbeddedFrameBlock<'_>> {
    let mut blocks = Vec::new();
    let mut search_start = 0usize;

    while let Some(style_start_relative) = source[search_start..].find("<style") {
        let style_start = search_start + style_start_relative;
        let Some(tag_end_relative) = source[style_start..].find('>') else {
            break;
        };
        let tag_end = style_start + tag_end_relative;
        let tag = &source[style_start..=tag_end];
        search_start = tag_end + 1;

        if !is_frame_style_tag(tag) {
            continue;
        }

        let Some(close_relative) = source[search_start..].find("</style>") else {
            break;
        };
        let content_start = search_start;
        let content_end = search_start + close_relative;
        blocks.push(EmbeddedFrameBlock {
            content: &source[content_start..content_end],
            content_start,
            content_end,
        });
        search_start = content_end + "</style>".len();
    }

    blocks
}

pub fn map_diagnostic_from_block(
    mut diagnostic: Diagnostic,
    block: &EmbeddedFrameBlock<'_>,
) -> Diagnostic {
    diagnostic.span = Span {
        start: diagnostic.span.start + block.content_start,
        end: diagnostic.span.end + block.content_start,
    };
    diagnostic
}

fn is_frame_style_tag(tag: &str) -> bool {
    tag.contains("lang=\"frame\"")
        || tag.contains("lang='frame'")
        || tag.contains("lang=frame")
        || tag.contains("type=\"text/frame\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_frame_style_blocks() {
        let source = "<script></script>\n<style lang=\"frame\">\ncard Demo {\n}\n</style>";
        let blocks = frame_blocks(source);

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].content.contains("card Demo"));
        assert!(frame_block_at(source, source.find("Demo").unwrap()).is_some());
    }

    #[test]
    fn ignores_css_style_blocks() {
        let source = "<style>\n.card { color: red; }\n</style>";

        assert!(frame_blocks(source).is_empty());
    }
}
