use std::error::Error;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::vec::Vec;

fn main() {
    
    /*Accepts the arguments from command line*/	
    let args: Vec<String> = env::args().collect();
    /*Separating options and filename*/
    let mut vec=Vec::new();
    let mut nflag=0;
    let mut bflag=0;
    let mut tflag=0;
    //let mut hflag=0;
	for i in 1..args.len()
	{
		if args[i]=="n"
		{	nflag=1;
		}
		else if args[i]=="b"
		{
			bflag=1;
		}
		else if args[i]=="t"
		{
			tflag=1;
		}
		else if args[i]=="h"
		{
			println!("Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

   b,    number nonempty output lines, overrides -n and strips nonempty lines
   n,    number all output lines              
   t,    display TAB characters as ^I
   h,    display this help and exit
   v,    version  output version information and exit

Examples:
  cat f - g  Output f's contents, then standard input, then g's contents.
  cat        Copy standard input to standard output.
");
			return;
		}
		else if args[i]=="v"
		{
			println!("Version E0256");
		}
		else
		{
			vec.push(&args[i]);
		}
	}

        /*Cat command execution with options*/
	let mut count=1;
    	let mut content = String::new();
	if nflag==1 || bflag==1 ||tflag==1
	{

		for i in vec.iter()
		{
			
				let mut prev=0;
			    let path1 = Path::new(&i);
			    let display1 = path1.display();
			    let mut content1 = String::new();
			    let mut file1 = match File::open(&path1) {
			        Err(why) => panic!("Failed to open {} with error {}", display1, why.description()),
			        Ok(file1) => file1,
			    };
			if nflag==1
			{
			print!("{}\t\t\t",count);
			count+=1;
			file1.read_to_string(&mut content1); 
		        	for s in content1.chars()
				{
					if s=='\n'
					{
						print!("\n{}\t\t\t",count);
						count+=1;
					}
					else
						{
							print!("{}",s);
						}
				}println!();
			}
			else if bflag==1
			{
				
				print!("{}\t\t\t",count);
				count+=1;
				file1.read_to_string(&mut content1); 
		        	for s in content1.chars()
				{
					if s=='\t'||(s=='\n' && prev==1)
					{
						continue;	
					}
					else if s=='\n'
					{
						print!("\n{}\t\t\t",count);
						count+=1;
						prev=1;
					}
					else
						{
					
								print!("{}",s);
								prev=0;
						}
				}println!();
			}
			else if tflag==1
			{
			
					file1.read_to_string(&mut content1); 
			        	for s in content1.chars()
					{
						if s=='\t'
						{
							print!("^I");
						}
						else
							{
								print!("{}",s);
							}
					}println!();		

	
			}

	 	 }
	return;
	}
	/*Plain display of text file*/
	for i in vec.iter()
	{
		  let path = Path::new(&i);
		    let display = path.display();

		    let mut file = match File::open(&path) {
		        Err(why) => panic!("Failed to open {} with error {}", display, why.description()),
		        Ok(file) => file,
		    };


		    match file.read_to_string(&mut content) {
		        Err(why) => panic!("Couldn't read from file {} with error {}", display, why.description()),
		        Ok(_) => print!("{}", content),
		    }
	}
}
