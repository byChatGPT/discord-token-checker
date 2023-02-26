# Discord Token Checker

This is a simple Discord user token generator and checker written in Python. The generator creates a token by encoding a Discord user ID in base64 and appending random characters. The checker sends a request to the Discord API using the generated token and checks if the response status code is 200.

## Features
- Generates a random token using a Discord user ID
- Checks if the generated token is valid by sending a request to the Discord API
- Outputs whether the token is valid or invalid

## Usage
1. Clone this repository using `git clone https://github.com/username/repo.git`
2. Open the terminal and navigate to the project directory
3. Run the generator using `python generator.py`
4. Check the `valid.txt` file for a list of valid tokens

Note: You may need to install the required packages using `pip install -r requirements.txt`.

## Optimization
The original code has been optimized for readability and performance. Changes include:
- Using `requests.Session()` to reuse connections to the Discord API
- Caching the base64 encoded Discord user ID
- Using `f-strings` for string formatting
- Using `try-except` blocks for error handling

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.