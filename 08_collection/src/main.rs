// 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。
// 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。 UTF-8エンコードに関する詳細を心に留めておいてください！
// ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。 それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

use std::collections::HashMap;
// 無理矢理ベクタを使用
fn question1(numbers: &Vec<i32>) {
    let length: f32 = numbers.len() as f32;
    if length == 0.0 {
        println!("args is empty");
        return;
    }
    // average
    let mut sum: i32 = 0;
    for number in numbers {
        // ここで`*numbers`でないのね
        sum += number;
    }
    let average: f32 = sum as f32 / length;

    // median
    let mut numbers_sorted: Vec<i32> = numbers.clone();
    numbers_sorted.sort();

    let mid: usize = numbers_sorted.len() / 2; // 切り捨て
    let mut median: f32 = 0.0;
    if numbers_sorted.len() % 2 == 0 {
        median = (numbers_sorted[mid - 1] + numbers_sorted[mid]) as f32 / 2.0;
    } else {
        median = numbers_sorted[mid] as f32;
    }

    // mode
    let mut counts = HashMap::new();
    for number in numbers {
        *counts.entry(number).or_insert(0) += 1; // これ以外にも書き方ある？
    }

    let mut modes: Vec<i32> = Vec::new();
    let mut max_counts: i32 = 0;
    for (&num, &count) in &counts {
        if count > max_counts {
            max_counts = count;
            modes.clear();
            modes.push(*num);
        } else if count == max_counts {
            modes.push(*num)
        }
    }

    println!("{:?}", mid);
    println!("0: {} 1: {}", numbers_sorted[0], numbers_sorted[1]);
    println!(
        "average: {}, median: {}, modes: {:?}, times: {}",
        average, median, modes, max_counts
    )
}

fn question1_other1(numbers: &[i32]) {
    // average
    let mut sum: i32 = 0;
    for number in numbers {
        sum += number;
    }
    let length: f32 = numbers.len() as f32;
    let average: f32 = sum as f32 / length;

    // median

    println!("average: {}", average)
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'e' | 'o')
}
fn question2(word: &str) -> String {
    if word == "" {
        return String::new();
    }
    let lower_word = word.to_lowercase();

    let first_char = match lower_word.chars().next() {
        Some(c) => c,
        None => return String::new(),
    };

    if is_vowel(first_char) {
        format!("{}-hay", lower_word)
    } else {
        let first_char_len = first_char.len_utf8();
        let rest_word = &lower_word[first_char_len..];
        format!("{}-{}ay", rest_word, first_char)
    }
}

fn main() {
    let numbers_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 7, 7, 5, 5, 5, 5, 5, 5, 1];
    let numbers_slice: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // スライス参照として定義しないとunsizeとなりコンパイルエラー
    question1(&numbers_vec);
    question1_other1(numbers_slice);

    let word1 = "Hello";
    let pig_word1 = question2(word1);
    println!("{}", pig_word1);
    let word2 = "butter";
    let pig_word2 = question2(word2);
    println!("{}", pig_word2)
}
