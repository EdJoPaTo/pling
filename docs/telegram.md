Send a Telegram message

Documentation
- <https://core.telegram.org/bots/api#sendmessage>

# Usage TLDR
- Create `BOT_TOKEN` via [@BotFather in Telegram](https://telegram.me/BotFather)
- Add the created bot to the chat/channel or write the bot a first message
- Somehow obtain the Chat/User ID or Chat/Channel Username (TODO) that should receive the notification

# Environment Variables
- `TELEGRAM_BOT_TOKEN` (required)
- `TELEGRAM_TARGET_CHAT` (required)
- `TELEGRAM_DISABLE_WEB_PAGE_PREVIEW` (optional)
- `TELEGRAM_DISABLE_NOTIFICATION` (optional)
