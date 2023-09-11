# Heavy Metal Notifier

Do you often miss out on the latest heavy metal album releases from your favorite bands due to a busy schedule? If so, we have the perfect solution for you! Our project will email you every time there are new heavy metal album releases.

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

Copy the [config.json](https://github.com/reaper47/heavy-metal-notifier/blob/main/deploy/config.example.json) file next to the 
executable and [edit the variables](#configuration-file). Ensure to rename the `config.example.json` file to
`config.json`.

```bash
cp ./deploy/config.example.json ./bin/config.json
```

Start the server.

```bash
  ./bin/metal serve
```

## Configuration File

The [configuration file](https://github.com/reaper47/heavy-metal-notifier/blob/main/deploy/config.example.json) sets important 
variables for the application. Let's go over each of them.

Then, open *config.json* to edit the following variables:
- **email.from**: The administrator's email address
- **email.sendGridAPIKey**: Your [SendGrid](https://sendgrid.com/) API key. The free tier should be sufficient for your needs.
- **email.maxNumberUsers**: The maximum number of users you can have. The number depends on your SendGrid plan. The free plan can send a maximum of 100 emails daily.
- **port**: The port the app will be served through.
- **url**: The website the app is served on. This URL is used in the emails.

## Deployment

The project can currently be self-hosted using Docker or a service.

### Docker

A Docker image called `reaper99/heavy-metal` is produced on every release.

#### Using Docker

You first have to fetch it.
```bash
docker pull reaper99/heavy-metal:latest
```

Then, run the image. You must pass your `config.json` file to the container.
```bash
docker run -v path/to/config.json:/app/config.json -p [host port]:[port specified in config.json] -d reaper99/heavy-metal:latest
```

#### Using Docker Compose

You can use Docker Compose to run the container. First, you need to modify the ports and the path to your local 
config.json in the [compose.yaml](https://github.com/reaper47/heavy-metal-notifier/blob/main/deploy/compose.yaml).

Then, start the application.

```bash
docker-compose up -d
```

Access the app through your browser at `http://localhost:[host port]`.
If you are using Windows and you intend to access the app on other devices within your home network, please ensure
to `Allow the connection` of the `Docker Desktop Backend` inbound Windows Defender Firewall rule.

### Service

First download and extract the [latest release](https://github.com/reaper47/heavy-metal-notifier/releases).

Then, copy the [config.json](https://github.com/reaper47/heavy-metal-notifier/blob/main/deploy/config.example.json) file next 
to the executable and [edit the variables](#configuration-file).

Next, copy the [service example file](https://github.com/reaper47/heavy-metal-notifier/blob/main/deploy/metal-releases.service) 
and edit the variables to run the app automatically on boot.

```bash
sudo cp ./deploy/metal-releases.service /etc/systemd/system/ 
```

Finally, start the service on boot.

```bash
sudo systemctl start heavy-metal-notifier.service
sudo systemctl enable heavy-metal-notifier.service
```

## Contributing

Contributions are always welcome! Please open a pull request or email us at metal.releases.666@gmail.com.

## Sponsors

I am grateful for any support that helps me continue to develop this project. Your sponsorship will help me pay for 
the SendGrid paid plan to increase the limit of users. The free plan currently used allows sending a maximum of 100 
emails per day. This means the application can have a maximum of 100 users because one email per user is sent whenever 
there are new heavy metal album releases.

You can sponsor me on [GitHub Sponsors](https://github.com/sponsors/reaper47) or 
[Buy Me a Coffee](https://www.buymeacoffee.com/macpoule).

Your support is greatly appreciated!

## License

[MIT](https://choosealicense.com/licenses/mit/)
