# Discord Bot Project

![image](https://github.com/Xbz-24/Serenity-MusicBot/assets/68678258/9c745e87-e0c9-426c-99cf-469d65056ace)

This Discord bot is built using the Serenity library in Rust, designed to provide a robust framework for building Discord bots. It features commands for joining voice channels, playing audio from YouTube URLs, managing queues, and more.

## Features

- Join and leave voice channels
- Play audio from YouTube via URL
- Queue management for audio playback
- Volume control and track skipping
- Deafen, mute, undeafen, and unmute capabilities within voice channels

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust Programming Language installed on your machine.
- A Discord Bot Token. Follow [Discord's official guide](https://discord.com/developers/docs/intro) to set up a bot account and obtain your token.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Installation

1. Clone the repository:

```bash
git clone https://yourrepositorylink.com
cd discord-bot-project
```

2. Create a `.env` file in the root directory and add your Discord Bot Token:

```plaintext
DISCORD_TOKEN=your_discord_bot_token_here
```

3. Build and run the project:

```bash
cargo build
cargo run
```

### Usage

After running the bot, you can use the following commands in your Discord server:

- `~join` - The bot joins your current voice channel.
- `~leave` - The bot leaves the voice channel.
- `~play [URL]` - Play audio from the provided YouTube URL.
- `~queue [URL]` - Add a YouTube URL to the queue.
- `~skip` - Skip the current track.
- `~stop` - Stop playing and clear the queue.


## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Renato German - [@xbz24](xbz@waifu.club) - xbz@waifu.club

Project Link: [https://github.com/Xbz-24/Serenity-MusicBot/](https://github.com/Xbz-24/Serenity-MusicBot/)

## Acknowledgments

- [Serenity](https://github.com/serenity-rs/serenity) for the comprehensive Rust library for Discord bots.
- [YouTube-DL](https://github.com/ytdl-org/youtube-dl) for providing the means to stream audio from YouTube.
