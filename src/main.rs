extern crate nom_sql;

fn main() {
    let r_txt = "SELECT * FROM Article LEFT JOIN ";

    // we process all queries in lowercase to avoid having to deal with capitalization in the
    // parser.
    let q_bytes = String::from(r_txt.trim()).into_bytes();

    nom_sql::selection(&q_bytes);
}
