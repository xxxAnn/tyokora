enum Argument {
    Flag(String, u8),
    Command(String, u8),
    AssociatedParameter(String, String, u8),
}

pub fn parse(command: &str) {
    let raw = command.split(" ").collect::<Vec<&str>>();
    let mut data = Vec::new();
    for (k, v) in raw.into_iter().enumerate() {
        data.push(is_param(k as u8, v))
    }
    let _ctx = Context::new(data);
    // Do something cool with the context here, like you could match it to a pattern to create a command setup
}

pub struct Context {
    cmd: String,
    subcommands: Vec<String>,
    flags: Vec<String>,
    parameters: Vec<Parameter>,
}

struct Parameter {
    key: String,
    value: String,
}

fn is_param(k: u8, v: &str) -> Argument {
    return match &v[..1] {
        "-" => { Argument::Flag(String::from(&v[1..]), k) }
        "." => {
            let mut v_mut = String::from(v);
            v_mut = v_mut.replace(".", "");
            let slices: Vec<&str> = v_mut.split(":").collect();
            if slices.len() != 2 { panic!("Invalid command") } // Will panic for any text (\..*?\:{0,1}\s)
            Argument::AssociatedParameter(String::from(slices[0]), String::from(slices[1]), k)
        }
        _ => { Argument::Command(String::from(v), k) }
    }
}

impl Context {
    fn new(data: Vec<Argument>) -> Self {
        let mut subcommands = Vec::new();
        let mut parameters = Vec::new();
        let mut flags = Vec::new();
        let mut cmd = None;
        for element in data.iter() {
            match element {
                Argument::Flag(value, _) => { flags.push(String::from(value)) }
                Argument::Command(value, index) => {
                    if *index == 0 {
                        cmd = Some(String::from(value));
                    } else {
                        subcommands.push(String::from(value));
                    }
                }
                Argument::AssociatedParameter(key, value, _) =>  { parameters.push(Parameter {key: String::from(key), value: String::from(value)})}
            }
        }

        Context { cmd: cmd.unwrap(), subcommands, parameters, flags } // Will panic for any text ((\.|\-).*\s)
    }
}
