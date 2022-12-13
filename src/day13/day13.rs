use std::cmp::Ordering;

pub fn _run(){
    #[derive(Debug)]
    enum NestedArray{
        L(Vec<NestedArray>),
        I(i32),
    }

    impl PartialEq for NestedArray {
        fn eq(&self, other: &Self) -> bool{
            match (self, other){
                (NestedArray::I(v1), NestedArray::I(v2)) => v1 == v2,
                (l @ NestedArray::L(_),NestedArray::I(v)) => *l == NestedArray::L(vec![NestedArray::I(*v)]),
                (NestedArray::I(v), l @ NestedArray::L(_)) =>  *l == NestedArray::L(vec![NestedArray::I(*v)]),
                (NestedArray::L(a1), NestedArray::L(a2)) => a1.len() == a2.len() && a1.iter().zip(a2.iter()).all(|(v1, v2)| v1 == v2),
            }
        }
    }

    impl Eq for NestedArray {}

    impl Ord for NestedArray{
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self, other){
                (NestedArray::I(v1), NestedArray::I(v2)) => v1.cmp(v2),
                (l @ NestedArray::L(_),NestedArray::I(v)) => (*l).cmp(&NestedArray::L(vec![NestedArray::I(*v)])),
                (NestedArray::I(v), l @ NestedArray::L(_)) => (*l).cmp(&NestedArray::L(vec![NestedArray::I(*v)])).reverse(),
                (NestedArray::L(a1), NestedArray::L(a2)) => {
                    a1.iter().zip(a2.iter()).map(|(v1, v2)| v1.cmp(v2)).find(|comp| *comp != Ordering::Equal).unwrap_or(a1.len().cmp(&a2.len()))
                },
            }
        }
    }

    impl PartialOrd for NestedArray{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    fn split_braces(strng: &str) -> Vec<&str>{
        let mut result = vec![];
        let mut current = 0;
        let mut depth = 0;
        for i in 0..strng.len(){
            let current_char = strng.chars().nth(i).unwrap();
            if depth == 0 && current_char == ','{
                result.push(strng.get(current..i).unwrap());
                current = i + 1;
            }

            if current_char == '[' {
                depth += 1
            }
            if current_char == ']' {
                depth -= 1
            }
        }
        if current < strng.len(){
            result.push(strng.get(current..).unwrap());
        }
        return result
    } 

    fn parse_na(inp: &str) -> NestedArray{
        if inp.chars().next().unwrap() == '['{
            NestedArray::L(split_braces(inp.get(1..(inp.len() - 1)).unwrap()).iter().map(|strng| parse_na(strng)).collect())
        } else {
            NestedArray::I(inp.parse::<i32>().unwrap())
        }
    }

    let part1:usize = include_str!("input").split("\n\n").map(|pair| pair.split("\n").collect::<Vec<&str>>())
        .map(|ps| parse_na(ps[0]) <= parse_na(ps[1])).enumerate().filter(|(_, b)| *b).map(|(i, _)| i + 1).sum();
    println!("{}", part1);

    let mut input = include_str!("input").replace("\n\n", "\n").to_string();
    input.push_str("\n[[2]]\n[[6]]");
    let mut nested_arrays = input.split("\n").map(|s| parse_na(s)).collect::<Vec<NestedArray>>();
    nested_arrays.sort_unstable();

    let part2:usize = nested_arrays.iter().enumerate().filter(|(_, na)| **na == parse_na("[[2]]") || **na == parse_na("[[6]]")).map(|(i,_)| i+1).product();

    println!("{}", part2)
}