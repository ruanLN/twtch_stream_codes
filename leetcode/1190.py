class Solution:
    def reverseParentheses(self, s: str) -> str:
        result = ""
        temp = ""
        reverse = 0
        should_rerun = False
        for char in s:
            if char is '(':
                if reverse > 0:
                    should_rerun = True
                    temp = ')' + temp
                reverse += 1
            elif char is ')':
                reverse -= 1
                if reverse is 0:
                    result += temp
                    temp = ""
                else:
                    temp = '(' + temp
            else:
                if reverse > 0:
                    temp = char + temp
                else:
                    result += char
        if should_rerun:
            return self.reverseParentheses(result)
        else:
            return result





