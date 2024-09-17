# MK-Template

**MK-Template** is a command-line application that facilitates the creation and management of 
project, files and directory templates. It allows users to automate the creation of 
repetitive structures with predefined templates.

**Basic Knowledge:**
<ul>
    <li>The application is written in Rust.</li>
    <li>The configuration is saved in <i>$MKT_HOME$</i>. If <i>MKT_HOME</i> is not defined, ~/.mkt will be used instead.</li>
    <li>Templates are saved in <i>$MKT_HOME$/templates</i>, known as the templates' directory. The application always
        saves a file with the template's data and, sometimes, the template itself in this directory.</li>
</ul>

Right now, the application is in development and supports 3 types of templates:
<ul>
    <li><strong>Directories:</strong> You can save an entire directory as a template. The entire directory will be copied into the 
        template's directory if you use the <i>-as-dir</i> flag. Otherwise, it will be converted into a TXML template 
        and saved in that format.</li>
    <li><strong>Git:</strong> A .git directory or link to be cloned. The application will use <i>git clone</i> to 
        the path you provide. If the path becomes unavailable, <i>git clone</i> will fail.</li>
    <li><strong>TXML:</strong> A XML file that defines the template structure. The application will read the XML file and, if it is valid,
        save it in the template's directory.</li>
</ul>

## Table of Contents

- [Installation](#installation)
- [Commands](#commands)
- [Using TXML](#using-txml)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. Clone the repository and navigate into it.:

   ```bash
    $ git clone https://github.com/ZocoLini/mk-template.git;
    $ cd mk-template;
   ```
   
2. Compile and install the package (Linux) (Need cargo installed):

   ```bash
    $ chmod +x installer.sh;
    $ ./installer.sh;
   ```
   
    2.1. If you are using Windows or MacOS, you have to compile and install the package manually.

## Commands

### Basic syntax:

```bash
$ mkt [COMMAND] [OPTIONS]
```

### Available commands:
**mkt** [**add** **-p** \<Path to the template you want to add> [**-n** \<Custom name for the template>] [**-as-dir**]],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**list**],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**rm** **-n** \<Name of the template you want to remove>]],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**spawn** **-n** \<Name of the template you want to spawn> [**-o** \<Define an output name>]],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**help**],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**version**]<br>

### Examples

**Add a template:**

```bash
$ mkt add -p default-templates/txml.xml # Add a TXML template as txml
$ mkt add -p default-templates/txml.xml -n template # Add a TXML template as template
$ mkt add -p crates/ # Add a directory as crates
$ mkt add -p crates/ -n my-crates # Add a directory as my-crates
$ mkt add -p https://github.com/ZocoLini/mk-template.git # Add a git repository as mk-template
$ mkt add -p https://github.com/ZocoLini/mk-template.git -n mkt_repo # Add a git repository as mkt_repo
```

**Spawn a template:**

```bash
$ mkt spawn -n txml # Spawn the txml template
$ mkt spawn -n txml -o my-txml # Spawn the txml template as my-txml
```

**Remove a template:**

```bash
$ mkt rm -n txml # Remove the txml template
$ mkt rm -n my-crates # Remove the my-crates template
```

## Using TXML

### Basic structure:

```xml
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
</Root>
```
### XSD Schema

The schema is available at [lebastudios.org](https://lebastudios.org/xml-schemas/txml_schema.xsd)

### Elements
<ul>
    <li><strong>Root: </strong>Root defines the actual directory where you want to 'spawn' the 
        template. The root element doesn't have any special attributes. Inside the root element, you can define
        the following elements:
        <ul>
            <li>Directory</li>
            <li>File</li>
        </ul>
    </li>
    <li><strong>Directory: </strong>Directory defines a directory that will be created inside the element 
        it is defined. It can contain the same elements as the Root can. The directory element has the following attributes:
        <ul>
            <li><strong>name: </strong>Defines the name of the directory.</li>
            <li><strong>out_command: </strong>Commands that should be executed <strong>after</strong>
                the directory is created. The commands are separated by a semicolon, executed in the
                order they are defined and <strong>outside</strong> the created directory.</li>
            <li><strong>in_command: </strong>Commands that should be executed <strong>before</strong>
                the directory is created. The commands are separated by a semicolon, executed in the
                order they are defined and <strong>inside</strong> the created directory.</li>
        </ul>
    </li>
    <li><strong>File: </strong>File defines a file that will be created. The content od the file
        is the same as the text inside his tag. The file element has the following attributes:
        <ul>
            <li><strong>name: </strong>Defines the name of the file.</li>
            <li><strong>extension: </strong>Defines the extension of the file</li>
            <li><strong>command: </strong>Commands that should be executed <strong>after</strong>
                the file is created. The commands are separated by a semicolon and executed in the
                order they are defined.</li>
        </ul>
    </li>
</ul>

### Examples

**One file with content (-o will rename the file preserving the extension)**

```xml
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
   <File name="Hola" extension="rs">
      fn main()
      {
          println!("Hello, world!");
      }
   </File>
</Root>
```

**One directory with content (-o will rename the directory rust-project)**

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
   <Directory name="rust-project" in_command="git init">
      <Directory name="crates"/>
      
      <File name="Cargo" extension="toml">
         [workspace]
         resolver = "2"
         members = []
      </File>
      
      <File name=".gitignore"/>
      <File name="rustfmt" extension="toml"/>
   </Directory>
</Root>
```

**Multiple files and directories at Root level (-o won't work because it's impossible to determine 
   what element should be renamed)**
    
```xml 
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
   <Directory name="rust-project" in_command="git init">
      <Directory name="crates"/>
      
      <File name="Cargo" extension="toml">
         [workspace]
         resolver = "2"
         members = []
      </File>
      
      <File name=".gitignore"/>
      <File name="rustfmt" extension="toml"/>
   </Directory>
   
   <Directory name="c-project" in_command="git init">
      <Directory name="src"/>
      
      <File name="main" extension="c">
         #include &lt;stdio.h>
         int main()
         {
             printf("Hello, world!\n");
             return 0;
         }
      </File>
      
      <File name="Makefile">
         all:
         gcc -o main src/main.c
      </File>
      
   </Directory>
   
   <File name="README" extension="md">
      # Welcome to the project
      This is a project created with MK-Template
   </File>
</Root>
```

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -m 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

Please make sure your code adheres to the following guidelines:
- Write clear and concise commit messages.
- Ensure that your code follows the project's style guidelines.
- Add comments where necessary for clarity.
- Test your changes to ensure they don't break existing functionality.

If you find a bug or have a feature request, feel free to open an issue.

Thank you for your contributions!

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) 
file for details.

