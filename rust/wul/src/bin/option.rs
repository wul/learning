fn main() {
    let mut x=Some(2);
    let old=x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let x=12;
    let opt_x=Some(&x);
    assert_eq!(opt_x, Some(&12));
    let copied=opt_x.copied();
    assert_eq!(copied, Some(12));

    let x=12;
    let opt_x=Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned=opt_x.cloned();
    assert_eq!(cloned, Some(12));


    let good_year_from_input = "1909";
    let bad_year_from_input = "a190blarg";
    let good_year = good_year_from_input.parse().ok().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);


    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));
    
    let x: Option<String> = None;
    assert_eq!(x.as_deref(), None);
    let x: Option<Option<u32>> = Some(Some(6));
    assert_eq!(Some(6), x.flatten());
    
    let x: Option<Option<u32>> = Some(None);
    assert_eq!(None, x.flatten());
    
    let x: Option<Option<u32>> = None;
    assert_eq!(None, x.flatten());

    let v = Option::<u32>::None;
    assert!(v.is_none());
    println!("None is_none");


    let mut v = Some("abc".to_string());

    if let Some(s) = v.as_mut() {
	println!("{}", s.to_uppercase());
	*s = "ABC".to_string();
	println!("{}", s);	
    }

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");
    let mut opt = None;
    let val = opt.insert(1);
    assert_eq!(*val, 1);
    assert_eq!(opt.unwrap(), 1);
    let val = opt.insert(2);
    assert_eq!(*val, 2);
    //but val is an ref, not mut ref???
    *val = 3;
    assert_eq!(opt.unwrap(), 3);


    let v = Option::<u32>::None;
    assert_eq!(v.clone(), None);

    let v = Some(5);
    assert_eq!(v.clone(), Option::Some(5));
}
