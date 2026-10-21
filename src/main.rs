use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct UserInput{
    #[arg( num_args =1.., value_name= "TEXT",required=true)]
    text: Vec<String>,
    
    #[arg(short='n')]
    omit_newlines: bool
}

fn main() {
    // Get user input with keyword word and flag --n
    let user_input = UserInput::parse();
    let (text, omit_newlines) = (user_input.text, user_input.omit_newlines);
    print!("{}{}", text.join(" "), if omit_newlines {""} else {"\n"});
    
    

}


#[cfg(test)]
mod test{
    use std::{fs};
    use predicates::prelude::*;
    use assert_cmd::Command;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn runs_with_arg() -> TestResult {
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.arg("arg").assert().success();
        Ok(())
        
    }

    #[test]
    fn fails_when_empty() -> TestResult{
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.assert().failure();
        Ok(())
    }

    #[test]
    fn dialogue_when_fail() -> TestResult{
        let mut cmd = Command::cargo_bin("echor")?;
        cmd.assert().failure().stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn hello1_no_newline() -> TestResult {
        let hello1n_pth = "tests/expected/hello1.n.txt";
        let hello1n_args = ["Hello there", "-n"];
        run("echor", &hello1n_args, &hello1n_pth)
    }

    #[test]
    fn hello1() -> TestResult{
        let hello1_pth = "tests/expected/hello1.txt";
        let hello1_args = ["Hello there"];
        run("echor", &hello1_args, hello1_pth)
    }

    #[test]
    fn hello2() -> TestResult {
        let hello2_pth = "tests/expected/hello2.txt";
        let hello2_args = ["Hello", "there"];
        run("echor", &hello2_args, hello2_pth)
    }

    #[test]
    fn hello2_no_newline() -> TestResult {
        let hello2n_pth = "tests/expected/hello2.n.txt";
        let hello2n_args = ["Hello", "there", "-n"];
        run("echor", &hello2n_args, hello2n_pth)
    }
    
    fn run(program_name: &str, args: &[&str], file_pth: &str) -> TestResult{
        let file_txt = fs::read_to_string(file_pth)?;
        let mut cmd = Command::cargo_bin(program_name)?;
        cmd.args(args).assert().stdout(file_txt);        
        Ok(())
    }





}
