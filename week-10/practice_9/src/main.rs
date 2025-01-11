struct Rectangle{
    width : u32, height : u32
}

//logic to calculate area of a rectangle 
impl Rectangle {
    fn area(&self) ->u32{
        self.width * self.height
    }
}

fn main() {
   let small = Rectangle{
    width: 10,
    height: 20
   };

   println!("width is : {} \nheight is : {} \nArea of the Rectangle is {}", small.width,small.height,small.area());
}
