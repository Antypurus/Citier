use std::str::FromStr;

pub fn event_tokenizer(event: &str) -> Vec<&str> {
    let mut fields: Vec<&str> = Vec::new();

    let mut slice_start: usize = 0;
    let mut scope_stack: Vec<u8> = Vec::new();

    for (i, character) in event.bytes().enumerate() {
        match (scope_stack.last(), character) {
            (Some(b'"'), b'"') => {
                scope_stack.pop();
            }
            (Some(b'['), b']') => {
                scope_stack.pop();
            }
            (Some(b'('), b')') => {
                scope_stack.pop();
            }
            (_, b'[') => {
                scope_stack.push(b'[');
            }
            (_, b'"') => {
                scope_stack.push(b'"');
            }
            (_, b'(') => {
                scope_stack.push(b'(');
            }
            (None, b',') => {
                fields.push(&event[slice_start..i]);
                slice_start = i + 1;
            }
            (None, b'\n') => {
                fields.push(&event[slice_start..(i - 1)]);
                slice_start = i + 1;
            }
            (_, _) => {}
        }
    }

    return fields;
}

pub fn owning_event_tokenizer(event: &str) -> Vec<String> {
    let fields = event_tokenizer(event);

    let mut result: Vec<String> = Vec::new();
    result.reserve(fields.len());

    for field in fields {
        result.push(String::from_str(field).unwrap_or_default());
    }

    return result;
}
