use std::io;

fn main()
{
  let mut r=0;
  let mut digit;
  let mut n;
  println!("Enter a number which you want to check whether is it a palindrome or not");
  let mut number = String::new();
  io::stdin().read_line(&mut number).expect("Please enter a number");
  n=number.trim().parse::<u32>().unwrap();
  let m=n;
  while n>0
  {
    digit=n%10;
    r=(r*10)+digit;
    n=n/10;
  }
  if m==r
  {
    println!("the given number {} is a palindrome",m);
  }
  else
  {
    println!("the given number {} is not a palindrome",m);
  }
}
