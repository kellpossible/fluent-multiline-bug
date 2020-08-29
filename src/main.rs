use fluent_syntax::ast;

fn main() {
    let resource_string = std::fs::read_to_string("test.ftl").unwrap();

    println!("Contents of test.ftl:\n\n{}\n", &resource_string);

    let resource = match fluent::FluentResource::try_new(resource_string) {
        Ok(resource) => {
            resource
        }
        Err((resource, errors)) => {
            println!("The following errors occurred while parsing test.ftl:\n{:?}", errors);
            resource
        }
    };

    find_entry(&resource, "simple-message");
    find_entry(&resource, "simple-arg");
    find_entry(&resource, "multiline-message");
    find_entry(&resource, "multiline-message-selector");
}

fn find_entry<'a>(resource: &'a fluent::FluentResource, id: &str) {
    if let Some(entry) = resource.ast().body.iter().find(|resource_entry| {
        match resource_entry {
            ast::ResourceEntry::Entry(entry) => {
                match entry {
                    ast::Entry::Message(message) => {
                        message.id.name == id
                    }
                    _ => false,
                }
            },
            _ => false,
        }
    }) {
        println!("Successfully found entry: {:?}", entry);
    } else {
        println!("Error: Could not find entry matching id `{}`", id);
    }
}
