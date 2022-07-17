fn merge_sort(a: &mut Vec<usize>, start: usize, end: usize) {
    if end - start == 1{
        // すでにソート済み（要素数1）なので何もしない
        return;
    }
    // 2 つに分割した後、小さい配列をソート
    let middle = (start + end) / 2;
    merge_sort(a, start, middle);
    merge_sort(a, middle, end);

	// この時点で以下の 2 つの配列がソートされている：
	// 列 A' に相当するもの [A[start], A[start+1], ..., A[middle−1]]
	// 列 B' に相当するもの [A[middle], A[middle+1], ..., A[end−1]]
	// 以下が Merge 操作となります。
	let mut c1 = start; 
    let mut c2 = middle;
	let mut c_vec: Vec<usize> = Vec::new();
    // A'またはB'のどちらかが空になっている場合
	while c1 != middle || c2 != end {
		if c1 == middle {
			// 列 A' が空の場合
			c_vec.push(a[c2]);
			c2 += 1;
		}
		else if c2 == end {
			// 列 B' が空の場合
			c_vec.push(a[c1]);
			c1 += 1;
		}
		else {
			// A'とB'ともに要素がある場合
			if a[c1] <= a[c2] {
				c_vec.push(a[c1]);
				c1 +=1;	
			}
			else {
				c_vec.push(a[c2]);
				c2 += 1;	
			}
		}
	}
	
	// 列 A', 列 B' を合併した配列CをAに移す
	// [C[0], ..., C[cnt−1]] −> [A[l], ..., A[r−1]]
	let mut cnt = 0;
	for i in c_vec {
		a[start + cnt] = i;
		cnt += 1;
	}
}

fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }
    merge_sort(&mut a, 0, n);
	for i in a {
		print!("{} ", i);
	}
}
