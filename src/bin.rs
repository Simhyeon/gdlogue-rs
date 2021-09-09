use gdlogue::{error::GdlError, models::{Branch, Dialogue, Format, Node, Selection}};

fn main() -> Result<(), GdlError> {
    on_demand()?;
    dotify()?;
    Ok(())
}

fn on_demand() -> Result<(), GdlError> {
    let mut dialogue = Dialogue::new();

    // Add nodes
    dialogue.add_new_node(Node::start_node("Start", "1"))?;
    dialogue.add_new_node(Node::omit_node("1", "3"))?;
    dialogue.add_new_node(
        Node::selection_node(
            "3",
            Some("Speaker A"),
            "What do you think?",
            None,
            vec![
                Selection::new("No", "4"),
                Selection::new("Yes", "end")
            ]
        )
    )?;
    dialogue.add_new_node(
        Node::branch_node(
            "4", 
            None,
            vec![
            Branch::new("inventory", "spear","6"),
            Branch::new("status", "ill","7"),
            ]
        )
    )?;
    dialogue.add_new_node(Node::text_node("6", "Speaker B", "You have a spear?", Some("end")))?;
    dialogue.add_new_node(Node::text_node("7", "Speaker B", "You are ill...", Some("end")))?;
    dialogue.add_new_node(Node::end_node("End"))?;

    // Save to file
    dialogue.serialize(&std::env::current_dir()?.join("out.json"))?;

    Ok(())
}

fn dotify() -> Result<(), GdlError> {
    // Dialogue::deserialize(&std::env::current_dir()?.join("out.json"))?;
    Dialogue::new_file(&std::env::current_dir()?.join("out.json"), Format::Png)?;

    Ok(())
}
