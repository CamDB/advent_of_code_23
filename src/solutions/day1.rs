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
                        return Err(anyhow::anyhow!("Could not parse number {}", n));
                    }
                } else {
                    return Err(anyhow::anyhow!("Could not find 2 numbers in line {}", l));
                }
            } else {
                return Err(anyhow::anyhow!("Could not find any numbers in line {}", l));
            }
        }

        Ok(sum.to_string())
    }
}

pub struct Day1B;

impl Solution for Day1B {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut sum: u32 = 0;
        for l in input.lines() {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            for i in 0..l.len() {
                let current = l.get(i..i+1).and_then(|c| c.chars().next());
                let all_remaining = l.get(i..l.len());

                // Handle normal numerals first.
                if let Some(current) = current {
                    if current.is_numeric() {
                        if first.is_none() {
                            first = Some(current);
                        }
                        last = Some(current);
                        continue;
                    }
                }

                if let Some(all_remaining) = all_remaining {
                    if let Some(n) = get_numeric_word(all_remaining) {
                        if first.is_none() {
                            first = Some(n);
                        }
                        last = Some(n);
                    }
                }
            }

            if let Some(first) = first {
                if let Some(last) = last {
                    let n = format!("{}{}", first, last);
                    if let Ok(n) = n.parse::<u32>() {
                        sum += n;
                    } else {
                        return Err(anyhow::anyhow!("Could not parse number {}", n));
                    }
                } else {
                    return Err(anyhow::anyhow!("Could not find 2 numbers in line {}", l));
                }
            } else {
                return Err(anyhow::anyhow!("Could not find any numbers in line {}", l));
            }
        }

        Ok(sum.to_string())
    }
}

fn get_numeric_word(input: &str) -> Option<char> {
    if input.starts_with("one") {
        return Some('1');
    }
    if input.starts_with("two") {
        return Some('2');
    }
    if input.starts_with("three") {
        return Some('3');
    }
    if input.starts_with("four") {
        return Some('4');
    }
    if input.starts_with("five") {
        return Some('5');
    }
    if input.starts_with("six") {
        return Some('6');
    }
    if input.starts_with("seven") {
        return Some('7');
    }
    if input.starts_with("eight") {
        return Some('8');
    }
    if input.starts_with("nine") {
        return Some('9');
    }

    None
}
