# Siren
Your friendly neighborhood monitoring CLI tool.

Just write your own `Sirenfile.json` like this:
```json
{
  "switch_cwd": true,
  "tasks": [
    {
      "name": "foo",
      "description": "foo description",
      "command": "echo foo"
    },
    {
      "name": "bar",
      "description": "bar description",
      "command": "echo bar"
    }
  ]
}
```

Siren takes your tasks and executes them, alerting you if one of your checks fail. The checks are standard Nagios', so you can write your own checks just issuing an exit code different than `0` if something fails. When a task has its exit code equal to `0`, that check is seen as successful.

## Installation
```sh
cargo install siren
```

## Run
Once you placed your own Sirenfile into the current directory, you can run Siren:
```sh
$ siren
```

You can also run Siren with a different Sirenfile than the default one:

```sh
$ siren --file my/personal/checks/Sirenfile.json
```

## JSON output
Users can decide to have a recap of all tasks in JSON format instead of the plain text/console one.

This can be done using the `--json-output` flag:

```sh
$ siren --json-output
```

This is meant for further integrations like complex systems where Siren is only a piece of the puzzle.

## Configuration options
Here the fields you can configure in your `Sirenfile`:
- `switch_cwd`: Specifies if you want the current working directory to be changed to the one containing the `Sirenfile`. Useful if you want to write commands relative to that relative path.
- `tasks`: An array of tasks, purely. Every task has a field containing its name, a description, and a command field that gets executed as a child process.
