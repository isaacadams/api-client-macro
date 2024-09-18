generate!(ApiClient, {
    user {
        get     "user/{id}": get_by_id(id: &str),
        delete  "user/{id}": delete_by_id(id: &str),
        post    "user": create(),
        get     "users": list()
    },
    contact {
        get     "contact/{id}": get_by_id(id: &str),
        delete  "contact/{id}": delete_by_id(id: &str),
        post    "contact": create(),
        get     "contact": list()
    }
});

/* fn s() {
    let s = ApiClientBuilder::new("base_url",None);
} */