import operator
import string

# Exception for invalid tokens
class ICFPError(Exception):
    pass

# Token types
class Boolean:
    def __init__(self, value):
        self.value = value

class Integer:
    def __init__(self, value):
        self.value = value

class String:
    def __init__(self, value):
        self.value = value

class UnaryOp:
    def __init__(self, op, operand):
        self.op = op
        self.operand = operand

class BinaryOp:
    def __init__(self, op, left, right):
        self.op = op
        self.left = left
        self.right = right

class If:
    def __init__(self, cond, true_expr, false_expr):
        self.cond = cond
        self.true_expr = true_expr
        self.false_expr = false_expr

class Lambda:
    def __init__(self, var, body):
        self.var = var
        self.body = body

class Variable:
    def __init__(self, var):
        self.var = var

class Apply:
    def __init__(self, func, arg):
        self.func = func
        self.arg = arg

def parse_icfp(tokens):
    if not tokens:
        raise ICFPError("Empty tokens")

    token = tokens.pop(0)
    indicator = token[0]
    body = token[1:]

    if indicator == 'T':
        return Boolean(True)
    elif indicator == 'F':
        return Boolean(False)
    elif indicator == 'I':
        return Integer(parse_base94(body))
    elif indicator == 'S':
        return String(parse_cult_string(body))
    elif indicator == 'U':
        operand = parse_icfp(tokens)
        return UnaryOp(body, operand)
    elif indicator == 'B':
        left = parse_icfp(tokens)
        right = parse_icfp(tokens)
        return BinaryOp(body, left, right)
    elif indicator == '?':
        cond = parse_icfp(tokens)
        true_expr = parse_icfp(tokens)
        false_expr = parse_icfp(tokens)
        return If(cond, true_expr, false_expr)
    elif indicator == 'L':
        var = parse_base94(body)
        body_expr = parse_icfp(tokens)
        return Lambda(var, body_expr)
    elif indicator == 'v':
        return Variable(parse_base94(body))
    elif indicator == 'B$':
        func = parse_icfp(tokens)
        arg = parse_icfp(tokens)
        return Apply(func, arg)
    else:
        raise ICFPError(f"Unknown token indicator: {indicator}")

def parse_base94(s):
    result = 0
    for char in s:
        result = result * 94 + (ord(char) - 33)
    return result

def parse_cult_string(s):
    ascii_order = (
        string.ascii_lowercase + string.ascii_uppercase +
        string.digits + "!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
    )
    return ''.join(ascii_order[ord(c) - 33] for c in s)

def eval_icfp(expr, env=None):
    if env is None:
        env = {}

    if isinstance(expr, Boolean):
        return expr.value
    elif isinstance(expr, Integer):
        return expr.value
    elif isinstance(expr, String):
        return expr.value
    elif isinstance(expr, UnaryOp):
        operand = eval_icfp(expr.operand, env)
        if expr.op == '-':
            return -operand
        elif expr.op == '!':
            return not operand
        elif expr.op == '#':
            return parse_base94(operand)
        elif expr.op == '$':
            return ''.join(chr(33 + (operand // (94 ** i)) % 94) for i in range(len(operand)))
        else:
            raise ICFPError(f"Unknown unary operator: {expr.op}")
    elif isinstance(expr, BinaryOp):
        left = eval_icfp(expr.left, env)
        right = eval_icfp(expr.right, env)
        ops = {
            '+': operator.add,
            '-': operator.sub,
            '*': operator.mul,
            '/': operator.floordiv,
            '%': operator.mod,
            '<': operator.lt,
            '>': operator.gt,
            '=': operator.eq,
            '|': operator.or_,
            '&': operator.and_,
            '.': operator.add,  # string concatenation
            'T': lambda x, y: y[:x],  # take first x chars of string y
            'D': lambda x, y: y[x:]   # drop first x chars of string y
        }
        if expr.op in ops:
            return ops[expr.op](left, right)
        elif expr.op == '$':
            if isinstance(left, Lambda):
                new_env = env.copy()
                new_env[left.var] = right
                return eval_icfp(left.body, new_env)
            else:
                raise ICFPError("First argument to $ must be a lambda")
        else:
            raise ICFPError(f"Unknown binary operator: {expr.op}")
    elif isinstance(expr, If):
        cond = eval_icfp(expr.cond, env)
        if cond:
            return eval_icfp(expr.true_expr, env)
        else:
            return eval_icfp(expr.false_expr, env)
    elif isinstance(expr, Lambda):
        return expr
    elif isinstance(expr, Variable):
        if expr.var in env:
            return eval_icfp(env[expr.var], env)
        else:
            raise ICFPError(f"Unbound variable: {expr.var}")
    elif isinstance(expr, Apply):
        func = eval_icfp(expr.func, env)
        arg = expr.arg
        if isinstance(func, Lambda):
            new_env = env.copy()
            new_env[func.var] = arg
            return eval_icfp(func.body, new_env)
        else:
            raise ICFPError("First argument to application must be a lambda")
    else:
        raise ICFPError("Unknown expression type")

def interpret_icfp(code):
    tokens = code.split()
    expr = parse_icfp(tokens)
    return eval_icfp(expr)

# Example usage
icfp_code = "B$ L# B$ L$ v# B. SB%,,/ S}Q/2,$_ IK"
result = interpret_icfp(icfp_code)
print(result)  # Should print "Hello World!"

