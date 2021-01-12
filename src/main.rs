use fluent_syntax::ast;

fn main() {
    let resource_string = std::fs::read_to_string("test.ftl").unwrap();

    // let resource_string = resource_string.replace("\u{000D}\n", "\n");

    println!("Contents of test.ftl:\n\n{}\n", &resource_string);

    let resource = match fluent::FluentResource::try_new(resource_string) {
        Ok(resource) => {
            resource
        }
        Err((resource, errors)) => {
            eprintln!("The following errors occurred while parsing test.ftl:\n{:?}", errors);
            resource
        }
    };

    find_entry(&resource, "multiline-message-indented-breaks");
    find_entry(&resource, "multiline-message");
}

fn find_entry<'a>(resource: &'a fluent::FluentResource, id: &str) {
    if let Some(entry) = resource.ast().body.iter().find(|entry| {
        match entry {
            ast::Entry::Message(message) => {
                message.id.name == id
            }
            _ => false,
        }
    }) {
        println!("Successfully found entry: {:#?}", entry);
    } else {
        println!("Error: Could not find entry matching id `{}`", id);
    }
}
