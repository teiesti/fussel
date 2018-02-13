pub fn list<I>(iter: &mut I, delim: &str, delim_last: &str) -> String
where
    I: Iterator,
    <I as Iterator>::Item: AsRef<str>,
{
    let mut result = String::new();

    let mut cur = iter.next();
    let mut succ1 = iter.next();
    let mut succ2 = iter.next();

    while cur.is_some() {
        result += cur.unwrap().as_ref();

        result += match (&succ1, &succ2) {
            (&Some(_), &Some(_)) => delim,
            (&Some(_), &None   ) => delim_last,
            (&None   , _       ) => "",
        };

        cur   = succ1;
        succ1 = succ2;
        succ2 = iter.next();
    }

    result
}

pub fn list_and<I>(iter: &mut I) -> String
where
    I: Iterator,
    <I as Iterator>::Item: AsRef<str>,
{
    list(iter, ", ", " and ")
}

pub fn list_or<I>(iter: &mut I) -> String
where
    I: Iterator,
    <I as Iterator>::Item: AsRef<str>,
{
    list(iter, ", ", " or ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_with_zero() {
        let vec: Vec<&str> = vec![];
        assert_eq!(
            list(&mut vec.iter(), ", ", " and "),
            "",
        );
    }

    #[test]
    fn list_with_one() {
        let vec = vec!["1"];
        assert_eq!(
            list(&mut vec.iter(), ", ", " and "),
            "1",
        );
    }

    #[test]
    fn list_with_two() {
        let vec = vec!["1", "2"];
        assert_eq!(
            list(&mut vec.iter(), ", ", " and "),
            "1 and 2",
        );
    }

    #[test]
    fn list_with_three() {
        let vec = vec!["1", "2", "3"];
        assert_eq!(
            list(&mut vec.iter(), ", ", " and "),
            "1, 2 and 3",
        );
    }

    #[test]
    fn list_with_four() {
        let vec = vec!["1", "2", "3", "4"];
        assert_eq!(
            list(&mut vec.iter(), ", ", " and "),
            "1, 2, 3 and 4",
        );
    }
}
