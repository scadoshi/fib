use std::collections::{HashMap, VecDeque};

// basic recursion
#[allow(dead_code)]
pub fn r_fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        r_fib(n - 1) + r_fib(n - 2)
    }
}

// basic recursion with memoization - helper
fn r_fib_memo_helper(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    let mut _f = 0;
    if let Some(v) = memo.get(&n) {
        return *v;
    }
    if n < 2 {
        _f = n;
    } else {
        _f = r_fib_memo_helper(n - 1, memo) + r_fib_memo_helper(n - 2, memo);
    }
    memo.insert(n, _f);
    _f
}

// basic recursion with memoization - starter
pub fn r_fib_memo(n: usize) -> usize {
    let mut memo = HashMap::<usize, usize>::new();
    r_fib_memo_helper(n, &mut memo)
}

// bottom up
pub fn bu_fib(n: usize) -> usize {
    let mut fib = Vec::<usize>::from([0, 1]);
    for k in 2..=n {
        fib.push(fib[k - 1] + fib[k - 2]);
    }
    fib[n]
}

// bottom up with optimized storage
pub fn bu_fib_sized(n: usize) -> usize {
    let mut queue = VecDeque::<usize>::from([0, 1]);
    for _ in 2..=n {
        queue.push_back(queue[0] + queue[1]);
        if queue.len() > 2 {
            queue.pop_front();
        }
    }
    queue.pop_back().unwrap()
}

// any k'ibbonacci - bottom with optimized storage
pub fn bu_kib_sized(n: usize, k: usize) -> usize {
    let mut queue = VecDeque::<usize>::new();
    (0..k).for_each(|i| {
        queue.push_back(if i < k - 1 { 0 } else { 1 });
    });
    if n < k {
        return queue[n];
    }
    (k..=n).for_each(|_| {
        let acc = (1..=k).fold(0, |acc, kk| {
            let prev = queue.get(queue.len() - kk).copied().unwrap_or(0);
            acc + prev
        });
        queue.push_back(acc);
        if queue.len() > k {
            queue.pop_front();
        }
    });
    queue.pop_back().unwrap()
}
