use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // 配列でレーティングを表す
    // 答えを出力するときに最大/最小の計算は以下のように行う
    // 最大
    //   3200以上の人数 + 1以上のランクの数
    // 最小
    //   3200以上の人数 - 1以上のランクの数
    //     1以上の場合
    //       3200以上の人数 - 1以上のランクの数 + 1以上のランクの数
    //     0以下の場合
    //       1以上のランクの数
    // 最小
    // 3200が3人、茶色1人、赤色2人のばあい
    // 茶色 + 赤色 + 他1色 = 3
    // 3200が2人、茶色1人、赤色2人のばあい
    // 茶色 + 赤色 = 2
    // 3200が1人、茶色1人、赤色2人のばあい
    // 茶色 + 赤色 = 2

    let mut ranking = vec![0; 9];

    for aa in a {
        if 1 <= aa && aa <= 399 {
            ranking[0] += 1;
        } else if 400 <= aa && aa <= 799 {
            ranking[1] += 1;
        } else if 800 <= aa && aa <= 1199 {
            ranking[2] += 1;
        } else if 1200 <= aa && aa <= 1599 {
            ranking[3] += 1;
        } else if 1600 <= aa && aa <= 1999 {
            ranking[4] += 1;
        } else if 2000 <= aa && aa <= 2399 {
            ranking[5] += 1;
        } else if 2400 <= aa && aa <= 2799 {
            ranking[6] += 1;
        } else if 2800 <= aa && aa <= 3199 {
            ranking[7] += 1;
        } else if 3200 <= aa {
            ranking[8] += 1;
        }
    }

    let mut max = 0;
    let mut min = 0;

    let mut exist_ranking_count = 0;
    for r in ranking[0..=7].iter() {
        if *r >= 1 {
            exist_ranking_count += 1;
        }
    }

    let bakemon_count = ranking[8];

    // 最大
    max = exist_ranking_count + bakemon_count;

    // 最小
    if exist_ranking_count >= 1 {
        min = exist_ranking_count;
    } else {
        min = 1;
    }
    println!("{} {}", min, max)
}
