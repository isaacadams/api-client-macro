# Features

- 🎉 supports both async and blocking reqwest clients

# Usage

A reqwest powered REST API client can be generated using `api_client_macro::generate!` provided the following syntax.

```rust
api_client_macro::generate!(ApiClient, {
    user {
        #[get "user/{}"]
        get_by_id(id: &str),

        #[delete "user/{}"]
        delete_by_id(id: &str),

        #[post "user"]
        create(),

        #[get "users"]
        list()
    },
    contact {
        #[get "contact/{}"]
        get_by_id(id: &str),

        #[delete "contact/{}"]
        delete_by_id(id: &str),

        #[post "contact"]
        create(),

        #[get "contact"]
        list()
    }
});
```

After compilation, the following code is available.

```rust
async fn main_async() {
    let client = asynchronous::Builder::new("base_url", None);
    client.contact_create().body("<body>").send().await.unwrap();
    client.contact_get_by_id("<id>").send().await.unwrap();
    client.user_list().query(&[("email", "<email>")]).send().await.unwrap();
}

fn main_blocking() {
    let client = blocking::Builder::new("base_url", None);
    client.contact_create().body("<body>").send().unwrap();
    client.contact_get_by_id("<id>").send().unwrap();
    client.user_list().query(&[("email", "<email>")]).send().unwrap();
}
```
