///
/// easily creates an api client
///
/// ```rust
/// api_client_macro::generate!(ApiClient, {
///     user {
///         get     "user/{id}": get_by_id(id: &str),
///         delete  "user/{id}": delete_by_id(id: &str),
///         post    "user": create(),
///         get     "users": list()
///     },
///     contact {
///         get     "contact/{id}": get_by_id(id: &str),
///         delete  "contact/{id}": delete_by_id(id: &str),
///         post    "contact": create(),
///         get     "contact": list()
///     }
/// });
/// ```
///
#[macro_export]
macro_rules! generate {
    ($client_type:ident, {
        $(
            $resource:ident {
                $(
                    $method:ident $url:literal: $function_name:ident($($param:ident : $type:ty),*)
                ),* $(,)?
            }
        ),* $(,)?
    }) => {
        paste::paste! {
            pub struct [<$client_type Builder>]<T> {
                base_url: String,
                client: T,
            }

            impl<T> [<$client_type Builder>]<T> where T: Default {
                pub fn new(base_url: &str, client: Option<T>) -> Self {
                    Self {
                        base_url: base_url.to_string(),
                        client: client.unwrap_or_default(),
                    }
                }
            }

            impl [<$client_type Builder>]<reqwest::blocking::Client> {
                $(
                    $(
                        #[allow(dead_code)]
                        pub fn [<$resource _ $function_name>](&self $(,$param: $type)*) -> reqwest::blocking::RequestBuilder {
                            let url = format!(concat!("{}/", $url), self.base_url $(, $param = $param)*);
                            println!("{} {}", stringify!([<$method:upper>]), &url);
                            self.client.$method(&url)
                        }
                    )*
                )*
            }

            impl [<$client_type Builder>]<reqwest::Client> {
                $(
                    $(
                        #[allow(dead_code)]
                        pub fn [<$resource _ $function_name>](&self $(,$param: $type)*) -> reqwest::RequestBuilder {
                            let url = format!(concat!("{}/", $url), self.base_url $(, $param = $param)*);
                            println!("{} {}", stringify!([<$method:upper>]), &url);
                            self.client.$method(&url)
                        }
                    )*
                )*
            }
        }
    };
}
