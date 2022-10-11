fn main() {
    //let qa_model =
    //rust_bert::pipelines::question_answering::QuestionAnsweringModel::new(Default::default())
    //.unwrap();

    //let question = String::from("Where does Amy live ?");
    //let mut question = String::new();
    //std::io::stdin()
    //.read_line(&mut question)
    //.expect("Error parsing question.");
    //let context = String::from("Answer the question as comedically as possible.");

    //let answers = qa_model.predict(
    //&[rust_bert::pipelines::question_answering::QaInput { question, context }],
    //1,
    //32,
    //);
    //println!("{answers:?}");
    use rust_bert::pipelines::conversation::{ConversationManager, ConversationModel};
    let conversation_model = ConversationModel::new(Default::default()).unwrap();
    let mut conversation_manager = ConversationManager::new();

    loop {
        println!("Ask me a question.\n");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut context =
            String::from("Answer my question as if you are omnipotent and above all.");
        context.push_str(&line);

        let conversation_id = conversation_manager.create(&context);
        let output = conversation_model.generate_responses(&mut conversation_manager);

        println!("{}\n", output[&conversation_id]);
        conversation_manager.remove(&conversation_id);
    }
}
