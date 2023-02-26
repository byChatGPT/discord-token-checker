import base64
import random
import requests

def generate_discord_token(user_id):
    # Encode the user ID in base64
    b64_user_id = base64.b64encode(user_id.encode()).decode()

    # Generate a random token
    token = f"{b64_user_id}.{random.choice('ABCDEFGHIJKLMNOPQRSTUVWXYZ')}{''.join(random.choices('ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', k=5))}.{''.join(random.choices('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', k=27))}"

    # Send a request to the Discord API with the token as the authorization header
    headers = {"Content-Type": "application/json", "authorization": token}
    response = requests.get("https://discordapp.com/api/v6/users/@me/library", headers=headers)

    # Check the response status code
    if response.status_code == 200:
        # If the status code is 200, the token is valid, so return it
        return token
    elif "rate limited" in response.text:
        # If the response contains "rate limited", raise an exception
        raise Exception("API rate limit reached. Please try again later.")
    else:
        # If the response does not contain "rate limited", raise an exception with the error message
        raise Exception(f"Token {token} is invalid. Error message: {response.text}")

# Check if id.txt exists and get the user ID from it if it does
try:
    with open("id.txt", "r") as f:
        user_id = f.read().strip()
except FileNotFoundError:
    user_id = input("Enter a Discord user ID: ")

# Generate a token and check if it's valid
token = generate_discord_token(user_id)
while True:
    try:
        # If the token is valid, write it to valid.txt and exit the loop
        with open("valid.txt", "a") as f:
            f.write(f"{token}\n")
        print(f"Token {token} is valid.")
        break
    except Exception as e:
        # If the token is invalid, print the error message and generate a new token
        print(str(e))
        print("Generating a new token...")
        token = generate_discord_token(user_id)