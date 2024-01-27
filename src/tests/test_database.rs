use super::*;

#[test]
fn new() {
    let db = Database::new("data/test_new");
    assert_eq!(db.len(), 0);
}

#[test]
fn create_collection() {
    let mut db = Database::new("data/test_create_collection");

    let records = gen_records::<128>(100);
    let collection: Collection<usize, 128, 32> =
        db.create_collection("test", None, Some(&records));

    assert_eq!(collection.len(), 100);
    assert_eq!(db.len(), 1);
}

#[test]
fn get_collection() {
    let db = create_test_database("data/test_get_collection");
    let collection: Collection<usize, 128, 32> = db.get_collection("vectors");
    assert_eq!(collection.len(), 100);
}

#[test]
fn save_collection_new() {
    let mut db = Database::new("data/test_save_collection_new");

    // Create a collection from scratch.
    let config = Config::default();
    let mut collection: Collection<usize, 128, 32> = Collection::new(&config);
    collection.insert(gen_records(1).first().unwrap());

    db.save_collection("new", &collection);
    assert_eq!(collection.len(), 1);
    assert_eq!(db.len(), 1);
}

#[test]
fn save_collection_update() {
    let mut db = create_test_database("data/test_save_collection_update");

    // Update the collection.
    let mut collection: Collection<usize, 128, 32> =
        db.get_collection("vectors");
    collection.insert(gen_records(1).first().unwrap());

    db.save_collection("vectors", &collection);
    assert_eq!(collection.len(), 101);
    assert_eq!(db.len(), 1);
}

#[test]
fn delete_collection() {
    let mut db = create_test_database("data/test_delete_collection");
    db.delete_collection("vectors");
    assert_eq!(db.len(), 0);
}