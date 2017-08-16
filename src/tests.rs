use util;
use util::*;
use security::*;
use IsoMsg;

#[test]
fn test_ansi_example() {
    let expected_mac = util::from_hex("9CCC7817");

    let pek = util::from_hex("042666B49184CFA368DE9628D0397BC9");
    let data = util::from_hex("343031323334353637383930394439383700000000000000");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}

#[test]
fn test_data_not_divisible_by_8() {
    let expected_mac = util::from_hex("8BD6699E");

    let pek = util::from_hex("042666B49184CFA368DE9628D0397BC9");
    let data = util::from_hex("34303132333435363738393039443938370000000000000001");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}

#[test]
fn test_real_msg_mac() {
    let expected_mac = util::from_hex("3FFAE34A");

    let pek = util::from_hex("DA4A499174AC32C03AE2452C79DD6668");
    let data = util::from_hex("044D1F8B08000000000000008592CD4EC3301084EF3C45940BB77677EDF58F9470A8688E088927A05050C44F24027D7E9C6C4562B3081FA2E89B893D3B4ED38FC3DBF87C755155CD53D53FB635D4D5293D09A0DEAEB0110C0885606781B1C05EFC1E3D1ACBCC998838ABDE7B0C982B342B04E8FFFA966787E8B912244AF426E3247BC63C20F179A27C4CA1EC80D8C618D3B4800E5A0C8048D3F48E27164D286A90DE9227C732299DA1B42D0AD5135AD24B252E8A75C939F3021A0DDA7C8766BBBADBF5C1463BF8F6FEEBF572ACEE8E1FA7FEE138AA216E86E1450D02C6E9590CB87FC2042D8C257B8065919A06FD0668F3737FDB5F7F328B2F746187A1DB43B7BB666B52B2CFB63EF4EFD93DB10CD2A5158377C94708FBF9F60BFF2ABC432D3CE869F5E2D4DA4ACA2A752AF52A0D2A45D0B1CCC125A6B57BE97B79FB061696CFBC4D040000");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}

#[test]
fn test_real_response_mac() {
    let expected_mac = util::from_hex("60226317");
    let pek = util::from_hex("042666B49184CFA368DE9628D0397BC9");

    let data = util::from_hex("035F1F8B08000000000000008553CB6AC33010BCF72B8C3F20D9D56B25480AF50B02A5D0D01E7A0A7DE3436348DC42FFBEB236A92D23880E3ACC8C7634BBD2AA3D765FC7CFEBAB2C5B7D64EDDB3A873CFBF1BB40C8971358320C0810132A10286630B19E242229D231891858A78C45113322300290C0A24C9DD54121110829666C60B42319E1826BBAB88ED0A744714C46B501A19573CEA70534B0460BE833FAF4460F98937676D0B1898D9B03E7EEC430E73F47E7193023F2011A33052119C7D2F1F6019F813205AAB8C26A3999F8D458A58C3539B226695376FBFEF9B5CF1E7E0FDD85D236555A09550C8F292C4CC6435A085C9C27F44F6A6E757FF87E9F31C42FA0AACBDBCD5D5DED8AA7DD4D79FFB8D9D6DBC4154FD5D8CB36B640DBD4D0149556D277AD5FE72FED3E9A9BE6E88D5FCE92F13AFF4D6A0E30D18F2E7F847A42D85F030000");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}

#[test]
fn test_decrypt() {
    let key = util::from_hex("68DE9628D03984C9");
    let encrypted = util::from_hex("C5BC8EC1621EF5EE");
    let expected = util::from_hex("F720DC72F93339DF");

    let decrypted = des_decrypt(&key, &encrypted).unwrap();

    assert_eq!(expected, decrypted);
}

#[test]
fn test_xor() {
    let a = from_hex("AB");
    let b = from_hex("CD");

    let xor = xor(a.as_slice(), b.as_slice());

    assert_eq!(xor, from_hex("66"));
}

#[test]
fn test_gzip() {
    let input = [0, 1, 2, 3];
    let expected = from_hex("1F8B08000000000000006360646206001386B98B04000000");
    let gzipped = gzip(&input);

    println!("expected:\n1F8B08000000000000006360646206001386B98B04000000");
    println!("but was\n{}", to_hex(&gzipped));
    assert_eq!(expected, gzipped);
}

#[test]
fn test_another_gzip() {
    let input = from_hex("012345678901");
    let expected = from_hex("1F8B08000000000000006354764DEF640400EC41EB8006000000");

    let gzipped = gzip(&input);
    assert_eq!(expected, gzipped);
}

#[test]
fn test_more_gzip() {
    let input = from_hex("0123456789012345678901234567890123456789012345678901234567890123456789");
    let expected = "1F8B08000000000000006354764DEF64C44B0000476314E623000000";
    let gzipped = gzip(&input);

    assert_eq!(expected, to_hex(&gzipped));
}

#[test]
fn test_larger_gzip() {
    let input = "3C69736F6D73673E0A20203C662069643D22302220763D2230323030222F3E0A20203C662069643D22332220763D22303031303030222F3E0A20203C662069643D22342220763D2235313030222F3E0A20203C662069643D22372220763D2230383035323234333339222F3E0A20203C662069643D2231312220763D22313831363338222F3E0A20203C662069643D2231322220763D223230313730383035323234333339222F3E0A20203C662069643D2231352220763D22303530383137222F3E0A20203C662069643D2231382220763D2235393733222F3E0A20203C662069643D2232322220763D223930222F3E0A20203C662069643D2232352220763D223030222F3E0A20203C662069643D2233352220763D22353630323534393939303030303136303D3138303131323031303036353030303039333830222F3E0A20203C662069643D2234302220763D22313230222F3E0A20203C662069643D2234312220763D2232222F3E0A20203C69736F6D73672069643D223432223E0A202020203C662069643D22312220763D22373639222F3E0A202020203C662069643D22322220763D22222F3E0A202020203C662069643D22332220763D22222F3E0A202020203C662069643D22342220763D22373639222F3E0A20203C2F69736F6D73673E0A20203C69736F6D73672069643D223433223E0A202020203C662069643D22312220763D225061756C2773205365727669636573222F3E0A202020203C662069643D22322220763D224E6F6F6B222F3E0A202020203C662069643D22332220763D22303336222F3E0A202020203C662069643D22342220763D2237333036222F3E0A20203C2F69736F6D73673E0A20203C69736F6D73672069643D223438223E0A202020203C662069643D22312220763D2234323462303030303030303030303032222F3E0A202020203C662069643D22322220763D2231372E30322E3030222F3E0A20203C2F69736F6D73673E0A20203C662069643D2235322220763D22384638423138464530464244353433342220743D2262696E222F3E0A20203C662069643D2235332220763D2246464646393837363534333231304530303030312220743D2262696E222F3E0A20203C69736F6D73672069643D223631223E0A202020203C662069643D22312220763D2230222F3E0A202020203C662069643D22322220763D2231222F3E0A202020203C662069643D22332220763D2230222F3E0A202020203C662069643D22342220763D2230222F3E0A202020203C662069643D22352220763D2230222F3E0A202020203C662069643D22362220763D2230222F3E0A202020203C662069643D22372220763D2230222F3E0A202020203C662069643D22382220763D2230222F3E0A202020203C662069643D2231302220763D2230222F3E0A202020203C662069643D2231312220763D2235222F3E0A202020203C662069643D2231322220763D2230222F3E0A20203C2F69736F6D73673E0A3C2F69736F6D73673E0A";
    let input_bytes = from_hex(input);
    let expected = "1F8B08000000000000008592CD4E85400C85F73E0561E3EED276FE3A09B8B8F1B234263E8157AF86F843227A9F5FA04498B1C65910F29D43E7B4A5EE86FE6D78BEBA288AFAA9E81E9B12CAE23C3E09A0AC36D80806844CB0B3E030C341FC0C8EC81A131311715691D11B4E159A15020C7F7DEBA4B203C6902A2C5162300927A919D380B4D4C9DA14EA3C90B331C6B15B400F0D3220D2D4BD77138B86B331C8DC464F8AA5535AA04C5B142A27B4A69F8DC12FDDAE39679E41A3419B56A8ABCD6EB7171BEDE2DBFBAFD7CBA1B83B7D9CBB87D3A086B8E9FB17350818AF6731E0FF09C35A184BF608EB21350D861DD0EE677FD5AF3FD9898F5BDE23B70768F7D7CE9A31D967531EBBF7644F4E1A69C71339F8D147088779FB997F13DEA3161EF4B4FAE0D4B1E5D4A9D4AB34A894558AA063E9C3E598B6EE75DEEBDB379A2420454D040000";
    let gzipped = gzip(&input_bytes);

    println!("expected:\n{}", expected);
    println!("actual:\n{}", to_hex(&gzipped));
    assert_eq!(expected, to_hex(&gzipped));
}

#[test]
fn test_real_msg_gzip() {
    let input = "3C69736F6D73673E0A20203C662069643D22302220763D2230323030222F3E0A20203C662069643D22332220763D22303031303030222F3E0A20203C662069643D22342220763D2235313030222F3E0A20203C662069643D22372220763D2230373331313734373530222F3E0A20203C662069643D2231312220763D22393436383132222F3E0A20203C662069643D2231322220763D223230313730383132303734373530222F3E0A20203C662069643D2231352220763D22333130373137222F3E0A20203C662069643D2231382220763D2235393733222F3E0A20203C662069643D2232322220763D223930222F3E0A20203C662069643D2232352220763D223030222F3E0A20203C662069643D2233352220763D22353630323534393939303030303136303D3138303131323031303036353030303039333830222F3E0A20203C662069643D2234302220763D22313230222F3E0A20203C662069643D2234312220763D2232222F3E0A20203C69736F6D73672069643D223432223E0A202020203C662069643D22312220763D22373639222F3E0A202020203C662069643D22322220763D22222F3E0A202020203C662069643D22332220763D22222F3E0A202020203C662069643D22342220763D22373639222F3E0A20203C2F69736F6D73673E0A20203C69736F6D73672069643D223433223E0A202020203C662069643D22312220763D225061756C2773205365727669636573222F3E0A202020203C662069643D22322220763D224E6F6F6B222F3E0A202020203C662069643D22332220763D22303336222F3E0A202020203C662069643D22342220763D2237333036222F3E0A20203C2F69736F6D73673E0A20203C69736F6D73672069643D223438223E0A202020203C662069643D22312220763D2234323462303030303030303030303131222F3E0A202020203C662069643D22322220763D2231372E30322E3030222F3E0A20203C2F69736F6D73673E0A20203C662069643D2235322220763D22384638423138464530464244353433342220743D2262696E222F3E0A20203C662069643D2235332220763D2246464646393837363534333231304530303030312220743D2262696E222F3E0A20203C69736F6D73672069643D223631223E0A202020203C662069643D22312220763D2230222F3E0A202020203C662069643D22322220763D2231222F3E0A202020203C662069643D22332220763D2230222F3E0A202020203C662069643D22342220763D2230222F3E0A202020203C662069643D22352220763D2230222F3E0A202020203C662069643D22362220763D2230222F3E0A202020203C662069643D22372220763D2230222F3E0A202020203C662069643D22382220763D2230222F3E0A202020203C662069643D2231302220763D2230222F3E0A202020203C662069643D2231312220763D2235222F3E0A202020203C662069643D2231322220763D2230222F3E0A20203C2F69736F6D73673E0A3C2F69736F6D73673E0A";
    let input_bytes = from_hex(input);
    let expected = "1F8B08000000000000008592CD4EC3301084EF3C45940BB766D7F67A6DA9E550D11C11124F40A1A0889F4804FAFCB85E6862B3123E44D137137B669DF5308D6FD3F3D545D3AC9F9AE171D342DB1CD3D300B4DD025BC18050092E0B841566F1B34564C7548A88598DCE0734A562B26200199206CAB7941D1681914B254894C8B6E046F68CE53E867E1A9535859207432EC698DA027AD860004C71524B4F27166DA8C620734B9E124BD3DF92326D514C7B4273FA6C641FC53AE7CCBC825683AEDC61DD2DEE7679B0D50EBEBDFF7ABD9C9ABBC3C77178384C6A889B717C518380F57A160BFE9F30410BE38CDBC379A5BF454B83BC02B33ADF5FF7E74F26F1853E6C31F43BE8B7D7E46C4AF6B969F7C37B714F2445FAB462609F7C0661974FAFFC8BF01EB5F0A0A7D507A78EADA6A452AF5256695029828EA507D5D82CDDF3BCE7B76FB9963E714D040000";

    let gzipped = gzip(&input_bytes);

    assert_eq!(expected, to_hex(&gzipped));
}

#[test]
fn test_real_response_msg() {
    let input = "3c69736f6d73673e0a20203c662069643d22302220763d2230323130222f3e0a20203c662069643d22332220763d22303031303030222f3e0a20203c662069643d22342220763d2231323030222f3e0a20203c662069643d22372220763d2230373331313734373530222f3e0a20203c662069643d2231312220763d22393436383132222f3e0a20203c662069643d2231322220763d223230313730383133313734373530222f3e0a20203c662069643d2231352220763d22333130373137222f3e0a20203c662069643d2231382220763d2235393733222f3e0a20203c662069643d2232322220763d223930222f3e0a20203c662069643d2232352220763d223030222f3e0a20203c662069643d2233352220763d22353630323534393939303030303136303d3138303131323031303036353030303039333830222f3e0a20203c662069643d2233392220763d223938222f3e0a20203c662069643d2234302220763d22313230222f3e0a20203c662069643d2234312220763d2232222f3e0a20203c69736f6d73672069643d223432223e0a202020203c662069643d22312220763d22373639222f3e0a202020203c662069643d22322220763d22222f3e0a202020203c662069643d22332220763d22222f3e0a202020203c662069643d22342220763d22373639222f3e0a20203c2f69736f6d73673e0a20203c69736f6d73672069643d223434223e0a202020203c662069643d22312220763d22353739373836222f3e0a202020203c662069643d22332220763d22436f6e74616374205479726f222f3e0a20203c2f69736f6d73673e0a20203c69736f6d73672069643d223438223e0a202020203c662069643d22312220763d223432344231303030303030303031222f3e0a202020203c662069643d22322220763d2231372e32312e3030222f3e0a202020203c662069643d2235392220763d2274727565222f3e0a202020203c662069643d2237382220763d224445434c494e45445f42595f4143515549524552222f3e0a20203c2f69736f6d73673e0a20203c662069643d2235322220763d22384638423138464530464244353433342220743d2262696e222f3e0a20203c662069643d2235332220763d2246464646393837363534333231304530303030312220743d2262696e222f3e0a3c2f69736f6d73673e0a";
    let input_bytes = from_hex(input);
    let expected = "1F8B08000000000000008553CB6AC33010BCF72B8C3F20D9D56B25480AF50B02A5D0D01E7A0A7DE3436348DC42FFBEB236A92D23880E3ACC8C7634BBD2AA3D765FC7CFEBAB2C5B7D64EDDB3A873CFBF1BB40C8971358320C0810132A10286630B19E242229D231891858A78C45113322300290C0A24C9DD54121110829666C60B42319E1826BBAB88ED0A744714C46B501A19573CEA70534B0460BE833FAF4460F98937676D0B1898D9B03E7EEC430E73F47E7193023F2011A33052119C7D2F1F6019F813205AAB8C26A3999F8D458A58C3539B226695376FBFEF9B5CF1E7E0FDD85D236555A09550C8F292C4CC6435A085C9C27F44F6A6E757FF87E9F31C42FA0AACBDBCD5D5DED8AA7DD4D79FFB8D9D6DBC4154FD5D8CB36B640DBD4D0149556D277AD5FE72FED3E9A9BE6E88D5FCE92F13AFF4D6A0E30D18F2E7F847A42D85F030000";

    let gzipped = gzip(&input_bytes);

    assert_eq!(expected, to_hex(&gzipped));
}


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
</isomsg>\n";

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
\
                    </isomsg>\n";

    assert_eq!(expected, packed);
}

#[test]
fn test_pack_with_binary_fields() {
    let mut msg = IsoMsg::new_with_mti("0200");
    msg.set_binary(52, "8F8B18FE0FBD5434");

    let packed = msg.pack();
    let expected = "<isomsg>
  <f id=\"0\" v=\"0200\"/>
  <f id=\"52\" v=\"8F8B18FE0FBD5434\" \
                    t=\"bin\"/>
</isomsg>\n";

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
    <f id=\"1\" \
                    v=\"01\"/>
    <f id=\"2\" v=\"02\"/>
    <f id=\"11\" v=\"Eleven\"/>
  \
                    </isomsg>
  <f id=\"64\" v=\"CAFEBABE\"/>
</isomsg>\n";

    println!("expected:\n{}", expected);
    assert_eq!(expected, packed);
}

#[test]
fn test_msg_1508_mac() {
    let expected_mac = util::from_hex("1BD3E21D");

    let pek = util::from_hex("042666B49184CFA368DE9628D0397BC9");
    let data = util::from_hex("044D1F8B08000000000000008592CD4EC3301084EF3C45940BB776779D5DDB52CBA1A23922249E804241113F9508F4F971BD4062B3081FA2E89B893DB3CE6A180F2FE3E3C559D3AC1E9AE17EDD42DB1CD39300DAE50C3BC58050095D16182BECD51F903192672A44C4AC8A88632915CA0A01FABFBEE5EC404EBA2F95A051A2770527DD33960189BF1A953595B2007117634C6D0105D61800914EED854F2CBA508D41E7963C25D6A6DF1574DAAA507B4253FA6CF412D53AE5CCBC82CE825DB9C36A39BBDBF9C1CE3AF8FAF6E3F97C6C6EF66FC7E16E3F9A21AE0E8727330838B1B338907FC2042B4C47DD0EA645661AF40BA0C5CFFD2D7FFDC9ACBED0870D867E0BFDE6923B9792BDAFDBDDF05ADC136B913EAD18BC241F216CF3ED57FE5978412B3CD869EDC19963AB299B544CEA4D1A4C8A6063EDC135A6B97B9AF7F4F60945214B114D04000000");
    let mac = generate_mac(pek.as_slice(), data.as_slice());

    assert_eq!(expected_mac, mac);
}
