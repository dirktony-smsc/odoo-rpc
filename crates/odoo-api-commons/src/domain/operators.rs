macro_rules! operators {
($($name:ident = $value:literal,)*) => {
    $(
        pub const $name: &str = $value;
    )*
};
}

operators! {
    EQUALS_TO = "=",
    NOT_EQUALS_TO = "!=",
    GREATER_THAN = ">",
    GREATER_THAN_OR_EQUAL_TO = ">=",
    LESS_THAN = "<",
    LESS_THAN_OR_EQUAL_TO = "<=",
    UNSET_OR_EQUALS_TO = "=?",
    EQUALS_LIKE = "=like",
    NOT_EQUALS_LIKE = "not =like",
    LIKE = "like",
    NOT_LIKE = "not like",
    ILIKE = "ilike",
    NOT_ILIKE = "not ilike",
    IN = "in",
    NOT_IN = "not in",
    CHILD_OF = "child_of",
    PARENT_OF = "parent_of",
    ANY = "any",
    NOT_ANY = "not any",
    ANY_NOT = "any!",
    NOT_ANY_NOT = "not any!",
}
