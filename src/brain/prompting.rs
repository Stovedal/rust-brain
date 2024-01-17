const MAX_PROMPT_LENGTH: usize = 3000;

pub fn build_setup_prompt() -> String {
    return "You are a documentation brain. You're here to help a human navigate a large library of documentation."
        .to_string();
}

pub fn build_answer_question_prompt(question: &str, information: Vec<String>) -> String {
    let mut prompt = format!("Write a short answer to this question: '{}', based on this knowledge: '{}'. If you can't find the answer in the provided knowledge, just answer 'I don't know.'.", question, information.join(" "));

    prompt.truncate(MAX_PROMPT_LENGTH);

    return prompt;
}

#[cfg(test)]
mod building_setup_prompts {
    use super::*;

    #[test]
    fn generates_setup_prompt() {
        assert_eq!(
            build_setup_prompt(),
            "You are a documentation brain. You're here to help a human navigate a large library of documentation."
        );
    }
}
#[cfg(test)]
mod building_question_prompts {
    use super::*;

    #[test]
    fn generates_prompt_including_asking_prompt() {
        let question = "What is the meaning of life?";
        let information = vec!["The meaning of life is 42.".to_string()];
        let prompt = build_answer_question_prompt(question, information);

        assert!(prompt.contains("Write a short answer to this question:"));
        assert!(prompt.contains("based on this knowledge: "));
        assert!(prompt.contains(
            "If you can't find the answer in the provided knowledge, just answer 'I don't know.'."
        ));
    }

    #[test]
    fn generates_prompt_including_provided_question() {
        let question = "What is the meaning of life?";
        let information = vec!["The meaning of life is 42.".to_string()];
        let prompt = build_answer_question_prompt(question, information);

        assert!(prompt.contains(question));
    }

    #[test]
    fn generates_prompt_including_relevant_information() {
        let question = "What is the meaning of life?";
        let information = vec![
            "The meaning of life is 42.".to_string(),
            "Life is short but also long.".to_string(),
        ];
        let prompt = build_answer_question_prompt(question, information);

        assert!(prompt.contains("The meaning of life is 42."),);
    }

    #[test]
    fn truncates_prompt_to_3000_characters() {
        let question = "What is the meaning of life?";
        let super_long_information = vec!["The meaning of life is 42.".repeat(1000).to_string()];
        let prompt = build_answer_question_prompt(question, super_long_information);

        assert_eq!(prompt.len(), MAX_PROMPT_LENGTH);
    }
}
