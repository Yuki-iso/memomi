match bson::to_document(&f) {
    Ok(doc) => println!("bson: {}", doc),
    Err(err) => eprintln!("Error converting to BSON document: {}", err),
}