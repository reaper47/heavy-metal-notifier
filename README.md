
# Heavy Metal Notifier

Do you often miss out on the latest heavy metal album releases from your favorite bands due to a busy schedule? If so, we have the perfect solution for you! Our project will send you an email every time there are new heavy metal album releases.

The application works by creating a calendar from [Wikipedia heavy metal releases](https://en.wikipedia.org/wiki/2023_in_heavy_metal_music) page that lists all the heavy metal album releases throughout the year. It is consulted every day at 1am local time zone. If there are any new releases, an email containing a list of the releases will be sent to all confirmed users.
## Run Locally

Clone the project.

```bash
  git clone https://github.com/reaper47/heavy-metal-notifier.git
```

Go to the project directory.

```bash
  cd heavy-metal-notifier
```

Build the project.

```bash
  make
```

Start the server.

```bash
  ./bin/metal serve
```


## Deployment

Currently, the project can only be self-hosted.

First download and extract the [latest release](https://github.com/reaper47/heavy-metal-notifier/releases).

Then, open *config.json* to edit the following variables:
- **email.from**: The administrator's email address
- **email.sendGridAPIKey**: Your [SendGrid](https://sendgrid.com/) API key. The free tier should be sufficient for your needs.
- **port**: The port the app will be served through.
- **url**: The website the app is served on. This URL is used in the emails.

Finally, create a service to run the app automatically on boot.

```bash
sudo nano /etc/systemd/system/heavy-metal-notifier.service 
```

Copy the following content to the newly-created file.

```bash
[Unit]
Description=Heavy Metal Releases Service
Wants=network.target

[Service]
ExecStart=/path/to/binary/metal serve

[Install]
WantedBy=multi-user.target
```

Start the service on boot.

```bash
sudo systemctl start heavy-metal-notifier.service
sudo systemctl enable heavy-metal-notifier.service
```
## Contributing

Contributions are always welcome! Please open a pull request or send us an email at metal.releases.666@gmail.com.


## License

[MIT](https://choosealicense.com/licenses/mit/)
