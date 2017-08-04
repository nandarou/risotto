use std::collections::BTreeMap;

pub mod util;
pub mod security;

#[cfg(test)]
mod tests {
    use IsoMsg;

    #[test]
    fn msg_returns_mti() {
        let msg_0200 = IsoMsg::new_with_mti("0200");
        assert_eq!(msg_0200.get_mti(), "0200");

        let msg_0210 = IsoMsg::new_with_mti("0210");
        assert_eq!(msg_0210.get_mti(), "0210");
    }

    #[test]
    fn set_string_values() {
        let mut msg = IsoMsg::new();
        msg.set_string(3, "001000");
        msg.set_string(70, "301");
        assert_eq!("001000", msg.get_string(3));
        assert_eq!("301", msg.get_string(70));
    }

    #[test]
    fn test_pack_simple() {
        let msg = IsoMsg::new_with_mti("0200");

        let packed = msg.pack();
        let expected = "<isomsg>
  <f id=\"0\" v=\"0200\"/>
</isomsg>";

        assert_eq!(expected, packed);
    }

    #[test]
    fn test_pack_with_fields() {
        let mut msg = IsoMsg::new_with_mti("0200");
        msg.set_string(64, "CAFEBABE");

        let packed = msg.pack();
        let expected = "<isomsg>
  <f id=\"0\" v=\"0200\"/>
  <f id=\"64\" v=\"CAFEBABE\"/>
</isomsg>";

        assert_eq!(expected, packed);
    }

    #[test]
    fn test_pack_with_fields_and_subfields() {
        let mut msg = IsoMsg::new_with_mti("0200");
        msg.set_string(64, "CAFEBABE");

        let mut f48 = IsoMsg::sub_field(48);
        f48.set_string(1, "01");
        f48.set_string(2, "02");
        f48.set_string(11, "Eleven");

        msg.set_field(48, f48);

        let packed = msg.pack();
        let expected = "<isomsg>
  <f id=\"0\" v=\"0200\"/>
  <isomsg id=\"48\">
    <f id=\"1\" v=\"01\"/>
    <f id=\"2\" v=\"02\"/>
    <f id=\"11\" v=\"Eleven\"/>
  </isomsg>
  <f id=\"64\" v=\"CAFEBABE\"/>
</isomsg>";

        println!("expected:\n{}", expected);
        assert_eq!(expected, packed);
    }

}

trait IsoField {
    fn to_string(&self, depth: i32) -> String;
    fn get_value(&self) -> &str;
}

struct IsoFieldString {
    id: i32,
    value: String,
}

impl IsoFieldString {
    fn new(id: i32, value: String) -> Self {
        IsoFieldString {
            id: id,
            value: value,
        }
    }
}

impl IsoField for IsoFieldString {
    fn to_string(&self, depth: i32) -> String {
        let mut packed = String::new();
        for _ in 0..depth * 2 {
            packed.push(' ');
        }

        packed.push_str(format!("<f id=\"{}\" v=\"{}\"/>\n",
                                self.id,
                                &self.value.to_string())
            .as_str());
        packed
    }

    fn get_value(&self) -> &str {
        self.value.as_str()
    }
}

pub struct IsoMsg {
    id: i32,
    fields: BTreeMap<i32, Box<IsoField>>,
}

impl IsoMsg {
    pub fn new() -> Self {
        IsoMsg {
            id: 0,
            fields: BTreeMap::new(),
        }
    }

    pub fn sub_field(id: i32) -> Self {
        IsoMsg {
            id: id,
            fields: BTreeMap::new(),
        }
    }

    pub fn new_with_mti(mti: &str) -> Self {
        let mut msg = IsoMsg::new();
        msg.set_string(0, mti);
        msg
    }

    pub fn get_mti(&self) -> &str {
        self.get_string(0)
    }

    pub fn set_string(&mut self, field: i32, value: &str) {
        self.fields.insert(field,
                           Box::new(IsoFieldString::new(field, value.to_string())));
    }

    pub fn set_field(&mut self, field: i32, value: IsoMsg) {
        self.fields.insert(field, Box::new(value));
    }

    pub fn get_string(&self, field: i32) -> &str {
        let v = self.fields.get(&field).unwrap();
        v.get_value()
    }

    pub fn pack(&self) -> String {
        let mut packed = String::new();
        packed.push_str("<isomsg>\n");
        for (_, v) in &self.fields {
            let field = v.to_string(1);
            packed.push_str(field.as_str());
        }
        packed.push_str("</isomsg>");
        packed
    }
}

impl IsoField for IsoMsg {
    fn to_string(&self, depth: i32) -> String {
        let mut packed = String::new();
        for _ in 0..depth * 2 {
            packed.push(' ');
        }

        packed.push_str(format!("<isomsg id=\"{}\">\n", self.id).as_str());
        for (_, v) in &self.fields {
            let field = v.to_string(1);
            for _ in 0..depth * 2 {
                packed.push(' ');
            }

            packed.push_str(field.as_str());
        }
        for _ in 0..depth * 2 {
            packed.push(' ');
        }

        packed.push_str("</isomsg>\n");
        packed
    }

    fn get_value(&self) -> &str {
        "get_value"
    }
}
