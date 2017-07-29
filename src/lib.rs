use std::collections::HashMap;

pub mod util;
pub mod security;

#[cfg(test)]
mod tests {
    use IsoMsg;

    #[test]
    fn msg_returns_mti() {
        let msg = IsoMsg::new_with_mti("0200");

        assert_eq!(msg.get_mti(), "0200");
    }

    #[test]
    fn set_string_values() {
        let mut msg = IsoMsg::new();
        msg.set_string(70, "301");
        let value = msg.get_string(70);

        assert_eq!(value, "301");
    }
}

#[derive(Debug)]
pub struct IsoField<T> {
    value: T,
}

impl<T> IsoField<T> {
    fn new(value: T) -> IsoField<T> {
        IsoField { value: value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

#[derive(Debug)]
pub struct IsoMsg {
    fields: HashMap<i32, IsoField<String>>,
}

impl IsoMsg {
    pub fn new() -> IsoMsg {
        IsoMsg { fields: HashMap::new() }
    }

    pub fn new_with_mti(mti: &str) -> IsoMsg {
        let mut msg = IsoMsg::new();
        msg.set_string(0, mti);
        msg
    }

    pub fn get_mti(&self) -> &str {
        self.get_string(0)
    }

    pub fn set_string(&mut self, field: i32, value: &str) {
        self.fields.insert(field, IsoField::new(value.to_string()));
    }

    pub fn get_string(&self, field: i32) -> &str {
        let value = self.fields.get(&field);
        value.unwrap().get()
    }

    // temp
    pub fn build_echo(hwid: i32) -> String {
        let echo = format!("<isomsg><f id=\"0\" v='0800'/><isomsg id=\"48\"><f id=\"1\" \
                            v=\"ABCD{}\"/><f id=\"2\" v=\"16.00.00\"/></isomsg><f id=\"70\" \
                            v=\"301\"/></isomsg>\n",
                           hwid);
        echo.to_string()
    }

    // temp otra vez ;)
    pub fn build_purchase() -> String {
        "<isomsg><f id='0' v='0200'/><f id='53' v='FFFF9876543210E00001'/><f id='64' \
         v='DEADBEEF'/></isomsg>\n"
            .to_string()
    }
}
