fn main() {
	let day = [
		"first",
		"second",
		"third",
		"fourth",
		"fifth",
		"sixth",
		"seventh",
		"eighth",
		"ninth",
		"tenth",
		"eleventh",
		"twelfth"
	];

	let lirics = [
		"A partridge in a pear tree",
		"Two turtle doves",
		"Three French hens",
		"four calling birds",
		"five gold rings",
		"six geese a-laying",
		"seven swans a-swimming",
		"eight maids a-milking",
		"nine ladies dancing",
		"ten lords a-leaping",
		"eleven pipers piping",
		"twelve drummers drumming"
	];
	
	for n in 0..12{
		println!("[{}]",n+1);
		println!("On the {} day of Christmas, my true love sent to me", day[n]);
	
		for m in (0..n+1).rev(){
			if m == 0 && n >0 {
				println!("and");
			}
			println!("{}",lirics[m]);
		}
	}

}
