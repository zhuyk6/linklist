#[cfg(test)]
mod tests {
    use linklist::list1::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        list.push(3);

        // println!("list: {:#?}", list);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn format() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        assert_eq!(&format!("{}", list), "List[2, 1, 0]");
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        assert_eq!(vec![&2,&1,&0], list.iter().collect::<Vec<_>>());
    }

    #[test]
    fn iter_mut() {
        let mut list =  List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        for v in list.iter_mut() {
            *v *= 2;
        }

        assert_eq!(vec![&4, &2, &0], list.iter().collect::<Vec<_>>())
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        assert_eq!(vec![2, 1, 0], list.into_iter().collect::<Vec<_>>())
    }
}
