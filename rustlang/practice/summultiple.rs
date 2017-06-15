pub fn sum_of_multiples(limit :i64, multiples :&Vec<i64>) -> i64 {
  (1..limit)
    .filter(|n| {
        multiples
            .into_iter()
            .any(|m| n % m == 0)
    })
    .fold(0, |sum, i| sum + i)
}
