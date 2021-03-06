use crate::*;
use extend::ext;

table! {
    users {
        id -> Integer,
        name -> Text,
        country_id -> Integer,
    }
}

table! {
    countries {
        id -> Integer,
        name -> Text,
    }
}

#[ext(name = UserScopes)]
impl<T> T
where
    T: Into<Query<users::table>>,
{
    fn named(self, name: &str) -> Query<users::table> {
        self.filter(users::name.eq(name))
    }

    fn in_country(self, country_id: i32) -> Query<users::table> {
        self.inner_join(countries::table.on(countries::id.eq(users::country_id)))
            .merge(countries::table.with_id(country_id))
    }
}

#[ext(name = CountryScopes)]
impl<T> T
where
    T: Into<Query<countries::table>>,
{
    fn with_id(self, country_id: i32) -> Query<countries::table> {
        self.filter(countries::id.eq(country_id))
    }
}

#[test]
fn scopes() {
    let (query, mut binds) = users::table
        .named("Bob")
        .in_country(1)
        .select(users::star)
        .to_sql();

    assert_eq!(
        query,
        r#"SELECT "users".* FROM "users" INNER JOIN "countries" ON "countries"."id" = "users"."country_id" WHERE "users"."name" = $1 AND "countries"."id" = $2"#
    );
    assert_eq!(binds.next(), Some(Bind::String("Bob".to_string())));
    assert_eq!(binds.next(), Some(Bind::I32(1)));
    assert_eq!(binds.next(), None);
}
