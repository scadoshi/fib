pub fn perf<I>(functions: I, reps: usize)
where
    I: IntoIterator<Item = (&'static str, Box<dyn Fn(usize) -> usize>)>,
{
    let functions: Vec<_> = functions.into_iter().collect();
    let longest_name_length = functions
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or_default();
    println!("# Perf'ing {} times on {} functions", reps, functions.len());
    functions.iter().for_each(|(name, fun)| {
        let name = format!("{}{}", name, " ".repeat(longest_name_length - name.len()));
        let start = std::time::Instant::now();
        let result = (0..=reps).fold(0, |_, n| fun(n));
        std::hint::black_box(result);
        println!("- {} => {} in {:.2?}", name, result, start.elapsed());
    });
}

pub fn funbox<I, R, F>(name: &'static str, fun: F) -> (&'static str, Box<dyn Fn(I) -> R>)
where
    F: Fn(I) -> R + 'static,
{
    (name, Box::new(fun))
}
