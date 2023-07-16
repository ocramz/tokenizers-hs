use tokenizers::tokenizer::{Result, Tokenizer};

fn encode(hfTokName, str) {
    let tokenizer = Tokenizer::from_pretrained(hfTokName, None)?;

    let encoding = tokenizer.encode(str, false)?;

    Ok(encoding.get_tokens());
}
