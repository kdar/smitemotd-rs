smitemotd
=========

Tool to retrieve the latest Smite MOTD (Match of the Day). It can either print this to console or to various other places like notification services or email.

## Features

* Single executable with no external dependencies (other than common dynamically linked libraries).
* Output to console with color (ANSI) support.
* Works on linux and *should* work on OSX and Windows.
* Can notify via email, pushbullet, pushed.co, and slack.

## Developer registration

You must first sign up with HiRez to get your developer credentials. Go here: [https://fs12.formsite.com/HiRez/form48/secure_index.html](https://fs12.formsite.com/HiRez/form48/secure_index.html).

Once accepted, you will get your DevId and AuthKey.

## Usage

```console
USAGE:
    smitemotd [OPTIONS] --auth-key <KEY> --dev-id <ID>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --auth-key <KEY>
            Sets the auth key for the Smite API [env: AUTH_KEY=]

        --dev-id <ID>
            Sets the dev ID for the Smite API [env: DEV_ID=]

        --notify-email <OPTS>...
            Notifies via email

            Options:
              from - The from email address
              recipients - A list of recipients to receive the email,
              subject - Optional subject for email
              username - Optional username for SMTP server
              password - Optional password for SMTP server
              smtp - The smtp host to connect to

            Example:
              'from="hey@gmail.com", recipients = ["yolo@gmail.com"], subject = "Hey", smtp = "smtp.gmail.com"'

            Extra: [env: NOTIFY_EMAIL=]
        --notify-pushbullet <OPTS>...
            Notifies via Pushbullet

            Options:
              token - The token for Pushbullet
              channel_tag - The channel tag to push to

            Example: 'token="1234", channel_tag="smitemotd"'

            Extra: [env: NOTIFY_PUSHBULLET=]
        --notify-pushed <OPTS>...
            Notifies via Pushed.co

            Options:
              key - The app key for Pushed
              secret - The app secret for Pushed

            Example: 'key="1234", secret="5678"'

            Extra: [env: NOTIFY_PUSHED=]
        --notify-slack <OPTS>...
            Notifies via Slack

            Options:
              hook - The url for the slack hook

            Example: 'hook="https://hooks.slack.com/services/..."'

            Extra: [env: NOTIFY_SLACK=]
        --notify-stream <OPTS>...
            Notifies via stdout, stderr, or a file

            Options:
              stdout - Whether to use stdout [true/false]
              stderr - Whether to use stderr [true/false]
              file - The file name to write to
              color: Whether to colorize output (ANSI) [true/false]

            stdout, stderr, and file are mutually exclusive

            Examples:
              'stdout=true'
              'stderr=true, color=true'
              'file="/tmp/output"'

            Extra: [env: NOTIFY_STREAM=]
```

## Motivation

I wanted to be able to know what the daily was without having to login. I run this on a home linux server in cron.

## License

Distributed under the MIT license. See `LICENSE` for more information.

## Contributing

1.  Fork it (<https://github.com/kdar/smitemotd/fork>)
2.  Create your feature branch (`git checkout -b feature/fooBar`)
3.  Commit your changes (`git commit -am 'Add some fooBar'`)
4.  Push to the branch (`git push origin feature/fooBar`)
5.  Create a new Pull Request
