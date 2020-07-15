impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut reverse = 0;
        let mut result = String::from("");
        let mut temp = String::from("");
        let mut should_rerun = false;
        
        for c in s.chars() {
            if c == '(' {
                if reverse > 0 {
                    should_rerun = true;
                    temp.insert(0, ')');
                }
                reverse += 1;
            } else if c == ')' {
                reverse -= 1;
                if reverse == 0 {
                    result.push_str(&temp);
                    temp = String::from("");
                } else {
                    temp.insert(0, '(');
                }
            } else {
                if reverse > 0 {
                    temp.insert(0, c);
                } else {
                    result.push(c);
                }
            }
        }
        if should_rerun {
            Solution::reverse_parentheses(result)
        } else {
            result
        }
    }
}