use std::env;
use rand::Rng;

fn main( )
{
    let arguments: Vec< String > = env::args( ).collect( );
    if arguments.len( ) < 2 || arguments.len( ) > 3
    {
    	println!( "USAGE: passwd_gen <length> [charset]");
    	return;
    }
    let charset_str = gen_charset( arguments.clone( ) );
    let mut charset_vec: Vec< char > = Vec::with_capacity( charset_str.len( ) );
    
    for chr in charset_str.chars( )
    {
    	charset_vec.push( chr );
    }

    let mut passwd: Vec< char > = Vec::new( );
    let mut rng = rand::thread_rng( );

    for _x in 0..arguments[ 1 ].parse::< i32 >( ).unwrap( )
    {
    	let rnd: f32 = rng.gen( );
    	let pw_char: f32 = charset_str.len( ) as f32 * rnd;
    	passwd.push( charset_vec[ pw_char as usize ] );
    }

    let passwd_final: String = passwd.into_iter( ).collect( );
    println!( "{}", passwd_final );
}

fn gen_charset( args: Vec< String > ) -> String
{
	let mut set: String = String::from( "abcdefghijklmnopqrstuvw1234567890" );

	if args.len( ) == 3
	{
		set = args[ 2 ].clone( );
	}

	return set;
}