use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const MIN_SIZE: usize = 4;
const PADDING: char = '　';

#[derive(Debug, Clone, Copy, PartialEq)]
enum Start {
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

impl Start {
    fn parse(s: &str) -> Option<Start> {
        match s {
            "rightup" | "ru" => Some(Start::RightUp),
            "rightdown" | "rd" => Some(Start::RightDown),
            "leftup" | "lu" => Some(Start::LeftUp),
            "leftdown" | "ld" => Some(Start::LeftDown),
            _ => None,
        }
    }
}

struct Args {
    input: PathBuf,
    output: PathBuf,
    start: Start,
}

fn usage(program: &str) -> ! {
    eprintln!(
        "使い方: {} <入力ファイル> [-p rightup|ru|rightdown|rd|leftup|lu|leftdown|ld] [-o <出力ファイル>]",
        program
    );
    process::exit(1);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("jp-swl");

    let mut input = None;
    let mut output = None;
    let mut start = Start::RightUp;

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-o" => match iter.next() {
                Some(v) => output = Some(PathBuf::from(v)),
                None => usage(program),
            },
            "-p" => match iter.next().and_then(|v| Start::parse(v)) {
                Some(s) => start = s,
                None => {
                    eprintln!("エラー: 開始位置の指定が不正です。");
                    usage(program);
                }
            },
            _ if input.is_none() => input = Some(PathBuf::from(arg)),
            _ => usage(program),
        }
    }

    let input = input.unwrap_or_else(|| usage(program));
    let output = output.unwrap_or_else(|| default_output(&input));
    Args { input, output, start }
}

// input.txt -> input_result.txt (入力ファイルと同じディレクトリ)
fn default_output(input: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "input".to_string());
    input.with_file_name(format!("{}_result.txt", stem))
}

// 半角カナ(U+FF61..U+FF9F)に対応する全角文字
const HALF_KANA: [char; 63] = [
    '。', '「', '」', '、', '・', 'ヲ', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ッ', 'ー',
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ',
    'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ン', '゛', '゜',
];

// 半角文字を全角に変換する。半角カナの濁点・半濁点は直前の文字と合成する。
fn to_fullwidth(text: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for c in text.chars() {
        let code = c as u32;
        match code {
            0x20 => result.push(PADDING),
            0x21..=0x7E => result.push(char::from_u32(code + 0xFEE0).unwrap()),
            0xFF61..=0xFF9F => {
                let full = HALF_KANA[(code - 0xFF61) as usize];
                let combined = match (full, result.last()) {
                    ('゛', Some(&'ウ')) => Some('ヴ'),
                    ('゛', Some(&prev)) if "カキクケコサシスセソタチツテトハヒフヘホ".contains(prev) => {
                        char::from_u32(prev as u32 + 1)
                    }
                    ('゜', Some(&prev)) if "ハヒフヘホ".contains(prev) => {
                        char::from_u32(prev as u32 + 2)
                    }
                    _ => None,
                };
                match combined {
                    Some(c) => *result.last_mut().unwrap() = c,
                    None => result.push(full),
                }
            }
            _ => result.push(c),
        }
    }
    result
}

// 文字数以上で最小の平方数の一辺 (最小 4)
fn square_size(len: usize) -> usize {
    let mut n = MIN_SIZE;
    while n * n < len {
        n += 1;
    }
    n
}

// 開始位置の角から時計回りに外側から内側へ文字を置く
fn swirl(chars: &[char], start: Start) -> Vec<Vec<char>> {
    let n = square_size(chars.len());
    let mut grid = vec![vec![None; n]; n];

    // 時計回り順: 下, 左, 上, 右
    const DIRS: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let last = n as isize - 1;
    let (mut row, mut col, mut dir) = match start {
        Start::RightUp => (0, last, 0),
        Start::RightDown => (last, last, 1),
        Start::LeftDown => (last, 0, 2),
        Start::LeftUp => (0, 0, 3),
    };

    let padded = chars.iter().copied().chain(std::iter::repeat(PADDING));
    for c in padded.take(n * n) {
        grid[row as usize][col as usize] = Some(c);

        let in_free_cell = |r: isize, c: isize| {
            r >= 0 && r <= last && c >= 0 && c <= last && grid[r as usize][c as usize].is_none()
        };
        let (dr, dc) = DIRS[dir];
        if !in_free_cell(row + dr, col + dc) {
            dir = (dir + 1) % 4;
        }
        let (dr, dc) = DIRS[dir];
        row += dr;
        col += dc;
    }

    grid.into_iter()
        .map(|line| line.into_iter().map(Option::unwrap).collect())
        .collect()
}

fn main() {
    let args = parse_args();

    let text = fs::read_to_string(&args.input).unwrap_or_else(|e| {
        eprintln!("エラー: {} を開けませんでした: {}", args.input.display(), e);
        process::exit(1);
    });

    // BOM と改行は捨てる
    let text: String = text
        .trim_start_matches('\u{FEFF}')
        .chars()
        .filter(|&c| c != '\r' && c != '\n')
        .collect();
    let chars = to_fullwidth(&text);

    let grid = swirl(&chars, args.start);
    let mut result = String::new();
    for line in &grid {
        result.extend(line);
        result.push('\n');
    }

    if let Err(e) = fs::write(&args.output, &result) {
        eprintln!("エラー: {} に書き込めませんでした: {}", args.output.display(), e);
        process::exit(1);
    }
    println!(
        "{} 文字 -> {}x{} ({} 文字埋め) : {}",
        chars.len(),
        grid.len(),
        grid.len(),
        grid.len() * grid.len() - chars.len(),
        args.output.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(grid: &[Vec<char>]) -> Vec<String> {
        grid.iter().map(|line| line.iter().collect()).collect()
    }

    #[test]
    fn readme_sample() {
        let text = "縦読みで文章を書いてもＡＩに見破られてインプレゾンビが湧くなら渦巻き状に並べればさすがに人間しか読めずインプレゾンビ湧かない説。";
        let grid = swirl(&to_fullwidth(text), Start::RightUp);
        assert_eq!(
            render(&grid),
            [
                "プレゾンビが湧縦",
                "ンに人間しかく読",
                "イがビ湧か読なみ",
                "てすン。なめらで",
                "れさゾ説いず渦文",
                "らばレプンイ巻章",
                "破れべ並に状きを",
                "見にＩＡもてい書",
            ]
        );
    }

    #[test]
    fn sizes_and_padding() {
        assert_eq!(square_size(0), 4);
        assert_eq!(square_size(9), 4);
        assert_eq!(square_size(15), 4);
        assert_eq!(square_size(16), 4);
        assert_eq!(square_size(17), 5);
        assert_eq!(square_size(200), 15);

        let chars: Vec<char> = "あいうえおかきくけこさしすせそたち".chars().collect();
        let grid = swirl(&chars, Start::RightUp);
        let pads = grid.iter().flatten().filter(|&&c| c == PADDING).count();
        assert_eq!(pads, 8);
    }

    #[test]
    fn start_positions() {
        let chars: Vec<char> = "あいうえおかきくけこさしすせそた".chars().collect();
        let cases = [
            (Start::RightUp, ["こさしあ", "けたすい", "くそせう", "きかおえ"]),
            (Start::RightDown, ["きくけこ", "かそたさ", "おせすし", "えういあ"]),
            (Start::LeftDown, ["えおかき", "うせそく", "いすたけ", "あしさこ"]),
            (Start::LeftUp, ["あいうえ", "しすせお", "さたそか", "こけくき"]),
        ];
        for (start, expected) in cases {
            assert_eq!(render(&swirl(&chars, start)), expected, "{:?}", start);
        }
    }

    #[test]
    fn fullwidth_conversion() {
        let converted: String = to_fullwidth("AI 1!ｶﾞｷﾞﾊﾟｳﾞｱ").into_iter().collect();
        assert_eq!(converted, "ＡＩ　１！ガギパヴア");
    }
}
