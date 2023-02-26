# Discord User Token Generator and Checker

This is a simple Python program that generates Discord user tokens and checks whether they are valid by making an API request to Discord.

## How to Use

1. Clone or download the repository.
2. Install the required libraries: `pip install requests`
3. Open a terminal or command prompt and navigate to the directory where the program is located.
4. Run the program using the command: `python discord_token_checker.py`
5. If you have a file called `id.txt` in the program directory, the program will read the Discord user ID from that file. Otherwise, it will prompt you to enter the ID.
6. The program will generate tokens and check them until a valid token is found. If a valid token is found, it will be saved to a separate text file called `valid.txt`.
7. If the program encounters a "rate limited" response from Discord, it will exit.

## How it Works

The program works by first encoding the user ID in base64 and then generating a token string in the format `{base64 encoded discord user id}.{one random uppercase letter}{5 random numbers and upper/lowercase letters}.{27 random numbers and lower/uppercase letters}`.

The program then makes an API request to Discord using the generated token, and checks the response status code. If the status code is 200, the token is valid and is saved to a separate text file called `valid.txt`. If the response text includes "rate limited", the program exits. If the token is invalid, the program generates a new token and repeats the process.

## Requirements

- Python 3.6+
- requests library

## Limitations

This program should only be used for educational purposes and should not be used to access or manipulate other users' accounts without their consent. The program is subject to Discord's rate limiting and may be blocked if used excessively.
