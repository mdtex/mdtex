use std::ops::Deref;

use crate::ir::RawBlock;

enum ParseStateMachine {
    Nop,
    Paragraph,
    Header,
    MathBlock,
    CodeBlock,
    List,
}

/// This function
pub fn parse_to_raw_blocks(source: impl Deref<Target = str>) -> Vec<RawBlock> {
    let mut state = ParseStateMachine::Nop;

    for line in source.lines() {
        // match state {}
    }

    vec![]
}
