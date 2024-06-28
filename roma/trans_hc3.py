import string

# Mapping of characters for encoding and decoding
ENCODING_CHARS = (
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    "!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)

def decode_base94(encoded_str):
    base = len(ENCODING_CHARS)
    decoded_value = 0
    for char in encoded_str:
        decoded_value = decoded_value * base + ord(char) - 33
    return decoded_value

def encode_string(encoded_str):
    return ''.join(chr(ENCODING_CHARS.index(char) + 33) for char in encoded_str)
def decode_string(value):
    return ''.join(ENCODING_CHARS[ord(char) - 33] for char in value)

def transpile_icfp_to_haskell(tokens):
    def transpile(tokens, env=None):
        if env is None:
            env = {}
        if not tokens:
            return None, tokens

        token = tokens.pop(0)
        indicator = token[0]
        body = token[1:]

        if indicator == 'T':
            return 'True', tokens
        elif indicator == 'F':
            return 'False', tokens
        elif indicator == 'I':
            return str(decode_base94(body)), tokens
        elif indicator == 'S':
            return f'"{decode_string(body)}"', tokens
        elif indicator == 'U':
            operator = body
            operand, tokens = transpile(tokens, env)
            if operator == '-':
                return f'(-{operand})', tokens
            elif operator == '!':
                return f'(not {operand})', tokens
            elif operator == '#':
                return f'decodeBase94 {operand}', tokens
            elif operator == '$':
                return f'encodeString {operand}', tokens
        elif indicator == 'B':
            operator = body
            x, tokens = transpile(tokens, env)
            y, tokens = transpile(tokens, env)
            if operator == '+':
                return f'({x} + {y})', tokens
            elif operator == '-':
                return f'({x} - {y})', tokens
            elif operator == '*':
                return f'({x} * {y})', tokens
            elif operator == '/':
                return f'({x} `div` {y})', tokens
            elif operator == '%':
                return f'({x} `mod` {y})', tokens
            elif operator == '<':
                return f'({x} < {y})', tokens
            elif operator == '>':
                return f'({x} > {y})', tokens
            elif operator == '=':
                return f'({x} == {y})', tokens
            elif operator == '|':
                return f'({x} || {y})', tokens
            elif operator == '&':
                return f'({x} && {y})', tokens
            elif operator == '.':
                return f'({x} ++ {y})', tokens
            elif operator == 'T':
                return f'take {x} {y}', tokens
            elif operator == 'D':
                return f'drop {x} {y}', tokens
            elif operator == '$':
                return f'({x} {y})', tokens
        elif indicator == '?':
            condition, tokens = transpile(tokens, env)
            if_true, tokens = transpile(tokens, env)
            if_false, tokens = transpile(tokens, env)
            return f'if {condition} then {if_true} else {if_false}', tokens
        elif indicator == 'L':
            variable_number = decode_base94(body)
            lambda_body, tokens = transpile(tokens, env)
            return f'(\\v{variable_number} -> {lambda_body})', tokens
        elif indicator == 'v':
            variable_number = decode_base94(body)
            return f'v{variable_number}', tokens
        return None, tokens

    transpiled_code, _ = transpile(tokens.split())
    return transpiled_code

def main():
    # Example ICFP string
    icfp_string = "B. SF B$ B$ L\" B$ L\" B$ L# B$ v\" B$ v# v# L# B$ v\" B$ v# v# L$ L# ? B= v# I\" v\" B. v\" B$ v$ B- v# I\" Sl I#,"

    # Transpile ICFP to Haskell
    haskell_code = transpile_icfp_to_haskell(icfp_string)

    # Wrap the transpiled code in a Haskell main function
    haskell_code_with_main = f"""
import Data.Char (ord, chr)
import Data.List (unfoldr, foldl')

decodeBase94 :: String -> Int
decodeBase94 = foldl' (\\acc c -> acc * 94 + (ord c - ord '!')) 0

encodeString :: Int -> String
encodeString 0 = "!"
encodeString n = reverse (unfoldr f n)
    where f 0 = Nothing
          f x = Just (chr (ord '!' + x `mod` 94), x `div` 94)

main :: IO ()
main = putStrLn ({haskell_code})
"""
    # Save the generated Haskell code to a file
    with open("output.hs", "w") as f:
        f.write(haskell_code_with_main)

    print("Transpiled Haskell Code saved to output.hs")
    print("You can compile it using: ghc output.hs")

if __name__ == "__main__":
    main()

