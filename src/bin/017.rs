const GAP: &str = " and ";
const TENS_ONES: &str = "-";

fn convert(x: u64) -> String {
    match x {
        0 => return "Zero".to_string(),
        1 => return "One".to_string(),
        2 => return "Two".to_string(),
        3 => return "Three".to_string(),
        4 => return "Four".to_string(),
        5 => return "Five".to_string(),
        6 => return "Six".to_string(),
        7 => return "Seven".to_string(),
        8 => return "Eight".to_string(),
        9 => return "Nine".to_string(),  
        10 => return "Ten".to_string(),
        11 => return "Eleven".to_string(),
        12 => return "Twelve".to_string(),
        13 => return "Thirteen".to_string(),
        14 => return "Fourteen".to_string(),
        15 => return "Fifteen".to_string(),
        16 => return "Sixteen".to_string(),
        17 => return "Seventeen".to_string(),
        18 => return "Eighteen".to_string(),
        19 => return "Nineteen".to_string(),
        _ => {}
    };

    // two-digit composite names
    if x >= 20 && x < 100 {
        let ones = x % 10;
        let tens = x / 10;
        let str_ones;
        if ones != 0 {
            str_ones = format!("{TENS_ONES}{}", convert(ones));
        } else {
            str_ones = "".to_string();
        }
        match tens {
            3 => return format!("Thirty{str_ones}"),
            2 => return format!("Twenty{str_ones}"),
            4 => return format!("Forty{str_ones}"),
            5 => return format!("Fifty{str_ones}"),
            6 => return format!("Sixty{str_ones}"),
            7 => return format!("Seventy{str_ones}"),
            8 => return format!("Eighty{str_ones}"),
            9 => return format!("Ninety{str_ones}"),
            _ => {}
        };
    } else {
    }
    // three-digit composite names
    if x >= 100 && x < 1000 {
        let ones_and_tens = x % 100;
        let hundreds = x / 100;
        let str_ones_and_tens;
        if ones_and_tens != 0 {
            str_ones_and_tens = format!("{GAP}{}", convert(ones_and_tens));
        } else {
            str_ones_and_tens = "".to_string();
        }
        return format!("{}Hundred{str_ones_and_tens}", convert(hundreds));
    } else {
    }

    // four to six digits
    if x >= 1000 && x < 1000000 {
        let low = x % 1000;
        let high = x / 1000;
        let str_low;
        if low != 0 {
            str_low = format!("{GAP}{}", convert(low));
        } else {
            str_low = "".to_string();
        }
        return format!("{}Thousand{str_low}", convert(high));
    } else {
    }

    // seven to nine digits
    if x >= 1000000 && x < 1000000000 {
        let low = x % 1000000;
        let high = x / 1000000;
        let str_low;
        if low != 0 {
            str_low = format!("{GAP}{}", convert(low));
        } else {
            str_low = "".to_string();
        }
        return format!("{}Million{str_low}", convert(high));
    } else {
    }

    // ten to twelve digits
    if x >= 1000000000 && x < 1000000000000 {
        let low = x % 1000000000;
        let high = x / 1000000000;
        let str_low;
        if low != 0 {
            str_low = format!("{GAP}{}", convert(low));
        } else {
            str_low = "".to_string();
        }
        return format!("{}Billion{str_low}", convert(high));
    } else {
    }

    // thirteen to fifteen digits
    if x >= 1000000000000 && x < 1000000000000000 {
        let low = x % 1000000000000;
        let high = x / 1000000000000;
        let str_low;
        if low != 0 {
            str_low = format!("{GAP}{}", convert(low));
        } else {
            str_low = "".to_string();
        }
        return format!("{}Trillion{str_low}", convert(high));
    } else {
    }

    return "...".to_string();
}

fn number_letter_counts(k: u64) -> u32 {
  (1..k + 1)
  .map(convert)
  .map(|w| w.chars().filter(|&c| c != '-' && c != ' ').count() as u32)
  .sum()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    println!("{}", number_letter_counts(1000));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(number_letter_counts(1000), 21124);
    }
}
