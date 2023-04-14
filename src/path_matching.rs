#![allow(dead_code)]
fn path_matching(prefix: &str, requested_path: &str) -> bool {
    let prefixes = prefix.split('/');
    let requested_paths = requested_path.split('/')
            .map(|p| Some(p))
            .chain(std::iter::once(None));

    for (x,y) in prefixes.zip(requested_paths) {
        match y {
            Some(y) => {
                if (x != "*") && (x != y) {
                    return false;
                }
            }

            None => return false,
            
        }
    }

    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(path_matching("/v1/publishers", "/v1/publishers"));
    assert!(path_matching("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(path_matching("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!path_matching("/v1/publishers", "/v1"));
    assert!(!path_matching("/v1/publishers", "/v1/publishersBooks"));
    assert!(!path_matching("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(path_matching(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(path_matching(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(path_matching(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!path_matching("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!path_matching(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {
    println!("Hello World");
}