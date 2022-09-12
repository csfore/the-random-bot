#[cfg(test)]
mod command_tests {
    use crate::{generators, helpers};
    use rand::Rng;

    #[test]
    fn capitalize_test() {
        let control = helpers::capitalize_first_letter("hello");
        assert_eq!(control, "Hello");
    }

    #[test]
    fn fibo_test() {
        let control = generators::nth_fibo(5);
        assert_eq!(control, 5)
    }

    #[test]
    fn eight_ball_test() {
        let responses = ["It is certain",
            "Without a doubt",
            "You may rely on it",
            "Yes, definitely",
            "It is decidedly so",
            "As I see it, yes",
            "Most likely",
            "Yes",
            "Outlook good",
            "Signs point to yes",
            "Reply hazy try again",
            "Better not tell you now",
            "Ask again later",
            "Cannot predict now",
            "Concentrate and ask again",
            "Don't count on it",
            "Outlook not so good",
            "My sources say no",
            "Very doubtful",
            "My reply is no"
        ];
        let index = rand::thread_rng().gen_range(0..=responses.len() - 1);
        assert!(responses.contains(&responses[index]))
    }
}