pub fn ownership()
{
    let v = vec![1,2,3];

    //let v2 = v; // copying a pointer, not the actual values - both v and v2 cannot bind to the resource

    // println!("{:?}", v); -> v is now invalid

    let foo = |v:Vec<i32>| (); // this closure grabs v and it is not usable after 
    //foo(v);

    let print_vector_and_return = |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", x);
        return x // returns the var after it is done owning it
    };    
    //let vv = print_vector(v);

    let print_vector = |x:&Vec<i32>| // takes a borrowed pointer
    {
        println!("x[0] = {}", x[0]);
    };

    let vvv = vec![3,2,1];
    print_vector(&vvv); // <- borrowing
    println!("v[0] = {}", v[0]);

    let mut a = 40;

    {
        let b = &mut a; // b is borrowing a -> must be put in a scope
        *b += 2;
    }
    println!("a = {}", a);

    let mut z = vec![3,2,1];

    for i in &z{
        println!("i = {}", i);
        // z.push(5); not allowed
    }
}