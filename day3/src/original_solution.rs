use color_eyre::eyre::Result;

pub fn original_solution(lines: &Vec<String>) {
    use std::time::Instant;
    let now = Instant::now();

    let mut priority_sum: u32 = 0;
    let mut priority_badge: u32 = 0;

    for group in lines.chunks(3) {
        if let Some(badge) = find_badge(group) {
            priority_badge += get_priority(badge).unwrap() as u32;
        }

        for line in group {
            if let Some(found_object) = find_doubled_object(line) {
                priority_sum += get_priority(found_object).unwrap() as u32;
            }
        }

    }

    let elapsed = now.elapsed();

    println!("\nOriginal Solution");
    println!("Priority sum is {}", priority_sum);
    println!("Priority badges is {}", priority_badge);
    println!("Elapsed: {:.2?}", elapsed);
}

fn find_doubled_object(bag: &str) -> Option<char> {
    let bag_compartments = split_string(bag).unwrap();

    for object_first_compartment in bag_compartments.0.chars() {
        for object_second_compartment in bag_compartments.1.chars() {
            if object_first_compartment == object_second_compartment {
                return Some(object_first_compartment);
            }
        }
    }

    None
}

fn find_badge(group: &[String]) -> Option<char> {
    if group.len() == 3 {
        for object_first_bag in group[0].chars() {
            for object_second_bag in group[1].chars() {
                if object_first_bag == object_second_bag {
                    for object_third_bag in group[2].chars() {
                        if object_first_bag == object_third_bag {
                            return Some(object_first_bag);
                        }
                    }
                }
            }
        }
    }

    None
}

fn split_string(slice: &str) -> Result<(&str, &str)> {
    let length = slice.len();

    if length % 2 == 0 {
        Ok(slice.split_at(length/2))
    } else {
        Err(color_eyre::eyre::eyre!("string length is not even: {}", length))
    }
}

fn get_priority(character: char) -> Result<u8> {
    const LOWER_CASE_OFFSET:u8 = 96;
    const UPPER_CASE_OFFSET:u8 = 38;

    if character.is_ascii_lowercase() {
        Ok(character as u8 - LOWER_CASE_OFFSET)
    } else if character.is_ascii_uppercase() {
        Ok(character as u8 - UPPER_CASE_OFFSET)
    } else {
        Err(color_eyre::eyre::eyre!("not a valid character: {character:?}"))
    }
}

#[cfg(test)]
mod tests_part1 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_priority_low_lowercase() {
        assert_eq!(get_priority('a').unwrap(), 1);
    }

    #[test]
    fn test_get_priority_high_lowercase() {
        assert_eq!(get_priority('z').unwrap(), 26);
    }

    #[test]
    fn test_get_priority_lowercase() {
        assert_eq!(get_priority('p').unwrap(), 16);
    }

    #[test]
    fn test_get_priority_low_uppercase() {
        assert_eq!(get_priority('A').unwrap(), 27);
    }

    #[test]
    fn test_get_priority_high_uppercase() {
        assert_eq!(get_priority('Z').unwrap(), 52);
    }

    #[test]
    fn test_get_priority_uppercase() {
        assert_eq!(get_priority('P').unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "not a valid character: '!'")]
    fn test_get_priority_invalid_character() {
        let _ = get_priority('!').unwrap();
    }

    #[test]
    fn test_split_string() {
        assert_eq!(split_string("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
    }

    #[test]
    #[should_panic(expected = "string length is not even: 25")]
    fn test_split_string_odd() {
        let _ = split_string("vJrwpWsdfsdfhscsFMMfFFhFp").unwrap();
    }
}