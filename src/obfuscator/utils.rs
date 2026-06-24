pub fn index_of<T>(list: &[T], value: T) -> usize
where
    T: PartialEq<T>,
{
    list.iter().position(|v| *v == value).unwrap()
}
