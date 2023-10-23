use std::collections::VecDeque;

/// Trait for selecting a problem to run
pub trait ProblemSelector {
    /// Name of the problem that will be displayed as `"Chose {Self::NAME} problem"`
    const NAME: &'static str;

    /// Vector of all available problems to choose
    fn list() -> Vec<&'static str>;

    /// Given a problem number, run the problem using switch statement.
    /// Problem can be the [`select(args)`] function of other [ProblemSelector]
    fn methods(number: &str, args: &mut VecDeque<String>);

    /// Select a problem to run preselected or from user input
    /// The problem can be run with -1 to run all problems.
    fn select(args: &mut VecDeque<String>) {
        println!("Chose {} problem", Self::NAME);
        let arg = args.pop_front();

        match arg {
            Some(arg) => {
                if arg == "-1" {
                    args.push_front("-1".to_string());
                    for (i, _) in Self::list().iter().enumerate() {
                        Self::methods(&i.to_string(), args);
                    }

                    return;
                }

                Self::methods(&arg.to_string(), args)
            }
            None => {
                println!();
                println!("Provide a problem number:");
                println!("-1: run all problems");

                let problems = Self::list();
                for (i, problem) in problems.iter().enumerate() {
                    println!("{}: {}", i, problem);
                }

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input == "-1" {
                    args.push_front("-1".to_string());
                    for (i, _) in problems.iter().enumerate() {
                        Self::methods(&i.to_string(), args);
                    }

                    return;
                }

                Self::methods(&input.trim(), args)
            }
        }
    }
}
