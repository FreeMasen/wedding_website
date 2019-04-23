use postgres::{
    Connection,
    TlsMode,
};
use warp::{
    Filter,
    post2,
    body::form,
    path,
    Reply,
    log,
    http::Response,
    header,
    fs::dir,
};

static CONNECTION_STRING: &str = include_str!("sql_conn");

fn main() {
    pretty_env_logger::init();
    let bbq_rsvp = post2()
        .and(path("bbq-rsvp"))
        .and(header::<String>("referer"))
        .and(form())
        .map(rsvp_bbq);
    warp::serve(bbq_rsvp.or(dir("public")).with(log("mashton.party")))
        .run(([127, 0, 0, 1], 9211));
}



fn rsvp_bbq(referer: String, rsvp: Vec<(String, String)>) -> impl Reply {
    let rsvp = match BbqRsvp::from(rsvp) {
        Ok(rsvp) => rsvp,
        Err(e) => return redirect(&format!("{}/error?reason={}", referer, e)),
    };
    let endpoint = match insert_bbq_rsvp(&rsvp) {
        Ok(_) => format!("success?{}", rsvp.as_query_string()),
        Err(e) => format!("error?reason={}", e),
    };
    let new_location = if referer.ends_with("/") {
        format!("{}{}", referer, endpoint)
    } else {
        format!("{}/{}", referer, endpoint)
    };
    println!("redirecting to {}", new_location);
    redirect(&new_location)
}

fn redirect(location: &str) -> impl Reply {
    Response::builder()
        .status(303)
        .header("Location", warp::http::header::HeaderValue::from_str(&location).unwrap())
        .body("")
}

fn insert_bbq_rsvp(rsvp: &BbqRsvp) -> Result<(), String> {
    let conn = Connection::connect(CONNECTION_STRING.trim(), TlsMode::None).map_err(|e| format!("Failed to connect to db {}", e))?;
    let trans = conn.transaction().map_err(|e| format!("Unable to open transaction {}", e))?;
    let rows = trans.query(
        "SELECT new_rsvp FROM new_rsvp($1, $2, $3, $4)", 
        &[&rsvp.first_name, &rsvp.last_name, &rsvp.regret_digit(), &rsvp.diet]
    ).map_err(|e| format!("Failed to insert values {}", e))?;
    let row = rows.get(0);
    let id: i32 = row.get(0);
    println!("new id: {}", id);
    let second_insert = trans.prepare(
        "INSERT INTO bbq_guest_name (invite_id, name) VALUES ($1, $2)"
    ).map_err(|e| format!("failed to prepare query {}", e))?;
    for ref name in &rsvp.guest_name {
        second_insert
            .execute(&[&id, name])
            .map_err(|e| 
                format!("Failed on insert of guest {} {}", name, e)
            )?;
    }
    Ok(())
}

#[derive(Debug)]
struct BbqRsvp {
    pub first_name: String,
    pub last_name: String,
    pub regrets: bool,
    pub diet: String,
    pub guest_name: Vec<String>,
}

impl BbqRsvp {
    pub fn regret_digit(&self) -> i32 {
        if self.regrets {
            1
        } else {
            0
        }
    }
    pub fn as_query_string(&self) -> String {
        use urlencoding::encode;
        let first_name = encode(&self.first_name);
        let last_name = encode(&self.last_name);
        format!("first_name={}&last_name={}&party_size={}&regrets={}", first_name, last_name, self.guest_name.len(), self.regret_digit())
    }

    pub fn from(other: Vec<(String, String)>) -> Result<Self, String> {
        let mut first_name: Option<String> = None;
        let mut last_name: Option<String> = None;
        let mut regrets = false;
        let mut diet = String::new();
        let mut guest_name: Vec<String> = vec![];
        let mut missing = vec![];
        for pair in other {
            match (pair.0.as_str(), pair.1) {
                ("first-name", value) => first_name = Some(value),
                ("last-name", value) => last_name = Some(value),
                ("regrets", value) => regrets = value == "on",
                ("guest-name[]", value) => if value != "" {
                    guest_name.push(value)
                },
                ("diet", value) => diet = value,
                _ => (),
            }
        }
        if first_name.is_none() {
            missing.push("First Name");
        }
        if last_name.is_none() {
            missing.push("Last Name");
        }
        if missing.len() > 0 {
            return Err(format!("{:?} are required fields", missing));
        }
        Ok(Self {
            first_name: first_name.unwrap(),
            last_name: last_name.unwrap(),
            regrets,
            diet,
            guest_name,
        })
    }
}
