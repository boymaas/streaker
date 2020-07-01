use barrel::backend::Pg;
use barrel::{types, Migration};

pub fn migration() -> String {
    let mut m = Migration::new();
    println!("Applying: {}", file!());

    m.create_table("members", |t| {
        t.add_column("bucket", types::integer());
        t.add_column("streak_bucket", types::integer());
        t.add_column("streak_total", types::integer());
        t.add_column("balance", types::float());
    });

    m.make::<Pg>()
}
