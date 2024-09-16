pub mod objects;
mod commands;

#[cfg(test)]
mod tests {
    use crate::objects::TxmlStructure;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn process_txml_test_from_file() {
        if let Err(e) = TxmlStructure::from_file(
            &PathBuf::from_str(
                "/home/borja/projects/mk-template/tests/resources/template_example_1.xml",
            )
            .expect("Should exist"),
        ) {
            panic!("Error: {:?}", e);
        }
    }

    #[test]
    fn process_txml_test_from_str() {
        if let Err(e) = TxmlStructure::from_str(
            r#"
            <?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
    <Directory name="crates" in_command="git init   ;      mkdir pepe">
        <File name="Hola" extension="rs">
            fn main()
            {
                println!("Hola, mundo!");
            }
        </File>
        <Directory name="crate">
            <File name="sin_titulo" extension="txt">
                Hola me llamo Juan
                Pepe
            </File>
        </Directory>
    </Directory>

    <File name="Cargo" extension="toml">
        [workspace]
        resolver = "2"
        members = []
    </File>
    <File name="rustfmt" extension="toml"/>
    <File name=".gitignore"/>
</Root>
            "#,
        ) {
            panic!("Error: {:?}", e);
        }
    }
}
