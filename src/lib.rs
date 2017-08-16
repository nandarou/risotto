use std::collections::BTreeMap;

pub mod util;
pub mod security;
#[cfg(test)]
mod tests;

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

struct IsoFieldBinary {
    id: i32,
    value: String,
}

impl IsoFieldBinary {
    fn new(id: i32, value: String) -> Self {
        IsoFieldBinary {
            id: id,
            value: value,
        }
    }
}

impl IsoField for IsoFieldBinary {
    fn to_string(&self, depth: i32) -> String {
        let mut packed = String::new();
        for _ in 0..depth * 2 {
            packed.push(' ');
        }

        packed.push_str(format!("<f id=\"{}\" v=\"{}\" t=\"bin\"/>\n",
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

    pub fn set_binary(&mut self, field: i32, value: &str) {
        self.fields.insert(field,
                           Box::new(IsoFieldBinary::new(field, value.to_string())));
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
        packed.push_str("</isomsg>\n");
        packed
    }
}

impl IsoField for IsoMsg {
    fn to_string(&self, depth: i32) -> String {
        let mut packed = String::new();
        let mut indent = String::new();
        for _ in 0..depth * 2 {
            indent.push(' ');
        }

        packed.push_str(format!("{}<isomsg id=\"{}\">\n", indent.as_str(), self.id).as_str());
        for (_, v) in &self.fields {
            let field = v.to_string(1);
            packed.push_str(indent.as_str());
            packed.push_str(field.as_str());
        }

        packed.push_str((format!("{}</isomsg>\n", indent.as_str()).as_str()));
        packed
    }

    fn get_value(&self) -> &str {
        "get_value"
    }
}
