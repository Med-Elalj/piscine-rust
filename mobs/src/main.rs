use ::mobs::*;

fn main() {
    let boss1 = Boss::new("Don Vito", 65);
    let boss2 = Boss::new("Don Carlo", 60);
    let mut mob1 = Mob::new("Corleone", boss1, 1_000_000);
    let mut mob2 = Mob::new("Barzini", boss2, 800_000);

    mob1.recruit(("Michael", 30));
    mob1.recruit(("Sonny", 35));
    mob2.recruit(("Enzo", 28));
    mob2.recruit(("Tessio", 40));

    mob1.attack(&mut mob2);
    mob1.steal(&mut mob2, 100_000);
    mob1.conquer_city(&[&mob2], "New York".to_string());

    println!("{mob1:?}");
    println!("{mob2:?}");
}