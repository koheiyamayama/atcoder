use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut b2 = 0;
    let mut b4 = 0;
    let mut b1 = 0;

    for i in 0..n {
        if a[i] % 4 == 0 {
            b4 += 1;
        } else if a[i] % 2 == 0 {
            b2 += 1;
        } else {
            b1 += 1;
        }
    }

    if b2 == 0 && b1 <= b4 + 1 {
        println!("Yes")
    } else if b1 <= b4 {
        println!("Yes")
    } else {
        println!("No")
    }
}

// 数列が以下の条件を満たすか調べれば良さそう
// 4の倍数と偶数のペア、2と偶数のペアが奇数とペアにできない2の半分以上ある
// 1 2 3 4
// 4の倍数は1つ、奇数とペアにできない2が1,2,3
// 1 4 1
// 4の倍数1つ、奇数とペアにできない2が1,1
// 2 7 1 8 2 8
// 2 2 8 7 8 1
// 2*2=4
// 8*2=16
// 8*7=56
// 1*8=8
// 2 7 1 8 6 8
// 2 6 8 7 8 1
