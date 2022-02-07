use clap::{App, AppSettings, Arg, ArgMatches};
use rusqlite::{Connection, Result};
use std::result;
use std::path;

fn main() {
    let matches = App::new("Todo Manager")
        .version("0.0")
        .author("Ameya Kore")
        .about("Manages Todos")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("Adds a new pending task to the manager")
                .arg(Arg::new("task").help("The task that must be added"))
                .arg(
                    Arg::new("day")
                        .short('D')
                        .long("day")
                        .takes_value(true)
                        .required(false)
                        .help("The component of time in days before this task must be completed"),
                )
                .arg(
                    Arg::new("hour")
                        .short('H')
                        .long("hour")
                        .takes_value(true)
                        .required(false)
                        .help("The component of time in hours before this task must be completed"),
                )
                .arg(
                    Arg::new("minute")
                        .short('M')
                        .long("minute")
                        .takes_value(true)
                        .required(false)
                        .help(
                            "The component of time in minutes before this task must be completed",
                        ),
                ),
        )
        .subcommand(
            App::new("list")
                .about("Lists the tasks to do")
                .arg(
                    Arg::new("pending")
                        .short('p')
                        .long("pending")
                        .required(false)
                        .help("List the pending tasks"),
                )
                .arg(
                    Arg::new("completed")
                        .short('c')
                        .long("completed")
                        .required(false)
                        .help("List the completed tasks"),
                )
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .required(false)
                        .help("List all tasks"),
                )
                .arg(
                    Arg::new("bounded")
                        .short('b')
                        .long("bounded")
                        .required(false)
                        .help("Show time bounded tasks"),
                )
                .arg(
                    Arg::new("day")
                        .short('D')
                        .long("day")
                        .takes_value(true)
                        .required(false)
                        .help("The component of time in days before this task must be completed"),
                )
                .arg(
                    Arg::new("hour")
                        .short('H')
                        .long("hour")
                        .takes_value(true)
                        .required(false)
                        .help("The component of time in hours before this task must be completed"),
                )
                .arg(
                    Arg::new("minute")
                        .short('M')
                        .long("minute")
                        .takes_value(true)
                        .required(false)
                        .help(
                            "The component of time in minutes before this task must be completed",
                        ),
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .takes_value(true)
                        .required(false)
                        .help("The number of tasks to fetch (by defualt 10)"),
                )
                .arg(
                    Arg::new("offset")
                        .short('o')
                        .long("offset")
                        .takes_value(true)
                        .required(false)
                        .help("The offset of tasks to fetch from (by default 0)"),
                ),
        )
        // .subcommand(
        //     App::new("modify")
        //         .about("Modify existing task")
        //         .arg(Arg::new("id").help("The id of the task to modify"))
        //         .arg(Arg::new("field").help("The field that has to be modified"))
        //         .arg(Arg::new("value").help("The new value of the field")),
        // )
        .subcommand(App::new("setup").about("Setup database"))
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .required(false)
                .takes_value(true)
                .help("Get info about the task"),
        )
        .arg(
            Arg::new("delete")
                .long("delete")
                .required(false)
                .takes_value(true)
                .help("Delete the task with id"),
        )
        .arg(
            Arg::new("pending")
                .long("pending")
                .short('p')
                .required(false)
                .takes_value(true)
                .help("Make this task pending"),
        )
        .arg(
            Arg::new("complete")
                .long("complete")
                .short('c')
                .required(false)
                .takes_value(true)
                .help("Completes this task"),
        )
        .get_matches();

    // println!("{}", matches.value_of("list").unwrap_or("yep"));

    let mut res = match matches.subcommand() {
        Some(("add", sub_matches)) => Some(handle_add(sub_matches)),
        Some(("list", sub_matches)) => Some(handle_list(sub_matches)),
        // Some(("modify", sub_matches)) => Ok(handle_modify(sub_matches)),
        Some(("setup", _)) => Some(handle_setup()),
        _ => None,
    };

    if res.is_none() {
        if let Some(x) = matches.value_of("info") {
            res = Some(handle_info(x));
        }

        if let Some(x) = matches.value_of("delete") {
            res = Some(handle_delete(x));
        }

        if let Some(x) = matches.value_of("pending") {
            res = Some(handle_convert(x, true));
        }

        if let Some(x) = matches.value_of("complete") {
            res = Some(handle_convert(x, false));
        }
    }

    if res.is_none() {
        panic!("Something happened")
    } else {
        match res.unwrap() {
            Ok(f) => println!("{}", f),
            Err(r) => println!("{}", r),
        }
    }
}

pub fn handle_add(sub_matches: &ArgMatches) -> result::Result<String, String> {
    let conn;

    if let Ok(y) = get_connection() {
        conn = y;
    } else {
        return Err(String::from("Failed to get connection to db"));
    }

    let task;

    if let Some(y) = sub_matches.value_of("task") {
        task = y;
    } else {
        return Err(String::from("No Task found"));
    }
    let t1 = sub_matches.value_of("day");
    let t2 = sub_matches.value_of("hour");
    let t3 = sub_matches.value_of("minute");

    if t1.is_none() && t2.is_none() && t3.is_none() {
        if let Err(_) = conn.execute(
            "INSERT INTO task(todo, is_completed) VALUES (?1, ?2)",
            &[&task, "0"],
        ) {
            return Err(String::from("Error inserting into db"));
        }
    } else {
        let query = format!("INSERT INTO task(todo, is_completed, due_on) VALUES(?1, ?2, datetime('now', 'localtime', '+{} days', '+{} hours', '+{} minutes')) ", t1.unwrap_or("0"), t2.unwrap_or("0"), t3.unwrap_or("0"));

        // println!("{}", query.as_str());

        if let Err(_) = conn.execute(query.as_str(), &[&task, "0"]) {
            // println!("{:?}", e);
            return Err(String::from("Error inserting into db"));
        }
    }

    Ok(format!("{}\t{}", conn.last_insert_rowid(), task))
}

pub fn handle_list(sub_matches: &ArgMatches) -> result::Result<String, String> {
    let conn;

    if let Ok(f) = get_connection() {
        conn = f
    } else {
        return Err(String::from("Failed to get connection to db"));
    }

    let p1 = sub_matches.is_present("all");
    let p2 = sub_matches.is_present("pending");
    let p3 = sub_matches.is_present("completed");

    let mut query =
        String::from("SELECT id, todo, strftime('%Y/%m/%d at %H:%M', due_on) from task where ");

    if (!p1 && !p2 && !p3) || (p2) {
        query += "is_completed = 0 "
    } else if p1 {
        //do nothing
        query += " ";
    } else if p3 {
        query += "is_completed = 1 "
    }

    let t1 = sub_matches.value_of("day");
    let t2 = sub_matches.value_of("hour");
    let t3 = sub_matches.value_of("minute");

    let t4 = sub_matches.is_present("bounded");

    if t1.is_none() && t2.is_none() && t3.is_none() {
        // query += "AND due_on IS NULL ";
    } else {
        query += format!("AND due_on > datetime('now', 'localtime') AND due_on < datetime('now', 'localtime', '+{} days', '+{} hours', '+{} minutes') ", t1.unwrap_or("0"), t2.unwrap_or("0"), t3.unwrap_or("0")).as_str();
    }

    if t4 {
        query += "AND due_on IS NOT NULL order by due_on desc "
    } else {
        query += "order by id desc ";
    }
    query += "LIMIT ";

    let l = sub_matches.value_of("limit").unwrap_or("10");

    query += l;

    query += " OFFSET ";

    let o = sub_matches.value_of("offset").unwrap_or("0");

    query += o;

    let mut stmt;

    // println!("{}", query);

    match conn.prepare(query.as_str()) {
        Ok(f) => {
            stmt = f;
        }
        Err(_) => {
            // println!("{:?}", e);
            return Err(String::from("Error in preparing statement"));
        }
    }

    let iter = stmt.query_map([], |row| {
        let f1: Result<String, rusqlite::Error> = row.get(2);
        Ok(IdNameDate {
            x1: row.get(0)?,
            x2: row.get(1)?,
            x3: if f1.is_err() {
                // println!("{:?}",f1.err());
                None
            } else {
                Some(row.get(2).unwrap())
            },
        })
    });

    if let Err(_) = iter {
        return Err(String::from("Error in executing statment"));
    }

    let mut ret = iter
        .unwrap()
        .map(|x| {
            let r = x.unwrap();
            if let Some(f) = r.x3 {
                format!("{}\t{}\t Due on {}\n", r.x1, r.x2, f)
            } else {
                format!("{}\t{}\n", r.x1, r.x2)
            }
        })
        .collect::<String>();

    if let Ok(f) = conn.prepare("SELECT count(*) from task") {
        stmt = f
    } else {
        return Err(String::from("Error in getting count"));
    }

    let iter = stmt.query_map([], |row| {
        let t: i64 = row.get(0)?;
        Ok(t)
        // Ok()
    });

    if let Err(_) = iter {
        return Err(String::from("Error in executing count statment"));
    }

    let t = iter.unwrap().map(|x| x.unwrap()).next().unwrap();

    let diff = t - o.parse::<i64>().unwrap() - l.parse::<i64>().unwrap();

    if diff > 0 {
        ret += format!("and {} tasks more", diff).as_str();
    } else {
        ret.pop();
    }

    Ok(ret)
}

// pub fn handle_modify(sub_matches: &ArgMatches) {}

pub fn handle_convert(id: &str, pending: bool) -> result::Result<String, String> {
    let conn;

    if let Ok(f) = get_connection() {
        conn = f
    } else {
        return Err(String::from("Failed to get connection to db"));
    }

    if let Err(_) = conn.execute(
        "UPDATE task set is_completed = ?1 where id = ?2",
        [if pending { "0" } else { "1" }, id],
    ) {
        return Err(String::from("Failed to execute update statement"));
    }

    Ok(format!(
        "The task has been marked {} successfully",
        if pending { "pending" } else { "completed" }
    ))
}

pub fn handle_setup() -> result::Result<String, String> {
    let conn;

    if let Ok(f) = get_connection() {
        conn = f
    } else {
        return Err(String::from("Failed to start db"));
    }

    if let Err(_) = conn.execute("DROP TABLE IF EXISTS task", []) {
        return Err(String::from("Error in dropping table"));
    }

    if let Err(_) = conn.execute("CREATE TABLE task( id integer primary key autoincrement, todo TEXT, info TEXT, due_on TEXT DEFAULT NONE, is_completed INTEGER)", []) {
        return Err(String::from("Error in creating table"));
    }

    Ok(String::from("Setup completed Successfully"))
}

pub fn handle_info(id: &str) -> result::Result<String, String> {
    let conn;

    if let Ok(f) = get_connection() {
        conn = f
    } else {
        return Err(String::from("Failed to get connection to db"));
    }

    let mut stmt;

    if let Ok(f) = conn.prepare(
        format!(
            "SELECT id, todo, strftime('%Y/%m/%d at %H:%M', due_on) from task where id = {}",
            id
        )
        .as_str(),
    ) {
        stmt = f;
    } else {
        return Err(String::from("Error in preparing statement"));
    }

    let iter = stmt.query_map([], |row| {
        let f1: Result<String, rusqlite::Error> = row.get(2);
        Ok(IdNameDate {
            x1: row.get(0)?,
            x2: row.get(1)?,
            x3: if f1.is_err() {
                None
            } else {
                Some(row.get(2).unwrap())
            },
        })
    });

    let mut ret = iter
        .unwrap()
        .map(|x| {
            let r = x.unwrap();
            if let Some(f) = r.x3 {
                format!("{}\t{}\t Due on {}\n", r.x1, r.x2, f)
            } else {
                format!("{}\t{}\n", r.x1, r.x2)
            }
        })
        .collect::<String>();

    ret.pop();

    Ok(ret)
}

pub fn handle_delete(id: &str) -> result::Result<String, String> {
    let conn;

    if let Ok(f) = get_connection() {
        conn = f
    } else {
        return Err(String::from("Failed to get connection to db"));
    }

    if let Err(_) = conn.execute("DELETE from task where id = ?1", [id]) {
        return Err(String::from("Failed to execute update statement"));
    }

    Ok(String::from("The task has been deleted successfully"))
}

pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    let path = if let Some(x) = home::home_dir() {
        x.join(path::PathBuf::from("todo.db"))
    } else {
        path::PathBuf::from("todo.db")
    };
    return Connection::open(path);
}

pub struct IdNameDate {
    x1: i64,
    x2: String,
    x3: Option<String>,
}
