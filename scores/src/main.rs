use scores::*;

fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
    assert_eq!(score("ThiS is A Test"), 14);
    println!("{}", score("long sentences are working"));

    assert_eq!(score("long sentences are working"), 34);
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}
