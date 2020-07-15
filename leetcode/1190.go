func reverseParentheses(s string) string {
    var reversed = 0
    var solution = ""
    var temp = ""
    var should_rerun = false
    
    for _, c := range s {
        if c == '(' {
            if reversed > 0 {
                should_rerun = true
                temp = ")" + temp
            }
            reversed++
        } else if c == ')' {
            reversed--
            if reversed == 0 {
                solution += temp
                temp = ""
            } else {
                temp = "(" + temp
            }
        } else {
            if reversed > 0 {
                temp = string(c) + temp
            } else {
                solution += string(c)
            }
        }
	}
    if should_rerun {
        return reverseParentheses(solution)
    } else {
        return solution
    }
}