# serverless crowbar template

A sample template for bootstraping rustlang [crowbar](https://github.com/ilianaw/rust-crowbar) applications with ⚡ serverless framework ⚡

## 📦 install

Install the [serverless framework](https://serverless.com/framework/).

Then then run the following

```bash
$ serverless install \
  --url https://github.com/softprops/serverless-crowbar \
  --name my-new-app
```

This will download the source of a sample application and unpack it as a new service named
"my-new-app" in a directory called "my-new-app"


## 🧙 how to be a wizard

Assumming you have aws credentials with appropriate deployment permissions configured
in a profile named "prod". Ideally you could impress your friends by creating a project
that is _born_ in production.

```bash
$ serverless install \
  --url https://github.com/softprops/serverless-lando \
  --name my-new-app \
  && cd my-new-app \
  && AWS_PROFILE=prod make dependencies deploy
```

`make dependencies` will make sure dependencies are installed, this only needs run once.
The first time you run `make deploy` it will pull down and compile the base set
of dependencies and your application. Unless the dependencies change afterwards,
this should only happen once, resulting in an our of the box rabbit deployment
cycle.

## 👴 retiring

Experimentation will likely facilitate retiring ideas. Retiring applications should be as easy as creating and deploying them them. This project provides
 a dual to `make deploy` for doing so: `make destroy`

```bash
$ AWS_PROFILE=prod make destroy
```