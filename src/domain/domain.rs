use Flat::*;

pub enum Flat<T> {
    FlatBot,
    FlatTop,
    FlatElem(T),
}