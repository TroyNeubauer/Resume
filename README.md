# Resume

My resume.


### Local development

The app is compiled using `trunk`. Local development can be via running:
1. 
```
# Enters a nix shell (pulls in dependencies)
nix develop
```
2. 
```
trunk serve
```

Trunk will compile the rust code to a wasm executable and setup a web server that will reload when any files are changed

### Deploying
Once in a nix shell run `deploy` or if outside run `nix develop --command deploy`. This will:
1. Compile the rust code to a wasm executable
2. Create wasm js wrapper
3. Copy `index.html`, `style.css`, etc into the directory that will become the webserver
4. Run `generate_pdf`, which runs chrome headlessly on a temporaly server
5. Copies these files to `tneubauer.xyz`

