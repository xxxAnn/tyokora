use colored::{ColoredString, Colorize};

pub fn welcome_message() -> Vec<ColoredString> {
    // 【我が主】をUSERの選んだ呼び方にしましょう。
    vec!("こんにちは。".truecolor(255, 0, 150), "私はチョコラと申します。".truecolor(255, 0, 150), "今からよろしくお願いしますね。".truecolor(255, 0, 150), "ではご命令を、我が主。".truecolor(255, 0, 150))
}