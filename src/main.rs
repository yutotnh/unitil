use clap::Parser;
use std::fs;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let contents = fs::read(&args.file).expect("Something went wrong reading the file");

    let new_contents = unify_tilde(contents);

    fs::write(&args.file, new_contents)?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(required = true)]
    file: String,
}

fn unify_tilde(contents: Vec<u8>) -> Vec<u8> {
    let size = contents.len();
    let mut i = 0;
    let mut new_contents: Vec<u8> = Vec::new();
    while i < size {
        if contents[i] & 0x80 == 0x00 {
            new_contents.push(contents[i]);

            i += 1;
            continue;
        } else if contents[i] == 0x8F {
            if contents[i + 1] == 0xA2 && contents[i + 2] == 0xB7 {
                new_contents.push(0xA1);
                new_contents.push(0xC1);
            } else {
                new_contents.push(contents[i]);
                new_contents.push(contents[i + 1]);
                new_contents.push(contents[i + 2]);
            }

            i += 3;
            continue;
        } else if contents[i] & 0x80 == 0x80 {
            new_contents.push(contents[i]);
            new_contents.push(contents[i + 1]);

            i += 2;
            continue;
        }
    }
    new_contents
}

#[test]
fn test_unify_tilde() {
    // '～' -> 0x8F A2 B7
    assert_eq!(unify_tilde(vec![0x8F, 0xA2, 0xB7]), [0xA1, 0xC1]);

    // 'a'  -> 0x61
    // 'あ' -> 0xA4 A2
    // '苤' -> 0x8F D7 D4
    assert_eq!(
        unify_tilde(vec![0x61, 0xA4, 0xA2, 0x8F, 0xA2, 0xB7, 0x8F, 0xD7, 0xD4]),
        [0x61, 0xA4, 0xA2, 0xA1, 0xC1, 0x8F, 0xD7, 0xD4]
    );
}
