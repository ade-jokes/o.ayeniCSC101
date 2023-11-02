fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A{:?}",a);
    println!("Value of B{:?}",b );

    let mut res= a>b;
    println!("A is greater than B{:?}",res );

    res= a <b;
    println!("A is lesser than B{:?}",res );

    res=a>=b;
    println!("A is greater than or equal to B: {:?}",res );

    res=a <=b;
    println!("A is lesser of equals to B {:?}",res );

     res= a==b;
     println!("A is equals to B{:?}",res );

     res= a!=b;
     println!("A is not equals to B{:?}",res );
}
