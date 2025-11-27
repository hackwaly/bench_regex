use regex::Regex;

const TEST_STRING: &str = "Non enim laborum velit commodo deserunt incididunt elit sunt nulla ullamco. Quis ipsum aliqua id mollit minim velit et cupidatat eiusmod nostrud pariatur duis irure ad. Dolore enim et elit fugiat.\nNostrud elit minim aute qui labore id aute aute ea nostrud cupidatat. Aliquip et commodo anim dolor nostrud voluptate proident. Voluptate tempor amet consequat nisi excepteur aute anim aute. Sunt ipsum tempor esse consequat cupidatat.\nIpsum minim cillum adipisicing incididunt incididunt qui non excepteur mollit qui. Non aute sunt dolore eu sunt ea aute nisi dolor eiusmod. Fugiat id culpa exercitation consectetur cupidatat. Sunt sint nostrud dolor aute sit cupidatat voluptate reprehenderit ut cillum nulla. Dolore sunt elit elit et quis qui.\n";

fn wordcount(input: &str, re: &Regex) -> (usize, usize, usize) {
    let mut words = 0;

    for _cap in re.find_iter(input) {
        words += 1;
    }

    (0, words, 0)
}

fn main() {
    let re = Regex::new(r"\w+").unwrap();
    let (lines, words, chars) = wordcount(TEST_STRING, &re);
    println!("{} {} {}", lines, words, chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordcount() {
        let re = Regex::new(r"(\w+)").unwrap();
        let (lines, words, chars) = wordcount(TEST_STRING, &re);
        println!("lines: {}, words: {}, chars: {}", lines, words, chars);
        assert!(lines > 0);
        assert!(words > 0);
        assert!(chars > 0);
    }
}
