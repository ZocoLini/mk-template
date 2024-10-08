# MK-Template

**MK-Template** is a command-line application that facilitates the creation and management of 
project, files and directory templates. It allows users to automate the creation of 
repetitive structures with predefined templates.

**Basic Knowledge:**
<ul>
    <li>The application is written in Rust.</li>
    <li>The configuration is saved in <i>$MKT_HOME</i>. If <i>$MKT_HOME</i> is not defined, ~/.mkt will be used instead.</li>
    <li>Templates are saved in <i>$MKT_HOME/templates</i>, known as the templates' directory. The application always
        saves a file with the template's data and, sometimes, the template itself in this directory.</li>
</ul>

Right now, the application is in development and supports 3 types of templates:
<ul>
    <li><strong>Directories:</strong> You can save an entire directory as a template. The entire directory will be copied into the 
        template's directory if you use the <i>-as-dir</i> flag. Otherwise, it will be converted into a TXML template 
        and saved in that format. <i>Note:</i> If the dir contains some binary content, the template will always be saved 
        the same way as if the <i>-as-dir</i> flag was used.</li>
    <li><strong>Git:</strong> A .git directory or link to be cloned. The application will use <i>git clone</i> to 
        the path you provide. If the path becomes unavailable, <i>git clone</i> will fail.</li>
    <li><strong>TXML:</strong> An XML file that defines the template structure. The application will read the XML file and, if it is valid, 
    save it in the template's directory. This type of template won't include binaries, whereas a Dir Template would.</li>
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
   
2. Compile and install the binary (Need cargo installed):
<ul>
   <li>
        <strong>Linux:</strong> You have a installer.sh file that you can run. The binary will be copied to
        <i>~/-local/bin</i>

```bash
$ chmod +x installer.sh;
$ ./installer.sh;
```
   </li>

   <li>
        <strong>Windows:</strong> You have a installer.bat file that you can run. The binary will be copied to
        <i>%USERPROFILE%\AppData\Local\Microsoft\WindowsApps</i>

```cmd
> installer.bat
```
   </li>

   <li>
        <strong>MacOS:</strong> You have to compile and install the package manually.
   </li>
</ul>

## Commands

### Basic syntax:

```bash
$ mkt [COMMAND] [OPTIONS]
```

### Available commands:
**mkt** [**add** **-p** \<Path to the template you want to add> [**-n** \<Custom name for the template>] \[**-as-dir**]],<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[**list** [**-d**]],<br>
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

TXML is a simple XML format that defines the structure of a template. It is used to create templates that can be
spawned using the **MK-Template** application. The TXML schema is available at [lebastudios.org](https://lebastudios.org/xml-schemas/txml_schema.xsd).

Visit the [TXML Documentation](./TXML.md) for more information on how to use TXML.

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

