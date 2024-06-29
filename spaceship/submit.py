import string
import sys
import requests

def send(message):
# Define the authorization header
    url = "https://boundvariable.space/communicate"
    headers = {
        'Authorization': 'Bearer b44aa9e1-110b-48db-82d5-82159ec5fe47',  # Replace with your actual authorization token
        'Content-Type': 'text/plain'
    }

# Send the HTTP POST request
    response = requests.post(url, headers=headers, data=message)

# Print the response from the server
    if response.status_code != 200:
        print(response)
        print(response.text)
        assert(False)
    return response.text

# Mapping of characters for encoding and decoding
ENCODING_CHARS = (
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)

def decode_base94(encoded_str):
    base = len(ENCODING_CHARS)
    decoded_value = 0
    for char in encoded_str:
        decoded_value = decoded_value * base + ENCODING_CHARS.index(char)
    return decoded_value

def encode_base94(value):
    if value == 0:
        return ENCODING_CHARS[0]
    base = len(ENCODING_CHARS)
    encoded_str = ""
    while value > 0:
        encoded_str = ENCODING_CHARS[value % base] + encoded_str
        value //= base
    return encoded_str

def encode_string(encoded_str):
    """
    print(encoded_str)
    for char in encoded_str:
        print(char)
        print(ENCODING_CHARS.index(char))
"""
    return ''.join(chr(ENCODING_CHARS.index(char) + 33) for char in encoded_str)

def decode_string(value):
    return ''.join(ENCODING_CHARS[ord(char) - 33] for char in value)

def evaluate(tokens, env=None):
    if env is None:
        env = {}
    if not tokens:
        return None
    
    token = tokens.pop(0)
    indicator = token[0]
    body = token[1:]
    
    if indicator == 'T':
        return True
    elif indicator == 'F':
        return False
    elif indicator == 'I':
        return decode_base94(body)
    elif indicator == 'S':
        return decode_string(body)
    elif indicator == 'U':
        operator = body
        operand = evaluate(tokens, env)
        if operator == '-':
            return -operand
        elif operator == '!':
            return not operand
        elif operator == '#':
            return decode_base94(operand)
        elif operator == '$':
            return encode_string(operand)
    elif indicator == 'B':
        operator = body
        x = evaluate(tokens, env)
        y = evaluate(tokens, env)
        if operator == '+':
            return x + y
        elif operator == '-':
            return x - y
        elif operator == '*':
            return x * y
        elif operator == '/':
            return x // y
        elif operator == '%':
            return x % y
        elif operator == '<':
            return x < y
        elif operator == '>':
            return x > y
        elif operator == '=':
            return x == y
        elif operator == '|':
            return x or y
        elif operator == '&':
            return x and y
        elif operator == '.':
            return x + y
        elif operator == 'T':
            return y[:x]
        elif operator == 'D':
            return y[x:]
        elif operator == '$':
            return x(y)
    elif indicator == '?':
        condition = evaluate(tokens, env)
        if_true = evaluate(tokens, env)
        if_false = evaluate(tokens, env)
        return if_true if condition else if_false
    elif indicator == 'L':
        variable_number = decode_base94(body)
        lambda_body = tokens.copy()
        return lambda arg: evaluate(lambda_body.copy(), {**env, variable_number: arg})
    elif indicator == 'v':
        variable_number = decode_base94(body)
        return env.get(variable_number)
    return None

def encode_boolean(value):
    return 'T' if value else 'F'

def encode_integer(value):
    return 'I' + encode_base94(value)

def encode_icfp(value):
    if isinstance(value, bool):
        return encode_boolean(value)
    elif isinstance(value, int):
        return encode_integer(value)
    elif isinstance(value, str):
        return 'S' + encode_string(value)
    return None

def main():
    fname = sys.argv[1]
    msg = "solve " + fname[:-4].replace("p0", "p") + " " + open(fname).read().strip()
    # print("msg:", msg)
    encoded = encode_icfp(msg)
    # print("enc:", encoded)
    response = send(encoded)
    # print(response)
    decoded = evaluate(response.split())
    print(decoded)

if __name__ == "__main__":
    main()

