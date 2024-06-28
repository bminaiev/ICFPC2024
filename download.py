import string
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
            if callable(x):
                return x(y)
            else:
                raise TypeError(f"Expected a callable for application, got {x}")
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
        if variable_number in env:
            return env[variable_number]
        else:
            raise ValueError(f"Variable {variable_number} not found in environment")
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

"""
def main():
    # Evaluate an ICFP string
    icfp_string = "? B= B$ B$ B$ B$ L$ L$ L$ L# v$ I\" I# I$ I% I$ ? B= B$ L$ v$ I+ I+ ? B= BD I$ S4%34 S4 ? B= BT I$ S4%34 S4%3 ? B= B. S4% S34 S4%34 ? U! B& T F ? B& T T ? U! B| F F ? B| F T ? B< U- I$ U- I# ? B> I$ I# ? B= U- I\" B% U- I$ I# ? B= I\" B% I( I$ ? B= U- I\" B/ U- I$ I# ? B= I# B/ I( I$ ? B= I' B* I# I$ ? B= I$ B+ I\" I# ? B= U$ I4%34 S4%34 ? B= U# S4%34 I4%34 ? U! F ? B= U- I$ B- I# I& ? B= I$ B- I& I# ? B= S4%34 S4%34 ? B= F F ? B= I$ I$ ? T B. B. SM%,&k#(%#+}IEj}3%.$}z3/,6%},!.'5!'%y4%34} U$ B+ I# B* I$> I1~s:U@ Sz}4/}#,!)-}0/).43}&/2})4 S)&})3}./4}#/22%#4 S\").!29}q})3}./4}#/22%#4 S\").!29}q})3}./4}#/22%#4 S\").!29}q})3}./4}#/22%#4 S\").!29}q})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}k})3}./4}#/22%#4 S5.!29}_})3}./4}#/22%#4 S5.!29}a})3}./4}#/22%#4 S5.!29}b})3}./4}#/22%#4 S\").!29}i})3}./4}#/22%#4 S\").!29}h})3}./4}#/22%#4 S\").!29}m})3}./4}#/22%#4 S\").!29}m})3}./4}#/22%#4 S\").!29}c})3}./4}#/22%#4 S\").!29}c})3}./4}#/22%#4 S\").!29}r})3}./4}#/22%#4 S\").!29}p})3}./4}#/22%#4 S\").!29}{})3}./4}#/22%#4 S\").!29}{})3}./4}#/22%#4 S\").!29}d})3}./4}#/22%#4 S\").!29}d})3}./4}#/22%#4 S\").!29}l})3}./4}#/22%#4 S\").!29}N})3}./4}#/22%#4 S\").!29}>})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4 S!00,)#!4)/.})3}./4}#/22%#4"
    tokens = icfp_string.split()
    result = evaluate(tokens)
    print("Evaluation result:", result)

    # Encode values to ICFP
    boolean_value = True
    integer_value = 1337
    string_value = "Hello World!"
    
    icfp_boolean = encode_icfp(boolean_value)
    icfp_integer = encode_icfp(integer_value)
    icfp_string_value = encode_icfp(string_value)
    
    print("Encoded boolean:", icfp_boolean)
    print("Encoded integer:", icfp_integer)
    print("Encoded string:", icfp_string_value)
"""
def main():
    task = "3d"
    for i in range(1, 13):
    # for i in [6, 9, 10, 21, 25]:
        try:
            msg = f"get {task}{i}"
            encoded = encode_icfp(msg)
            #print("enc:", encoded)
            response = send(encoded)
            print(response)
            decoded = evaluate(response.split())
            #decoded = response
            with open(f"{task}{i:02d}.in", "w") as out:
                print(decoded, file=out)
            print(i, "ok")
        except:
            print(i, "fail")

if __name__ == "__main__":
    main()

