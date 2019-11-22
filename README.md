# quickproj

Flexible project creation for minimalists

- [Quick start](#quick-start)
- [FAQ](#faq)
- [Templates structure](#templates-structure)
- [License](#license)

## Features

- Template management with local and git repositories
- Initializing a new project from the existing templates
- Validating and parsing JSON configs
- Overriding config variables via user's input (with validation)

## Example of usage
<img src="https://github.com/Relrin/quickproj/blob/master/screenshots/demo.gif?raw=true">

## Quick start

1. Download executable/binary file in according to the used operation system from the [releases page](https://github.com/Relrin/quickproj/releases).

2. Link executable/binary file to operation system, so you could invoke `quickproj` everywhere:

    - Linux / Mac OS
  
        Move the binary file to the `/usr/local/bin` directory and restart the terminal
        ```
        mv ~/Downloads/quickproj /usr/local/bin
        ```
    
    - Windows
    
        1. Right click on the Windows Logo and select the `System` menu item.
        2. Click on the `Advanced System Settings` button.
        3. Click on the `Environment Variables` button.
        4. Select your `PATH` variable and click in the `Edit` button.
        5. Click on the `New` button.
        6. Add the file path to the directory with the `quickproj` executable.
        7. Click on the `OK` button a couple of times for applying changes.

3. After it you can call the `quickproj` command from any folder. For more information about acceptable arguments and options for each command, call any desired command with the `--help` option.

## F.A.Q.
Q: What is the purpose of this tool?  
A: I made it for easier developing new projects from the scratch. By defining the used templates you have more granular control over your needs and how to prepare the new project.

Q: Is it possible to run commands in the multi-threaded mode?   
A: No, it isn't possible to run the command in multi-threaded at the moment. Although the existing code base can be easily changed for those things, but for prevent "killing" your hard drive with massive I/O operations, I've decided to left it simple as much as possible.

## Templates structure
Each installed template has to have configuration file (named as the `config.json`) in the root template folder and files that needs to copy or generate for the new project.

### Configuration parameters
As the example we will take and modify one of the existing templates for the `quickproj` ([original](https://github.com/Relrin/quickproj-templates/blob/master/terraform-sage/config.json) file) application:
```json
  
{
  "files":{
    "sources": [
      { "from": "sources", "to": "." }
    ],
    "generated": [
      "configs/{{ terraform_sage_environment }}/variables.tfvars"
    ],
    "directories": [
      "configs/{{ terraform_sage_environment }}"
    ],
    "templates": {
      "variables.tfvars": "templates/variables.tfvars"
    }
  },
  "variables": {
    "terraform_sage_environment": [
        "dev", 
        "production", 
        "staging"
    ],
  },
  "scripts": {
    "after_init": [
       "ls -al"
    ]
  },
  "storage": {
    "variables": {
      "service_name": "service"
    }
  }
}
```
The configuration file must be saved in the root directory of the template with the `config.json` file name. Otherwise, the `quickproj` application will ignore the user's template.

### Files section
The main section of the configuration file of the template. It stores the information about what files and folders need to create, copy or generate.
- `sources`  
    
    This section stores paths to the used folders for copying files. Each record must have the following keys:  
     - The `from` key means the relative path to the template folder from which files need to copy to the target directory.
     - The `to` key means the relative path in the target folder in which files have to be copied from the source directory. 

- `generated`  

   Defines relative paths to templates needs to generated in the target folder. Each path can be specified as the static (=hardcoded) or dynamic (=with the usage of config variables) paths to target files. For using the templates in this section, the user must to specify the desired template name in the end of the path (the same key value as it was defined in `templates` section).  
   In the example above, we have specified a dynamic path, so that it will generate three files with the following paths based on the `variables.tfvars` template:
        
   - `configs/dev/variables.tfvars`
   - `configs/production/variables.tfvars`
   - `configs/staging/variables.tfvars`

- `directories`

   Defines the directories that needs to be generated before copying files. In the example, the `"configs/{{ terraform_sage_environment }}"` values means that we're going to create three different directories. The values for it will be extracted from the `terraform_sage_environment` variable, specified in the `variables` section.  
   In our example, the application will create three folders, based on the dynamic paths and variables from the `variables` section:
   
   - `configs/dev/`
   - `configs/production/`
   - `configs/staging/` 
  
- `templates`  

   Defines the list of records that have to be used for the project generation process, where: 
   
    - The key represented as the file name of the template, available to use in the `generated` section
    - The value represented as the relative path in the source folder to the template that must be used

### Variables section
Optional section which stores all variables that can be used during the project generation and can be overridden by the user if was specified the `--override-all` or the `--override` options in CLI.

Each variable, specified in this section has to met the following requirements:
- The key can be represented only as the `string` type.
- The value can be represented as the `string` or as the `array of strings` types. 

Key / values pairs that won't met the requirements will be ignored and not used during the project generation.

During the overriding stage (when the CLI will ask you to specify the value to override), you can specify any correct values for the certain types:
 
- Any non-empty string for the `string` type.
- Non-empty string, where each value separated by the `,` for the `array of strings` type.

Hitting the `Enter` key or setting the empty string for the certain key will lead to using the default value, specified in the configuration.

### Scripts section
Optional section that describes a list of commands/scripts that could be executed during the template installation process.

- `after_init`

   Required to be defined as the array of strings, where each string represented as the certain command needs to be executed after the template installation process.
   For example from the example on top, the application will execute the `ls -al` command to output the list of files in the current directory.  

### Storage section
Optional section which is using as the storage for the template data, without giving an access to endusers to override them.

- `variables`

   Optional. Defines template variables as the key-value pairs with the a reference support for accessing the main variables section.

   Each key-value pair has to met the following requirements:
    - The key can be represented only as the `string` type.
    - The value can be represented as the `string` or as the `array of strings` types.
    - Any reference to something in the `variables` section must be specified as the string with the `Vars.` prefix. For example: `{{ Vars.my_variable }}`.

## License

The quickproj is published under BSD license. For more details read the [LICENSE](https://github.com/Relrin/quickproj/blob/master/LICENSE) file.
