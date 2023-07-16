#![crate_type = "lib"]

use tokenizers::tokenizer::{Result, Tokenizer};

// from https://github.com/aisamanra/rust-haskell-ffi/blob/master/fact.rs

#[no_mangle]
pub extern fn getTokenizer(hfTokName:String) -> Result<Tokenizer> {
    let tok = Tokenizer::from_pretrained(hfTokName, None)?;
    Ok(tok);
}

#[no_mangle]
pub extern fn encode(hfTokName:String, str:String) -> Result<Vec<u32>> {
    let tokenizer = Tokenizer::from_pretrained(hfTokName, None)?;

    let encoding = tokenizer.encode(str, false)?;

    Ok(encoding.get_tokens());
}

#[no_mangle]
pub extern fn decode(hfTokName:String, toks:Vec<u32>) -> Result<String> {
    let tokenizer = Tokenizer::from_pretrained(hfTokName, None)?;
    let decoded = tokenizer.decode(toks)?;

    Ok(decoded);
}
