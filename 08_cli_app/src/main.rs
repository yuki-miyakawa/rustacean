use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    println!("welcome to the company employee management system!");
    println!("you can use the following commands:");
    println!("  - add [name] to [department]");
    println!("  - list [department]");
    println!("  - list all");
    println!("  - Quit");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        // io::stdout().flush().unwrap(); result型に対してErrを捨てる書き方

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let words: Vec<&str> = input.trim().split_whitespace().collect();
        // trim() は前後の空白を削除
        // split_whitespace() は空白で分割してイテレータを返す
        // collect() はイテレータをコレクションに変換する

        // collect()がめっちゃ便利

        // ex1 hash setに入れてuniqueを作る
        // use std::collections::HashSet;
        // let numbers = vec![1, 2, 2, 3, 4, 4, 5];
        // let unique_numbers: HashSet<i32> = numbers.into_iter().collect();
        // println!("{:?}", unique_numbers); // 出力例: {1, 2, 3, 4, 5} (順序は保証されない)

        // ex2 hash mapに変換する
        // use std::collections::HashMap;
        // let pairs = vec![("apple", 1), ("banana", 2), ("orange", 3)];
        // let map: HashMap<&str, i32> = pairs.into_iter().collect();
        // println!("{:?}", map); // 出力例: {"banana": 2, "apple": 1, "orange": 3} (順序は保証されない)

        // ex3 Stringに結合する
        // let chars = ['h', 'e', 'l', 'l', 'o'];
        // let greeting: String = chars.iter().collect();
        // println!("{}", greeting); // 出力: hello

        match words.as_slice() {
            ["add", name, "to", department] => {
                let name_str = name.to_string();
                let dept_str = department.to_string();

                println!("added {} to {}", name_str, dept_str);

                departments
                    // entry() は HashMap のメソッドで、キーが存在しない場合は新しい値を生成する
                    .entry(dept_str)
                    // or_insert_with() は、キーが存在する場合はなにもしない。キーが存在しない場合に新しい値を生成する、引数に生成するクロージャ(Vec::new)を受取り実行する
                    // クロージャとは関数のように振る舞う匿名のコードブロック
                    // Vec::new はなぜクロージャとして機能するのか？
                    // Vec::new は実際にはVec構造体に関連付けられた関連関数です。しかし、Rustでは引数なしで呼び出せる関数や関連関数は、FnOnce() トレイトを満たすクロージャとして自動的にCoerce（型強制）される特性があります。
                    .or_insert_with(Vec::new)
                    .push(name_str);
            }
            ["list", "all"] => {
                if departments.is_empty() {
                    println!("no employees found");
                    continue;
                }

                println!("list of all employees (by department):");
                let mut sorted_depts: Vec<String> = departments.keys().collect();
                sorted_depts.sort();

                for dept in sorted_depts {
                    println!("\n-- {} --", dept);
                    if let Some(employees) = departments.get(dept) {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        for employee in sorted_employees {
                            println!("- {}", employee);
                        }
                    }
                }
            }

            ["list", department] => match departments.get(*department) {
                Some(employees) => {
                    println!("list of {}:", department);
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    for employee in sorted_employees {
                        println!("- {}", employee);
                    }
                }
                None => println!("{} is not found", department),
            },

            ["quit"] => {
                println!("Quitting the program.");
                break;
            }
            _ => {
                println!("Invalid command. Please use the following format:");
                println!("  - add [name] to [department]");
                println!("  - list [department]");
                println!("  - list all");
                println!("  - quit");
            }
        }
    }
}
