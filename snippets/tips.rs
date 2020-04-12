#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

fn main() {
    let a = 0;
    let n = 0;

    // HashSet
    let mut st = HashSet::new();
    st.insert(1);
    st.contains(&a);

    let n = format!("{:b}", n);

    // Vec
    let mut vec = Vec::new();
    vec = Vec::with_capacity(100);

    // String
    let s: String;
    s.chars().collect::<Vec<char>>();
    s.chars().filter(|c| *c == '1').count();
    s.chars().filter(|&c| c == 'o').count();
    s.replace("2017", "2018");
    v.iter().collect::<String>()

    // 入力
    input! {s:String}
    let s = s.chars().collect::<Vec<char>>();

    let mut ans = None;
    for c in b'a'..b'z' + 1 {
        let c = char::from(c);
        ans = Some(c.to_string());
    }
    println!("{}", ans.unwrap_or("None".to_string()));

    let ans: String = t.into_iter().collect();
    println!(
        "{}",
        if a == b.chars().rev().collect::<String>() {
            "YES"
        } else {
            "NO"
        }
    );

    input! {
        n: usize,
        v: [(usize, usize); n],
    }

    for (a, b) in v {
        
    }

    // 出力 定数倍高速化
    let mut ans = String::new();
    for i in 0..n {
        ans.push_str(s);
        ans.push('\n');
    }
    print!("{}", ans);

    // max min
    v.into_iter().max().unwrap();
    v.into_iter().min().unwrap();
}
