
# Resume as Code

Forked from: https://github.com/kykosic/resume-as-code

View my resume: [tneubauer.xyz/resume](https://tneubauer.xyz/resume)


After years of rewriting resumes in Microsoft Word, Adobe Photoshop, and LaTeX, I decided to start maintaining my resume as a static web page with the content stored in a simple YAML file.


The [resume](resume.yaml) file makes it easy to update resume information without messing up the formatting. Additionally, web apps separate content from styling, so many different layouts/designs can be applied to the same resume by just using a different [styles.css](wasm-app/styles.css). Storing a resume as code also benefits from Git versioning, making it possible go back in time to previous views.


### Contents
* [Overview](#overview)
* [Build Instructions](#building)
* [Development Setup](#dev-setup)
* [Docker Development](#docker-dev)


<a name="overview"></a>
## Overview

* Resume content is kept in [resume.yaml](resume.yaml).
* The resume content compiled into a rust-based web assembly app to provide some interactivity using [Yew](https://yew.rs/docs/en/intro/).
* Content from [resume.yaml](resume.yaml) is embedded into the webapp during compilation so the data will be there later when run
* A PDF resume is rendered using headless Chrome.
* The static web app is uploaded to web server for hosting

In general, the only piece that needs to be updated is [resume.yaml](resume.yaml) for content. Occasionally [index.html](index.html) is updated to modify the layout.


<a name="building"></a>
## Build Instructions

Instructions for building the components of the resume. 

1. Compiles the rust code into a webassembly app
```
trunk build
```

2. Builds and runs a local development server with hot reload
```
trunk serve
```

3. Render a PDF from the web assembly app.
```
./generate_pdf.sh
```

4. Builds and publishes to local web server
```
./deploy.sh
```


<a name="dev-setup"></a>
## Development Setup

The following tools are used for end-to-end building of this project:
* [cargo](#cargo)
* [wasm-pack](#trunk)
* [chrome](#chrome)

A brief explanation of how each tool is used:

<a name="cargo"></a>
#### [cargo](https://github.com/rust-lang/cargo)
Rust's Cargo tool chain is used to compile the Rust/wasm components of this project. Can be installed following [Install Rust](https://www.rust-lang.org/tools/install).

<a name="trunk"></a>
#### [trunk](https://github.com/thedodd/trunk)
This tool is used to build Rust-based web assembly projects and compile to JavaScript-compatible targets. Can be installed along via cargo:
```
cargo install trunk
```

<a name="chrome"></a>
#### [chrome](https://www.google.com/chrome/)
Headless Chrome is used to render the static page into a PDF document that is exportable from the hosted web app. This is how the PDF resume is generated.


<a name="docker-dev"></a>
## Docker Development

Included is a [Dockerfile](Dockerfile) containing all of the [above](#dev-setup) tools installed. To develop and run the [build processes](#building) in a Docker container:

```
docker build -t resume .
docker run --rm -it -v $(pwd):/home/root resume
```
This will launch the Docker container with an interactive bash shell and this repo mounted to the container home directory.
