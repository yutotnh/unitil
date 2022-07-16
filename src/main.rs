use clap::Parser;
use std::fs;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let contents = fs::read(&args.file).expect("Something went wrong reading the file");

    if args.count {
        let count = count_tilde(contents);
        println!("Wave dash       (0xA1C1)   : {}", count.0);
        println!("Fullwidth Tilde (0x8FA2B7) : {}", count.1);
    } else {
        let new_contents = unify_tilde(contents);
        fs::write(&args.file, new_contents)?;
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(required = true)]
    file: String,

    /// 全角チルダ(8F A2 B7)の登場回数を表示する
    #[clap(short, long)]
    count: bool,
}

/// 渡されたバイナリから全角チルダを検出して、
/// 全角チルダを波ダッシュに置き換えたバイナリを返す
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

/// 渡されたバイナリから波ダッシュと全角チルダの個数をそれぞれ数えて返す
fn count_tilde(contents: Vec<u8>) -> (u64, u64) {
    let mut wave_dash_count: u64 = 0;
    let mut full_width_tilde_count: u64 = 0;

    let size = contents.len();
    let mut i = 0;
    while i < size {
        if contents[i] & 0x80 == 0x00 {
            i += 1;
            continue;
        } else if contents[i] == 0x8F {
            if contents[i + 1] == 0xA2 && contents[i + 2] == 0xB7 {
                full_width_tilde_count += 1;
            }
            i += 3;
            continue;
        } else if contents[i] & 0x80 == 0x80 {
            if contents[i] == 0xA1 && contents[i + 1] == 0xC1 {
                wave_dash_count += 1;
            }
            i += 2;
            continue;
        }
    }
    (wave_dash_count, full_width_tilde_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_tilde() {
        // '～'(Fullwidth tilde) -> 0x8F A2 B7
        // '～'(Wave dash) -> 0xA1 C1
        // 全角チルダを波ダッシュに変更
        // 渡されたバイナリは全角チルダ1つのみの場合
        assert_eq!(unify_tilde(vec![0x8F, 0xA2, 0xB7]), [0xA1, 0xC1]);
        // 渡されたバイナリが全角チルダ2つのみの場合
        assert_eq!(
            unify_tilde(vec![0x8F, 0xA2, 0xB7, 0x8F, 0xA2, 0xB7]),
            [0xA1, 0xC1, 0xA1, 0xC1]
        );

        // 'a'  -> 0x61
        // 'あ' -> 0xA4 A2
        // '苤' -> 0x8F D7 D4
        // 渡されたバイナリ中に全角チルダ1つと全角チルダ以外が含まれている場合
        assert_eq!(
            unify_tilde(vec![0x61, 0xA4, 0xA2, 0x8F, 0xA2, 0xB7, 0x8F, 0xD7, 0xD4]),
            [0x61, 0xA4, 0xA2, 0xA1, 0xC1, 0x8F, 0xD7, 0xD4]
        );

        // '\n' -> 0x0A
        // 渡されたバイナリ中に全角チルダが1つもない場合
        assert_eq!(
            unify_tilde(vec![0x61, 0xA4, 0xA2, 0x8F, 0xD7, 0xD4, 0x0A]),
            [0x61, 0xA4, 0xA2, 0x8F, 0xD7, 0xD4, 0x0A]
        );
    }

    #[test]
    fn test_count_tilde() {
        // '～'(Wave dash) -> 0xA1 C1
        // 波ダッシュを数える
        // 1つの場合
        assert_eq!(count_tilde(vec![0xA1, 0xC1]), (1, 0));
        // 2つの場合
        assert_eq!(count_tilde(vec![0xA1, 0xC1, 0xA1, 0xC1]), (2, 0));

        // '～'(Fullwidth tilde) -> 0x8F A2 B7
        // 全角チルダを数える
        // 1つの場合
        assert_eq!(count_tilde(vec![0x8F, 0xA2, 0xB7]), (0, 1));
        // 2つの場合
        assert_eq!(
            count_tilde(vec![0x8F, 0xA2, 0xB7, 0x8F, 0xA2, 0xB7]),
            (0, 2)
        );

        // 'a'  -> 0x61
        // 'あ' -> 0xA4 A2
        // '苤' -> 0x8F D7 D4
        // 波ダッシュと全角チルダ以外が存在するの文字列中に
        // 波ダッシュと全角チルダが1つずつある場合
        assert_eq!(
            count_tilde(vec![
                0x61, 0xA4, 0xA2, 0x8F, 0xA2, 0xB7, 0xA1, 0xC1, 0x8F, 0xD7, 0xD4
            ]),
            (1, 1)
        );
        // 波ダッシュと全角チルダ以外が存在するの文字列中に
        // 波ダッシュと全角チルダが1つもない場合
        assert_eq!(
            count_tilde(vec![0x61, 0xA4, 0xA2, 0x8F, 0xD7, 0xD4]),
            (0, 0)
        );
    }
}
