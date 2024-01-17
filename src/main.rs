use tokio::runtime::Runtime;
mod brain;
mod ui;

async fn app() {
    let vectordb_client = brain::vectordb::build_client();
    let llm_client = brain::llm::build_client();

    ui::write_introduction();

    loop {
        let question: String = ui::read_question();

        let answer = brain::ask(&llm_client, &vectordb_client, &question).await;

        match answer {
            Ok(answer) => {
                ui::write_response(&answer);
            }
            Err(_) => {
                ui::write_error();
            }
        }
    }
}

fn main() {
    Runtime::new()
        .expect("Failed to create runtime")
        .block_on(app());
}
