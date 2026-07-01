#[macro_export]
macro_rules! randomizable_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            pub fn random() -> Self {
                let mut rng = rand::rng();

                let variants = [$( $name::$variant ),*];
                *variants.choose(&mut rng).unwrap()
            }
        }
    };
}

pub fn index_of<T>(list: &[T], value: T) -> usize
where
    T: PartialEq<T>,
{
    list.iter().position(|v| *v == value).unwrap()
}
