///
/// easily creates an api client
///
/// ```rust
/// api_client_macro::generate!(ApiClient, {
///     user {
///         #[get "user/{}"]
///         get_by_id(id: &str),
/// 
///         #[delete "user/{}"]
///         delete_by_id(id: &str),
/// 
///         #[post "user"]
///         create(),
/// 
///         #[get "users"]
///         list()
///     },
///     contact {
///         #[get "contact/{}"]
///         get_by_id(id: &str),
/// 
///         #[delete "contact/{}"]
///         delete_by_id(id: &str),
/// 
///         #[post "contact"]
///         create(),
/// 
///         #[get "contact"]
///         list()
///     }
/// });
/// 
/// fn main() {}
/// 
/// async fn main_async() {
///     let client = asynchronous::Builder::new("base_url", None);
///     client.contact_create().body("<body>").send().await.unwrap();
///     client.contact_get_by_id("<id>").send().await.unwrap();
///     client.user_list().query(&[("email", "<email>")]).send().await.unwrap();
/// }
/// 
/// fn main_blocking() {
///     let client = blocking::Builder::new("base_url", None);
///     client.contact_create().body("<body>").send().unwrap();
///     client.contact_get_by_id("<id>").send().unwrap();
///     client.user_list().query(&[("email", "<email>")]).send().unwrap();
/// }
/// ```
///
#[macro_export]
macro_rules! generate {
    ($client_type:ident, {
        $(
            $resource:ident {
                $(
                    #[$method:ident $url:literal]
                    $function:ident($($param:ident : $type:ty),*)
                ),* $(,)?
            }
        ),* $(,)?
    }) => {
        paste::paste! {
            mod internal {
                pub struct [<$client_type Builder>]<T> {
                    pub(crate) base_url: String,
                    pub(crate) client: T,
                }

                impl<T> [<$client_type Builder>]<T> where T: Default {
                    pub fn new(base_url: &str, client: Option<T>) -> Self {
                        Self {
                            base_url: base_url.to_string(),
                            client: client.unwrap_or_default(),
                        }
                    }
                }
            }

            pub mod blocking {
                use super::internal::*;

                pub type Client = reqwest::blocking::Client;
                pub type Builder = [<$client_type Builder>]<Client>;

                pub fn new(base_url: &str, client: Option<Client>) -> Builder {
                    Builder::new(base_url, client)
                }

                impl Builder {
                    $(
                        $(
                            #[allow(dead_code)]
                            pub fn [<$resource _ $function>](&self $(,$param: $type)*) -> reqwest::blocking::RequestBuilder {
                                let url = format!(concat!("{}/", $url), self.base_url $(, $param )*);
                                println!("{} {}", stringify!([<$method:upper>]), &url);
                                self.client.$method(&url)
                            }
                        )*
                    )*
                }
            }

            pub mod asynchronous {
                use super::internal::*;

                pub type Client = reqwest::Client;
                pub type Builder = [<$client_type Builder>]<Client>;

                pub fn new(base_url: &str, client: Option<Client>) -> Builder {
                    Builder::new(base_url, client)
                }

                impl Builder {
                    $(
                        $(
                            #[allow(dead_code)]
                            pub fn [<$resource _ $function>](&self $(,$param: $type)*) -> reqwest::RequestBuilder {
                                let url = format!(concat!("{}/", $url), self.base_url $(, $param )*);
                                println!("{} {}", stringify!([<$method:upper>]), &url);
                                self.client.$method(&url)
                            }
                        )*
                    )*
                }
            }
        }
    };
}
