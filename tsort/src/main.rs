use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use array2d::Array2D;
use std::vec::Vec;
extern crate queues;
use queues::*;
fn main()
{



	//Reading the argument from command line which is the file name
	let args: Vec<String> = env::args().collect();
	
	let m=&args[1];
	if m=="version"
	{
		println!("E0256");
		return;
	}

	else
	if m=="help"
	{
		println!("Usage: tsort [OPTION] [FILE]
Write totally ordered list consistent with the partial ordering in FILE.

With no FILE, or when FILE is -, read standard input.

      help     display this help and exit
      version  output version information and exit");
		return;
	}
	let mut file=File::open(m).expect("Can't open the file!!!");
	
	let mut contents=String::new();
	
	//Reading the contents of the file in var contents
	
	file.read_to_string(&mut contents).expect("Cannot read the file..");
	
			//println!("File contents {}",contents);
	
	let split=contents.split_whitespace();
	
	//inserting the content in a vector
	let vec:Vec<&str>=split.collect();

	
	//Checking the number of input passed in the file

	if vec.len()%2!=0{
		println!("tsort: {}: input contains an odd number of tokens",args[1]);
		return;
	}
	
	//Hash Map
	let mut map:HashMap<&str,usize>=HashMap::new();
	let mut count=0 as usize;
	let mut revmap=Vec::new();
	for i in vec.iter()
	{
		if map.contains_key(i)==false
		{	map.insert(i,count);
			revmap.push(i);
			count+=1;
		}
	}
	


	/*Reverse_hash_map
	let mut revmap:HashMap<usize,&str>=HashMap::new();
	for i in 0..count
	{
		let p =*map.get(vec[i]).unwrap();
		
		revmap.insert(i,vec[i]);
	}
	*/
	let mut rand_vec=Vec::new();
	for i in 0..count*count{
	rand_vec.push(0);
	}
	
	let mut adjmat=Array2D::from_row_major(&rand_vec,count,count);
	let mut k=0;
	for i in 0..vec.len()/2
	{
		let p=*map.get(vec[k]).unwrap();
		let q=*map.get(vec[k+1]).unwrap();
		adjmat[(p,q)]=1;
		k+=2;
	}
	/*
	for i in 0..count
	{
		for j in 0..count
		{
			print!("{}",adjmat[(i,j)]);
		}
		println!();
	}
	*/
	

	let mut in_degree=Vec::new();
	let mut in_degree1= Vec::new();
	for i in 0..count
	{	let mut temp=0;
		for j in 0..count
		{
			temp+=adjmat[(j,i)];
		}
		in_degree.push(temp);
		in_degree1.push(temp);
	}
	
	/*Printing Indegree of each vertex
	for i in 0..count
	{
		println!("{}",in_degree[i]);
	}
	*/

	let mut que: Queue<usize>=queue![];
	let mut que1: Queue<usize>=queue![];
	for i in 0..count
	{
		if in_degree[i]==0
		{
			que.add(i);
			que1.add(i);
		}
	}
	
	//cycle_detection:
	while que1.size()!=0
	{	
		let h1=que1.peek().unwrap();
		for i in 0..count
		{
			if adjmat[(h1,i)]==1
				{	in_degree1[i]-=1;
					if in_degree1[i]==0
					{
						que1.add(i);
					}
				}
		}
		que1.remove();
	}
	
	for i in 0..count
	{
		if(in_degree1[i]>0)
		{
			println!("tsort: {}: input contains a cycle",args[1]);
			return;
		}
	}


	//println!("topological sort");
	/*Performing Topological Sort*/
	while que.size()!=0
	{
		let h=que.peek().unwrap();
		for i in 0..count
		{
			if adjmat[(h,i)]==1
				{	in_degree[i]-=1;
					if in_degree[i]==0
					{
						que.add(i);
					}
				}
		}
		println!("{} ",revmap[h]);
		que.remove();
	}
	
	/*
		Printing Map key,values
	for (key, value) in map.into_iter() {
 		println!("{} : {}", key, value);
		}
	*/

}
