use md2::MDContext;

fn display(ctx: MDContext) {
}

fn main() {
    let mut ctx = MDContext::new();

    let x: u32 = 'c' as u32;
    ctx.update(x);
    ctx.display();
    ctx.finalize();
    ctx.display();

    println!("Done");
}
