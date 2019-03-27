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

I currently use this on one of my home servers and run it via cron every day.

## License

Distributed under the MIT license. See `LICENSE` for more information.

## Contributing

1.  Fork it (<https://github.com/kdar/smitemotd/fork>)
2.  Create your feature branch (`git checkout -b feature/fooBar`)
3.  Commit your changes (`git commit -am 'Add some fooBar'`)
4.  Push to the branch (`git push origin feature/fooBar`)
5.  Create a new Pull Request
