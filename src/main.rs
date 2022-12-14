use std::env;
use std::fs;
use std::path::Path;

//The following crates are used for testing
#[cfg(test)]
extern crate tempfile; //Creates temp files and directories
#[cfg(test)]
use assert_cmd::prelude::*; // Add methods on commands
#[cfg(test)]
use predicates::prelude::*;
#[cfg(test)]
use std::process::Command; // Run programs // Used for writing assertions

fn main() {
    // This should be called with two command line arguments,
    // both file names. The first should be the name of a file
    // containing the text to disemvowel, and the second should
    // be the file we want to write the disemvoweled text to.
    let args: Vec<String> = env::args().collect();

    //TODO: Panic if not enough arguments are provided
    //Panic should output the string "Not enough arguments"

    //TODO:
    //  * Pass an argument to read_file to read the original text
    //  * Pass that to disemvowel to remove the vowels
    //  * Write the disemvoweled text using write_file

    // Replace String::from("dummy text") with what you get from read_file
    let from = Path::new(args.get(1).expect("Not enough arguments"));
    let to = Path::new(args.get(2).expect("Not enough arguments"));
    let s = read_file(&from);

    let s_disemvowel = disemvowel(&s);

    // Use command-line arguments for the name of the file,
    // and s_disemvowel for the text to write out.
    write_file(&to, &s_disemvowel);
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}
fn write_file(path: &Path, s: &str) {
    fs::write(path, s).expect("Unable to write file");
}

//TODO: Return the input string without vowels.
fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars().filter(|c| !vowels.contains(c)).collect()
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::*;
    mod disemvowel {
        use super::*;
        #[test]
        fn hello_world() {
            let input = "Hello, world!";
            let expected = "Hll, wrld!";

            assert_eq!(expected, disemvowel(input));
        }

        #[test]
        fn empty() {
            assert_eq!("", disemvowel(""));
        }

        #[test]
        fn no_vowels() {
            assert_eq!("pqrst", disemvowel("pqrst"));
        }

        #[test]
        fn all_vowels() {
            assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
        }

        #[test]
        fn morris_minnesota() {
            assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
        }

        #[test]
        fn handle_punctuation() {
            assert_eq!(
                "n (nxplnd) lphnt!",
                disemvowel("An (Unexplained) Elephant!")
            );
        }

        #[test]
        fn handle_unicode() {
            assert_eq!("Sm hrglyphs: ????	????	????	????", disemvowel("Some hieroglyphs: ????	????	????	????"));
            assert_eq!("Sm Lnr B: 	????	????	????	????", disemvowel("Some Linear B: 	????	????	????	????"));
            assert_eq!(" lttl Phncn: ????	????	????	????", disemvowel("A little Phoenician: ????	????	????	????"));
            assert_eq!(
                "W cn hndl mj s wll! ????????????",
                disemvowel("We can handle emoji as well! ????????????")
            )
        }
    }

    // Tests that check that the correct panics are generated when
    // there aren't the correct number of command line arguments
    // or the input file isn't readable.
    mod panic_tests {
        use super::*;
        #[test]
        fn requires_two_arguments() {
            let mut cmd = Command::new("target/debug/disemvowel-in-rust");
            cmd.arg("1");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Not enough arguments"));
        }
        #[test]
        fn requires_read_file() {
            let mut cmd = Command::new("target/debug/disemvowel-in-rust");
            cmd.arg("/this/path/does/not/exist")
                .arg("output/path/doesnt/matter");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Could not read the file"));
        }
    }
}
