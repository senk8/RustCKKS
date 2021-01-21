pub fn gcd(a:isize,b:isize)->isize
{
    if a%b == 0 { a }
    else{ gcd(b,a%b) }
}

pub fn lcm(a:isize,b:isize)->isize
{
    (a*b)/gcd(a,b)
}

//pub fn crt()