# project path
<!-- keep the format -->
## Create for your own project a new project folder in the console(terminal, bash shell),  e.g. in your own home folder,  and open it as a new project inside your program used - in my case MS VSCode / MS VSCodium
<!-- To comply with the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="new_project"
echo ${project_name} 
# cd && mkdir <project_name> folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir ${project_name} && cd $_
```
<!-- keep the format -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& mkdir tests \
&& rustup override set stable \
&& rustup show |sed -n '/active toolchain/,/^$/p' 

```

## Project path see here [![alt text][1]](./project_path.md)
<!-- keep the format -->
> NOTE:
>Add link to files - README.md [![alt text][1]](./README.md) and project_path.md [![alt text][1]](./project_path.md)
><!-- -->
>```bash <!-- markdownlint-disable-line code-block-style -->
> bash -c echo "\n\n<-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email --> \n\n[1]: ./img/link_symbol.svg"  >> ./README.md
>```
<!-- keep the format -->
<!-- keep the format -->
<!-- make folder and download the link sign vai curl -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->
