use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::sync::Arc;
use rust_tokenizers::{Vocab, BertTokenizer, TruncationStrategy, Tokenizer, TokenizedInput};
use std::fs;
use rust_tokenizers::preprocessing::tokenizer::base_tokenizer::Offset;


fn download_file_to_cache(src: &str, target: &str) -> Result<PathBuf, reqwest::Error> {
    let mut home = dirs::home_dir().unwrap();
    home.push(".cache");
    home.push(".rust_tokenizers");
    home.push(target);
    if !home.exists() {
        let mut response = reqwest::blocking::get(src)?;
        fs::create_dir_all(home.parent().unwrap()).unwrap();
        let mut dest = File::create(&home).unwrap();
        copy(&mut response, &mut dest).unwrap();
    }
    Ok(home)
}


#[test]
fn test_bert_tokenization() {
    let vocab_path = download_file_to_cache("https://s3.amazonaws.com/models.huggingface.co/bert/bert-base-uncased-vocab.txt",
                                            "bert-base-uncased_vocab.txt").unwrap();

    let vocab = Arc::new(rust_tokenizers::BertVocab::from_file(vocab_path.to_str().unwrap()));
    let bert_tokenizer: BertTokenizer = BertTokenizer::from_existing_vocab(vocab.clone(), true);


    let original_strings = [
        "This is a sample sentence to be tokénized",
        "Hello, y'all! How are you 😁 ?",
        "İs th!s 𩸽 Ϻ Šœ Ugljšić dấu nặng"
    ];

    let expected_results = [
        TokenizedInput {
            token_ids: vec!(101, 2023, 2003, 1037, 7099, 6251, 2000, 2022, 19204, 3550, 102),
            segment_ids: vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            special_tokens_mask: vec!(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1),
            overflowing_tokens: vec!(),
            num_truncated_tokens: 0,
            token_offsets: vec!(None, Some(Offset { begin: 0, end: 4 }), Some(Offset { begin: 5, end: 7 }), Some(Offset { begin: 8, end: 9 }), Some(Offset { begin: 10, end: 16 }),
                                Some(Offset { begin: 17, end: 25 }), Some(Offset { begin: 26, end: 28 }), Some(Offset { begin: 29, end: 31 }), Some(Offset { begin: 32, end: 38 }),
                                Some(Offset { begin: 38, end: 42 }), None),
            reference_offsets: vec!(),
            mask: vec!(),
        },
        TokenizedInput {
            token_ids: vec!(101, 7592, 1010, 1061, 1005, 2035, 999, 2129, 2024, 2017, 100, 1029, 102),
            segment_ids: vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            special_tokens_mask: vec!(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1),
            overflowing_tokens: vec!(),
            num_truncated_tokens: 0,
            token_offsets: vec!(None, Some(Offset { begin: 0, end: 5 }), Some(Offset { begin: 5, end: 6 }), Some(Offset { begin: 7, end: 8 }), Some(Offset { begin: 8, end: 9 }),
                                Some(Offset { begin: 9, end: 12 }), Some(Offset { begin: 12, end: 13 }), Some(Offset { begin: 14, end: 17 }), Some(Offset { begin: 18, end: 21 }),
                                Some(Offset { begin: 22, end: 25 }), Some(Offset { begin: 26, end: 27 }), Some(Offset { begin: 28, end: 29 }), None),
            reference_offsets: vec!(),
            mask: vec!(),
        },
        TokenizedInput {
            token_ids: vec!(101, 2003, 16215, 999, 1055, 100, 100, 1055, 29674, 1057, 23296, 22578, 2594, 4830, 2226, 16660, 2290, 102),
            segment_ids: vec!(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            special_tokens_mask: vec!(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1),
            overflowing_tokens: vec!(),
            num_truncated_tokens: 0,
            token_offsets: vec!(None, Some(Offset { begin: 0, end: 2 }), Some(Offset { begin: 3, end: 5 }), Some(Offset { begin: 5, end: 6 }), Some(Offset { begin: 6, end: 7 }),
                                Some(Offset { begin: 8, end: 9 }), Some(Offset { begin: 10, end: 11 }), Some(Offset { begin: 12, end: 13 }), Some(Offset { begin: 13, end: 14 }),
                                Some(Offset { begin: 15, end: 16 }), Some(Offset { begin: 16, end: 18 }), Some(Offset { begin: 18, end: 20 }), Some(Offset { begin: 20, end: 22 }),
                                Some(Offset { begin: 23, end: 25 }), Some(Offset { begin: 25, end: 26 }), Some(Offset { begin: 27, end: 30 }), Some(Offset { begin: 30, end: 31 }), None),
            reference_offsets: vec!(),
            mask: vec!(),
        },
    ].to_vec();

    let output = bert_tokenizer.encode_list(original_strings.to_vec(),
                                            128,
                                            &TruncationStrategy::LongestFirst,
                                            0);

//    println!("{:?}", output);
    for (idx, (predicted, expected)) in output.iter().zip(expected_results.iter()).enumerate() {
        let original_sentence_chars: Vec<char> = original_strings[idx].chars().collect();
        for offset in &predicted.token_offsets {
            match offset {
                Some(offset) => {
                    let (start_char, end_char) = (offset.begin as usize, offset.end as usize);
                    let text: String = original_sentence_chars[start_char..end_char].iter().collect();
                    println!("{:?} -  {}", offset, text)
                }
                None => continue
            }
        };

        assert_eq!(predicted.token_ids, expected.token_ids);
        assert_eq!(predicted.special_tokens_mask, expected.special_tokens_mask);
        assert_eq!(predicted.token_offsets, expected.token_offsets);
    }
    
}