macro_rules! newtype_id {
    ($($id_name:ident),*) => {
        pastey::paste! {
            $(
                #[derive(Eq, PartialEq, Copy, Clone, Debug,
                         sqlx::Type, serde::Serialize, serde::Deserialize)]
                #[sqlx(transparent)]
                #[doc = "Newtype wrapper for " $id_name]
                pub struct [<$id_name Id>](pub i64);


                impl std::fmt::Display for [<$id_name Id>] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }
            )*
        }
    };
}

newtype_id!(Attachment);

mod post;
mod user;

pub use user::*;

pub use post::*;
