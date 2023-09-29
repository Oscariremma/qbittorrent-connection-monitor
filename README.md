# qBittorrent Connection Monitor

A simple program to monitor the connection status of qBittorrent and restart it if it is not connected.
You need to have qBittorent running in a docker container that has restart set to always or use linuxserver.io's [qBittorrent image](https://github.com/linuxserver/docker-qbittorrent) that has a built-in mechanism to restart qBittorrent when it shuts down.

The purpose of this program is to restart qBittorrent if it thinks that it's not connected to the internet. This can happen if your VPN connection drops or if your VPN provider is having issues (this happens to me when using Proton VPN). This program will monitor the connection status of qBittorrent and restart it if it's not connected properly.

It is inspired by radixdev's [qbittorent-vpn-switch-rebooter](https://github.com/radixdev/qbittorent-vpn-switch-rebooter) that does basically the same thing but on windows without docker.

## Configuration

The program is configured using environment variables. The following environment variables are available:

| Environment variable  | Description                                                                                    | Default value | Example value           |
|:---------------------:|------------------------------------------------------------------------------------------------|---------------|-------------------------|
|    `QBIT_ENDPOINT`    | The URL to the qBittorrent web interface (required)                                            | none          | http://qbittorent:8080/ |
|    `QBIT_USERNAME`    | The username to use when logging in to the qBittorrent web interface (required)                | none          | admin                   |
|    `QBIT_PASSWORD`    | The password to use when logging in to the qBittorrent web interface (required)                | none          | adminadmin              |
| `QBIT_CHECK_INTERVAL` | The interval in seconds between each check of the connection status                            | 60            | 60                      |
| `QBIT_RETRY_INTERVAL` | The interval in seconds between each retry when something goes wrong                           | 5             | 5                       |
| `QBIT_SHUTDOWN_WAIT`  | The time in seconds to wait for qBittorrent to shut down and start again before checking again | 30            | 30                      |

If you have disabled authentication for the IP range that the program will connect from, set `QBIT_USERNAME` and `QBIT_PASSWORD` to empty strings as the program will not start otherwise.

## Installation

See the included docker-compose.yml file for an example of how to run the program in a docker container.

## Building

To build the program you need to have [rust](https://www.rust-lang.org/) installed. Then you can run `cargo build --release` to build the program. The binary will be located in `target/release/qbittorrent-connection-monitor`.