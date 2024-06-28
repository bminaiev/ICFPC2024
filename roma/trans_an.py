import string
from typing import List, Tuple

def base94_to_int(s: str) -> int:
    base94_chars = string.printable[:-6]
    return sum((ord(c) - 33) * (94 ** i) for i, c in enumerate(reversed(s)))

def decode_string(s: str) -> str:
    char_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    return ''.join(char_map[ord(c) - 33] for c in s)

def parse_tokens(tokens: List[str]) -> Tuple[str, List[str]]:
    if not tokens:
        raise ValueError("Empty token list")
    
    token = tokens.pop(0)
    indicator = token[0]
    body = token[1:]

    if indicator == 'T':
        return "True", tokens
    elif indicator == 'F':
        return "False", tokens
    elif indicator == 'I':
        return str(base94_to_int(body)), tokens
    elif indicator == 'S':
        return f'"{decode_string(body)}"', tokens
    elif indicator == 'U':
        op = body
        arg, tokens = parse_tokens(tokens)
        if op == '-':
            return f"(negate {arg})", tokens
        elif op == '!':
            return f"(not {arg})", tokens
        elif op == '#':
            return f"(read {arg})", tokens
        elif op == '$':
            return f"(show {arg})", tokens
        else:
            raise ValueError(f"Unknown unary operator: {op}")
    elif indicator == 'B':
        op = body
        arg1, tokens = parse_tokens(tokens)
        arg2, tokens = parse_tokens(tokens)
        if op in '+-*/%<>=':
            op_map = {'+': '+', '-': '-', '*': '*', '/': 'div', '%': 'mod', '<': '<', '>': '>', '=': '=='}
            return f"({arg1} {op_map[op]} {arg2})", tokens
        elif op == '|':
            return f"({arg1} || {arg2})", tokens
        elif op == '&':
            return f"({arg1} && {arg2})", tokens
        elif op == '.':
            return f"({arg1} ++ {arg2})", tokens
        elif op == 'T':
            return f"(take {arg1} {arg2})", tokens
        elif op == 'D':
            return f"(drop {arg1} {arg2})", tokens
        elif op == '$':
            return f"({arg1} {arg2})", tokens
        else:
            raise ValueError(f"Unknown binary operator: {op}")
    elif indicator == '?':
        cond, tokens = parse_tokens(tokens)
        true_branch, tokens = parse_tokens(tokens)
        false_branch, tokens = parse_tokens(tokens)
        return f"(if {cond} then {true_branch} else {false_branch})", tokens
    elif indicator == 'L':
        var_num = base94_to_int(body)
        lambda_body, tokens = parse_tokens(tokens)
        return f"(\\v{var_num} -> {lambda_body})", tokens
    elif indicator == 'v':
        return f"v{base94_to_int(body)}", tokens
    else:
        raise ValueError(f"Unknown indicator: {indicator}")

def transpile_icfp_to_haskell(icfp_code: str) -> str:
    tokens = icfp_code.split()
    haskell_expr, remaining_tokens = parse_tokens(tokens)
    if remaining_tokens:
        raise ValueError("Not all tokens were consumed")
    
    prelude = """
{-# LANGUAGE NoMonomorphismRestriction #-}
module ICFP where

import Data.Char (ord, chr)

base94ToInt :: String -> Int
base94ToInt = foldl (\acc c -> acc * 94 + (ord c - 33)) 0 . reverse

intToBase94 :: Int -> String
intToBase94 0 = "!"
intToBase94 n = reverse $ go n
  where
    go 0 = ""
    go n = chr (n `mod` 94 + 33) : go (n `div` 94)

decodeString :: String -> String
decodeString = map (charMap !!) . map (\c -> ord c - 33)
  where
    charMap = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \\n"

encodeString :: String -> String
encodeString = map (chr . (+ 33) . (flip (!!) charMap))
  where
    charMap = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \\n"

main :: IO ()
main = print result

result :: String
result = """
    
    return prelude + haskell_expr

# Example usage
icfp_code = """B. SF B$ B$ L" B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L$ L# ? B= v# I" v" B. v" B$ v$ B- v# I" Sl I#,"""
haskell_code = transpile_icfp_to_haskell(icfp_code)
print(haskell_code)
