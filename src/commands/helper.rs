use rand::{thread_rng, Rng};

use crate::{structs::{Type, ERROR, Scope, QueryW, Globals, NodeType, GError, Roots}, canvas::{Canvas, product}, util::{traverse, run_command, get_variable, make_tree}, gerr};


pub fn ovid(cnv : &mut Option<Canvas>) ->Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return Ok(Type::VOID())
    };

    cnv.pixels = {
        let mut v = vec![];

        for _ in 0.. product(&cnv.c_size) {
            v.append(
                &mut vec![
                    thread_rng().gen_range(0..255),
                    thread_rng().gen_range(0..255),
                    thread_rng().gen_range(0..255),
                    255
                ]
            );
        }

        v
    };
    cnv.apply();

    Ok(Type::VOID())
}


pub fn dorbell(args : Vec<Type>, roots : &Roots,glb : &mut Globals, qr : &QueryW, scp : &Scope,
    cnv : &mut Option<Canvas>
) -> Result<Type, ERROR> {
    let var = get_variable(&args[0], &glb.stack)?;

    let Type::STR(ref node) =  var else {
        return Ok(Type::VOID())
    };

    let node = node.replace("(", "[").replace(")", "]");


    let NodeType::Nested(n, c)  = make_tree(&node, true)? else {
        return Ok(Type::VOID())
    };

    let Type::STR(name) = traverse(&n, roots, qr, glb, scp, cnv)? else {
        return Ok(Type::VOID())
    };

    let mut args : Vec<Type> = vec![];
    for child in c  {
        args.push(traverse(&child, roots, qr, glb, scp, cnv)?);
    }
    Ok(run_command(roots, qr,&name, args, glb, scp, cnv)?)

}


pub fn badduck() -> Result<Type, ERROR> {

    let msg = 
    r#"

    I blame you for this.
    ████████████████████████████████████████
    ████████████████████████████████████████
    ████████████████████████████████████████
    ████████████████████████████████████████
    ███████▓▓▓▓▓▓▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▓██████
    ██████▓▓▓▓▓▓╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╣██████
    ██████▓▓▓▓▓▓╫╫╫╫Ñ╫▒▒╫╫╫╫╫╫▒▒▒╫╫╫╫╣██████
    ██████▓▓▓▓▓▓╫╫╫ÑÜ╫▓█▌╠╠╠╠╫██▓╫╫╫╫╣██████
    ██████▓▓▓▓▓▓╫╫╫ÑÜ╩╠╩╫████▓Ñ╫╫╫╫╫╫╣██████
    ██████▓▓▓▓▓▓╫╫╫Ñ╦╦Ü╦╩ÅÅÅÅ╠╩ÑÑ╫╫╫╫╣██████
    ██████▓▓▓▓▓▓╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╫╣▓█████
    ███████▓▓▓▓▓▓╫╫╫╫▓╫╫╫╫╫╫╫╫╫╫╫╫╫▓▓▓██████
    ██████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█████████
    ████████████▓▓▓▓╫╣▓▓▓▓▓▓▓▓▓▓▓███████████
    ████████████▓▓▓╫╫▓▓▓▓▓▓▓▓▓▓▓▓███████████
    ████████████▓▓▓╫╫╫▀▀╫╫╫▓▀▀╫╫▓███████████
    ████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓╫╫▓███████████
    ████████████▓▓▓███████████▓▓▓███████████
    ████████████▓▓▓█████████████████████████
    ████████████▓▓▓█████████████████████████
    ████████████▓▓▓█████████████████████████
    █████████████▓██████████████████████████
    ███████████████████████████████████
    "#;
    print!("{}", msg);
    Ok(Type::VOID())
}

pub fn zayther() -> Result<Type, ERROR> {
    gerr!("Error: Zayther could not be found")
}


pub fn astro() -> Result<Type, ERROR> {
    
    let msg = 
r#"
================================
    #AS

    // You can use single ' quotes to write a single character. 

    change 61294 = 'u'
    change 61295 = 's'
    change 61296 = 'e'
    change 61297 = 'Z'
    change 61298 = '#'

    #loop
    goto #loop
================================
"#;

    println!("{}", msg);

    Ok(Type::VOID())
}


pub fn blid(cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return Ok(Type::VOID())
    };

    if cnv.c_size.0 < 44 || cnv.c_size.1 < 6 {
        return Ok(Type::VOID())
    }


    let arr = [
       1, 1, 0, 0, 1, 0, 0,0, 1, 1, 1, 0,  1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0,0,1,1,1,0,0,1,0,0,1,0,1,1,1,0,1,1,1,0,0,1,1,1,  
       1, 0, 1, 0, 1, 0, 0,0, 0, 1, 0, 0,  1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0,1,0,0,0,0,0,1,0,0,1,0,1,0,0,0,1,0,0,1,0,1,0,0,  
       1, 1, 0, 0, 1, 0, 0,0, 0, 1, 0, 0,  1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0,0,1,1,0,0,0,1,1,1,1,0,1,1,1,0,1,0,0,1,0,1,1,1,  
       1, 0, 1, 0, 1, 0, 0,0, 0, 1, 0, 0,  1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0,0,0,0,1,0,0,1,0,0,1,0,1,0,0,0,1,1,1,0,0,1,0,0,  
       1, 1, 1, 0, 1, 1, 1,0, 1, 1, 1, 0,  1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0,1,1,1,0,0,0,1,0,0,1,0,1,1,1,0,1,0,0,1,0,1,1,1,  
    ];

    
    cnv.clear();
    for y in 0..5 {
        for x in 0..49 {
            let offest = ((y * cnv.c_size.0 * 4) + (x * 4)) as usize;
            let mini_offest = (y * 49 + x) as usize;
            if arr[mini_offest] == 1{
                cnv.pixels[offest..offest+4].copy_from_slice(&[255u8,255,255,255])
            }
        }
    }

    cnv.apply();


    Ok(Type::VOID())
}
