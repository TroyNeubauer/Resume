# Resume

My resume.


### Local development

The app is compiled using `trunk`. Local development can be via running:

Opening a nix shell (pulls in required dependencies):
```
nix develop
```

Using trunk to compile and interactively serve the webapp:
```
trunk serve
```

Trunk serve will recompile the webapp if any files are modified.

### Deploying
Once in a nix shell run `deploy` or if outside run `nix develop --command deploy`. This will:
1. Compile the rust code to a wasm executable
2. Create wasm js wrapper
3. Copy `index.html`, `style.css`, etc into the directory that will become the webserver
4. Run `generate_pdf`, which runs chrome headlessly on a temporaly server
5. Copies these files to `tneubauer.xyz`

