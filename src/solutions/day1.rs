use super::Solution;

pub struct Day1A;

impl Solution for Day1A {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut sum: u32 = 0;
        for l in input.lines() {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            for c in l.chars() {
                if c.is_numeric() {
                    if first.is_none() {
                        first = Some(c);
                    }
                    last = Some(c);
                }
            }
            if let Some(first) = first {
                if let Some(last) = last {
                    let n = format!("{}{}", first, last);
                    if let Ok(n) = n.parse::<u32>() {
                        sum += n;
                    } else {
                        return Err(anyhow::anyhow!("Could not parse number {}", n))
                    }
                } else {
                    return Err(anyhow::anyhow!("Could not find 2 numbers in line {}", l))
                }
            } else {
                return Err(anyhow::anyhow!("Could not find any numbers in line {}", l))
            }
        }

        Ok(sum.to_string())
    }
}
