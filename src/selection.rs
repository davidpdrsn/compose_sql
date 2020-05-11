use crate::{Table, Column, WriteSql};
use crate::binds::BindCount;
use std::fmt::{self, Write};
use itertools::Itertools;
use itertools::Position;

#[derive(Debug)]
pub enum Selection {
    Star(Table),
    Column(Column),
    List(Vec<Selection>),
}

impl WriteSql for Selection {
    fn write_sql<W: Write>(&self, f: &mut W, bind_count: &mut BindCount) -> fmt::Result {
        match self {
            Selection::Star(table) => {
                table.write_sql(f, bind_count)?;
                write!(f, ".*")
            }
            Selection::Column(col) => col.write_sql(f, bind_count),
            Selection::List(cols) => {
                for item in cols.into_iter().with_position() {
                    match item {
                        Position::First(col) | Position::Middle(col) => {
                            col.write_sql(f, bind_count)?;
                            write!(f, ", ")?;
                        }
                        Position::Last(col) | Position::Only(col) => {
                            col.write_sql(f, bind_count)?;
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

macro_rules! impl_select_dsl {
    (
        $first:ident, $second:ident,
    ) => {
        #[allow(warnings)]
        impl<$first, $second> Into<Selection> for ($first, $second)
        where
            $first: Into<Selection>,
            $second: Into<Selection>,
        {
            fn into(self) -> Selection {
                let ($first, $second) = self;
                let mut cols = vec![$first.into(), $second.into()];
                Selection::List(cols)
            }
        }
    };

    (
        $head:ident, $($tail:ident),*,
    ) => {
        #[allow(warnings)]
        impl<$head, $($tail),*> Into<Selection> for ($head, $($tail),*)
        where
            $head: Into<Selection>,
            $( $tail: Into<Selection> ),*
        {
            fn into(self) -> Selection {
                let ($head, $($tail),*) = self;
                let mut cols = vec![
                    $head.into(),
                    $( $tail.into(), )*
                ];
                Selection::List(cols)
            }
        }

        impl_select_dsl!($($tail),*,);
    };
}

impl_select_dsl!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
    T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32,
);