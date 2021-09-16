fn main() {
	let v1 = vec![1, 2, 3];
	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
	/*
	   Here, map() takes closure as parameter.
	   So we can define any operation we want to perform on each item.
	   But we have to collect the result by using 'collect()'
	*/
	assert_eq!(v2, vec![2, 3, 4]);


	/*
	   Question1
	   I can not understand why we should annotate 'v2' a specific type
	   When i didn't annotate its type, it made compile error. (error message : type annotation needed)

	   Question2
	   And what does 'Vec<_>' means? 
	   I have tried to change it as Vec<i31>, and it worked without problem.

	   I think Vec<_> means that the variable is Vector whatever data type it includes.
	   But still confused about why that annotation is needed...


	   

	   Answer

	   I searched about how collect() works and got some answer of my question.
	   In rust documentation of Trait Iterator, 
	   "Note that we needed the : Vec<i32> on the left-hand side. This is because we could collect into"

	   https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect

	   I found that these are also available..
	   let v2 = v1.iter().map(|x| x + 1).collect::<Vec<i32>>();
	   let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>(); 



	   underscore '_' means partial type hint.
	   <SomeType>. -> This syntax of rust is called 'Turbofish'
       
       https://techblog.tonsser.com/posts/what-is-rusts-turbofish
	 */

}
