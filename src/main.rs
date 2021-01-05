use md2::MDContext;

fn main() {
    let mut ctx = MDContext::new();

    let x: u8 = 'c' as u8;
    ctx.update(x);
    ctx.display();
    ctx.finalize();
    ctx.display();

    println!("Done");
}
