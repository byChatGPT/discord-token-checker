# Discord Token Checker

[![GitHub](https://img.shields.io/github/license/byChatGPT/discord-token-checker)](https://github.com/byChatGPT/discord-token-checker/blob/main/LICENSE)

This is a simple Discord user token generator and checker in Python.

## Usage

1. Take a Discord user ID from either input() or id.txt file if it exists.
2. Encode it in base64.
3. Generate a token.
4. Fetch the Discord API.
5. Check the response status code.

If the status code is 200, the token is valid, and it will be written to a separate text file "valid.txt". If the response text includes "rate limited", the program will exit. If the token is invalid, the program will repeat the steps above to generate a new token.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
