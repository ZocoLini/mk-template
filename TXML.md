# TXML Documentation

## Table of Contents

- [Basic structure](#basic-structure)
- [XSD Schema](#xsd-schema)
- [Elements](#elements)
- [Examples](#examples)

## Basic structure:

```xml
<?xml version="1.0" encoding="UTF-8" ?>

<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd">
</Root>
```
## XSD Schema

The schema is available at [lebastudios.org](https://lebastudios.org/xml-schemas/txml_schema.xsd)

## Elements
<ul>
    <li><strong>Root: </strong>Root defines the actual directory where you want to 'spawn' the 
        template. Inside the root element, you can define the following elements:
        <ul>
            <li>Directory</li>
            <li>File</li>
        </ul>
        This element also has an attribute called <strong>renamable</strong> that defines if the <i>-o</i> flag
        would be used or ignored. If you are using <i>Variables</i> in the template and the main File or Directory uses 
        the variable, you must set this attribute to <i>false</i> to avoid the variable being ignored.
    </li>
    <li>
        <strong>Variable: </strong>Variable defines a variable that can be used in the name of the file or directory.
         It can also be used in the content of the file. The variable element has the following attributes:
        <ul>
            <li><strong>name: </strong>Defines the name of the variable. To use this variable you should use
                the following syntax: <i>${variable_name}</i></li>
            <li><strong>value: </strong>Defines the value of the variable. All the occurrences of the variable
                will be replaced by this value. This attribute is optional and, if not defined, the variable
                will be asked when the template is spawned.</li>
        </ul>
    </li>
    <li>
        <strong>Metadata: </strong> The metadata element has the following attributes:
        <ul>
            <li><strong>author: </strong>Defines the author of the template.</li>
            <li><strong>date: </strong>Defines the date of the template.</li>
            <li><strong>version: </strong>Defines the version of the template.</li>
            <li><strong>description: </strong>Defines the description of the template.</li>
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

## Examples

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

**One directory with content (-o will not rename the directory rust-project)**

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<Root xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:noNamespaceSchemaLocation="https://lebastudios.org/xml-schemas/txml_schema.xsd" renamable="false">
   <Variable name="rust-project" />
   <Directory name="${rust-project}" in_command="git init">
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
   <Variable name="c-project" />
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
   
   <Directory name="${c-project}" in_command="git init">
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
