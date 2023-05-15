# Usage

A reqwest powered REST API client can be generated using `api_client_macro::generate!` provided the following syntax.

```rust
api_client_macro::generate!(ApiClient, {
    user {
        get_by_id: get "user/{id}" id: &str,
        delete_by_id: delete "user/{id}" id: &str,
        create: post "user",
        list: get "users"
    },
    contact {
        get_by_id: get "contact/{id}" id: &str,
        delete_by_id: delete "contact/{id}" id: &str,
        create: post "contact",
        list: get "contact"
    }
});
```

After compilation, the following code is available.

```rust
fn main() {
    let client = ApiClientBuilder::new("<api-base-url>", None);
    client.contact_create().body("<body>").send().unwrap();
    client.contact_get_by_id("<id>").send().unwrap();
    client.user_list().query(&[("email", "<email>")]).send().unwrap();
}
```
