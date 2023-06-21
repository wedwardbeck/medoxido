## medóxido
a Medication Tracker built on Rust

[![Coverage](https://github.com/wedwardbeck/medoxido/actions/workflows/rust.yml/badge.svg)](https://github.com/wedwardbeck/medoxido/actions/workflows/rust.yml)

## What is Medóxido?
medóxido (med ohk-see-doh), a combination of medication and óxido (Rust in Spanish), is a stand-alone medication tracker with reminders, notes, medication interaction information, and analytical reports to show trends, correlations, and other helpful feedback.  It is intended to be installed locally on most common platforms and uses a local built-in database engine and local file.

### Why build this?
This project began as a proof of concept (POC) to demonstrate a combination of some of the latest development frameworks and solutions built on Rust.  It was also intended to deepen my knowledge of the language and perhaps serve as an example of how to do (or not to do) things in an application.

### What is it built with, and why?
medóxido is built with Rust at it's core, using Axum as the web framework for the API, SurrealDB as the database, Qwik for the Javascript front-end, and Tauri to wrap it all together into an executable.
The rationale behind these tools was to show how effective the combination of these tools could be.  Rust was chosen for its safety and performance and its small impact on the machine's RAM when running.  SurrealDB was chosen since it is built into the application as an embedded database, provides an innovative and incredible query language and features, and is expected to be very fast.  Axum felt like a more understandable framework, and being part of Tokio provides easy use of other potential features from that framework.  Tauri is a Rust-based solution designed as a "polyglot approach" to building desktop applications using virtually any frontend framework in existence, making it much easier to bundle it all up for compilation and distribution.  Qwik is the only non-Rust-based framework.  However, it's one of the newest Javascript frameworks that delivers excellent results with little effort, and it was the choice for the front-end that Tauri would wrap.  Yew is another excellent option, built on Rust, but since I already have worked with Qwik decided to leverage that instead of having one more framework to figure out.

## Notes
Currently crates.io is reporting an error due to a conflict with the beta 9 version name.  This will be resolved once the beta 10 version is released, but the project will still build locally once cloned.
