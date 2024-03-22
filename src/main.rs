use tokenizers::tokenizer::Tokenizer;

fn print_encoded<'a, T>(input: T, tokenizer: &Tokenizer)
where
    T: std::convert::Into<tokenizers::EncodeInput<'a>> + std::fmt::Debug,
{
    println!("input: {input:?}");
    let output = tokenizer.encode(input, true).unwrap();

    println!("output: {:?}", output.get_ids());
    println!();
}

fn main() {
    let model = String::from("BAAI/bge-large-en-v1.5");
    let tokenizer = Tokenizer::from_pretrained(&model, None).unwrap();

    print_encoded("foo", &tokenizer);
    print_encoded("foo bar", &tokenizer);
    print_encoded(vec!["foo", "bar"], &tokenizer);
}

