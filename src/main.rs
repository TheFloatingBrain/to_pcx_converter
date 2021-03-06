extern crate image;
extern crate pcx;

fn create_pallete( from : &image::RgbImage ) -> Vec< image::Rgb< u8 > >
{
    let mut pallete : Vec< image::Rgb< u8 > > = Vec::new();
    for pixel in from.pixels()
    {
        let mut indexed = false;
        for color in &pallete
        {
            if pixel == color {
                indexed = true;
                break;
            }
        }
        if indexed == false {
            pallete.push( *pixel );
        }
    }
    return pallete.to_vec();
}

fn create_out_file_path( file_path : &std::path::Path ) -> String
{
    let out_file_name = file_path.file_stem().unwrap().to_str().unwrap();
    let mut out_file_path = file_path.parent().unwrap().to_str().unwrap().to_owned();
    out_file_path.push_str( "/" );
    out_file_path.push_str( out_file_name );
    out_file_path.push_str( ".pcx" );
    return out_file_path;
}

fn write_image_data< Writer : FnMut( &mut Vec< u8 > ) -> std::io::Result< () > >( 
        from : &image::RgbImage, mut write_row : Writer )
{
    let x_bound = from.dimensions().0 as usize;
    let y_bound = from.dimensions().1 as usize;
    let mut row : Vec< u8 > = Vec::new();
    row.resize( x_bound * 3, 0 );
    for y in 0..y_bound
    {
        for x in 0..x_bound
        {
            let current_pixel = from.pixels().nth( ( y * x_bound ) + x ).unwrap();
            row[ x * 3 ] = current_pixel[ 0 ];
            row[ ( x * 3 ) + 1 ] = current_pixel[ 1 ];
            row[ ( x * 3 ) + 2 ] = current_pixel[ 2 ];
        }
        let row_reference : &mut Vec< u8 > = row.as_mut();
        write_row( row_reference ).unwrap();
    }
}

fn write_pcx( file_path : &std::path::Path, from : &image::RgbImage )
{
    let x_bound = from.dimensions().0 as usize;
    let y_bound = from.dimensions().1 as usize;
    let mut pcx_writer = pcx::WriterRgb::create_file( 
            std::path::Path::new( &create_out_file_path( file_path ) ), 
            ( x_bound as u16, y_bound as u16 ), ( 100, 100 ) ).unwrap();
    write_image_data( from, | row : &mut Vec< u8 > | -> std::io::Result< () > { 
        return pcx_writer.write_row( row );
     } );
     pcx_writer.finish().unwrap();
}

fn write_pcx_paletted( file_path : &std::path::Path, from : &image::RgbImage )
{
    let x_bound = from.dimensions().0 as usize;
    let y_bound = from.dimensions().1 as usize;
    let mut pcx_writer = pcx::WriterPaletted::create_file( 
            std::path::Path::new( &create_out_file_path( file_path ) ), 
            ( ( x_bound * 3 ) as u16, y_bound as u16 ), ( 100, 100 ) ).unwrap();
    write_image_data( from, | row : &mut Vec< u8 > | -> std::io::Result< () > { 
        return pcx_writer.write_row( row );
     } );
    let palette : Vec< image::Rgb< u8 > > = create_pallete( from );
    let mut palette_u8 : Vec< u8 > = Vec::new();
    palette_u8.resize( x_bound as usize * 3, 0 );
    for color in 0..palette.len()
    {
        palette_u8[ color * 3 ] = palette[ color ][ 0 ];
        palette_u8[ ( color * 3 ) + 1 ] = palette[ color ][ 1 ];
        palette_u8[ ( color * 3 ) + 2 ] = palette[ color ][ 2 ];
    }
    pcx_writer.write_palette( &palette_u8 ).unwrap();
}

fn main()
{
    let arguments : Vec< String > = std::env::args().collect();
    let file_path = std::path::Path::new( &arguments[ 1 ] );
    let media = image::open( file_path ).unwrap();
    if arguments.len() > 2 {
        write_pcx( file_path, &media.to_rgb() );
    }
    else {
        write_pcx_paletted( file_path, &media.to_rgb() );
    }
}
