# The Anarchist's Workshop

This is a a bunch of code put together in order to run things at workshops.

As in, places where people work. Workshops.

The end goal is to have some Free Open-Source Software which can be installed
relatively easily on a computer, and be helpful from the start when running.

## Grabbing the code

To use as a dependency, add the crate from source at github (for  now):
```toml
[dependencies]
the-archist-workshop = { git = "https://github.com/saibatizoku/the-anarchist-workshop.git" }
```

To clone the git repository from source at github:
```sh
git clone https://github.com/saibatizoku/the-anarchist-workshop.git
cd the-anarchist-workshop
```

### Want to contribute?

To push code to the project, fork the project from github, add a reasonably named branch,
create a pull request. Simplicity and one-thing-at-a-time-edness is greatly appreciated. Play
nice, do your thing.

## Using the code

To start, you can run the anarchist workshop's command-line interface. We call it `taw`.

Since we're in prototype mode, the suggested way to run it is to grab the code from github, and
then you can build it and run it locally:

```
git clone ...
cd ...
cargo run -- --help
```

Then, when the code is updated upstream, you can type something like this:

```
git pull
cargo run -- --help
```

This way, the executable gets built again, incorporating the new code.

### Ideally

Another way to run the command-line interface is to _install_ it on your system.

For this, you can go to the directory where you cloned the code, and type:

`cargo install --path ./the-anarchist-workshop-command-line-interface`

When you install the code this way, an executable is built and put into your `$PATH`, usually
in the `$HOME/.cargo/bin` directory. Might be different for you.

And, the cool thing about this, is that you get to call it from the terminal like this:

```
taw --help
```

which should show you the usage and a list of available commands, options, etc.

## Motivation And Anarchical Theming

The title is inspired by the work of one Christopher M. Schwarz, well-known woodworker,
bookwriter, publisher, etc. By reading his book about building his design for "The Anarchist Workbench".
Yes, from reading his book. It's good, and it's licensed with Creative Commons over at the
[lostartpress.com](https://lostartpress.com), it's worth taking the time to look for it there, or somewhere in the
[blog.lostartpress.com](https://blog.lostartpress.com).
