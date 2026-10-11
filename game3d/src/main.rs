/*
A simple Cube Collection 3d Game for an introduction to 3d game development from scratch
*/

use macroquad::prelude::*;
use ::rand::RngExt;

fn window_conf() -> Conf{
    Conf{
        window_title:"Cube Collector".to_owned(),
        sample_count:10,
        ..Default::default()
    }
}

fn make_coin(center_position: Vec3) -> Mesh {
    let radius = 0.5;
    let height = 0.2;
    let sides = 32;

    let mut verticies: Vec<Vertex> = Vec::new();
    let mut indicies: Vec<u16> = Vec::new();

    // Back face center
    verticies.push(Vertex {
        color: GOLD.into(),
        position: vec3(
            center_position.x,
            center_position.y,
            center_position.z - height / 2.0,
        ),
        normal: vec4(0.0, 0.0, -1.0, 0.0),
        uv: vec2(0.0, 0.0),
    });

    // Front face center
    verticies.push(Vertex {
        color: GOLD.into(),
        position: vec3(
            center_position.x,
            center_position.y,
            center_position.z + height / 2.0,
        ),
        normal: vec4(0.0, 0.0, 1.0, 0.0),
        uv: vec2(0.0, 0.0),
    });

    // Rim vertices: back and front alternate
    for i in 0..sides {
        let angle =
            i as f32 * 2.0 * std::f32::consts::PI / sides as f32;

        let x = center_position.x + radius * angle.cos();
        let y = center_position.y + radius * angle.sin();

        verticies.push(Vertex {
            color: GOLD.into(),
            position: vec3(x, y, center_position.z - height / 2.0),
            normal: vec4(0.0, 0.0, -1.0, 0.0),
            uv: vec2(0.0, 0.0),
        });

        verticies.push(Vertex {
            color: GOLD.into(),
            position: vec3(x, y, center_position.z + height / 2.0),
            normal: vec4(0.0, 0.0, 1.0, 0.0),
            uv: vec2(0.0, 0.0),
        });
    }

    // Back and front faces
    for i in 0..sides {
        let current_back = (i * 2 + 2) as u16;
        let next_back = (((i + 1) % sides) * 2 + 2) as u16;

        let current_front = (i * 2 + 3) as u16;
        let next_front = (((i + 1) % sides) * 2 + 3) as u16;

        // Back face
        indicies.extend_from_slice(&[0, next_back, current_back]);

        // Front face
        indicies.extend_from_slice(&[1, current_front, next_front]);

        // Side wall: two triangles connecting each pair of rim edges
        indicies.extend_from_slice(&[
            current_back,
            next_back,
            current_front,
            next_back,
            next_front,
            current_front,
        ]);
    }

    Mesh {
        vertices: verticies,
        indices: indicies,
        texture: None,
    }
}

struct Cube<T>{
    cube_position: Vec3,
    cube_size:Vec3,
    texture:std::option::Option<T>,
    color:Color
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GameObject{
    Camera,
    Cube,
    Coin
}

#[macroquad::main(window_conf)]
async fn main(){
    let cube_position = vec3(0.0, 0.5, 0.0);
    
    let mut objects: Vec<GameObject> = Vec::new();

    let mut cube = Cube{
        cube_position:cube_position,
        cube_size:vec3(1.0,1.0,1.0),
        texture:None,
        color:BLUE
    };

    objects.push(GameObject::Camera);
    objects.push(GameObject::Cube);

    //let mut rng = ::rand::rng();
    let mut coin_positions = Vec::new();

    for _ in 0..10 {
        let mut rng = ::rand::rng();

        let x = rng.random_range(-9.0..9.0);
        let z = rng.random_range(-9.0..9.0);

        coin_positions.push(vec3(x, 0.5, z));
        objects.push(GameObject::Coin);
    }
    println!("{:#?}",objects);
    loop{
        clear_background(SKYBLUE);

        let camera = Camera3D{
            position:vec3(
                cube.cube_position.x ,
                cube.cube_position.y + 2.0,
                cube.cube_position.z +5.0
            ),
            target: cube.cube_position,
            up: vec3(0.0,1.0,0.0),
            ..Default::default()
        };
        
    

       
        
                
        let mut coin_id = 0;
        for obj in &objects{
            
            if obj == &GameObject::Camera{
                set_camera(&camera);
            }
            if obj == &GameObject::Cube{
                draw_cube(cube.cube_position,cube.cube_size,cube.texture,cube.color);
            }
            if obj == &GameObject::Coin{
                let coin = make_coin(coin_positions[coin_id]);
                draw_mesh(&coin);
                coin_id += 1;
            }
            //println!("{:#?}",obj);
        }
        draw_grid(20,1.0,GRAY, DARKGRAY);
        
        set_default_camera();
        draw_text("Cube Collector", 20.0, 30.0, 30.0, WHITE);
        draw_text("Collect all the coins!", 20.0, 60.0, 20.0, WHITE);  //x, y, font, color
        let progress = format!("Coins collected: {}/10", 10-coin_positions.len());
        draw_text(progress,20.0,90.0,20.0,WHITE);

        let font_size = 20.0;
        let text = "Use W,A,S,D or Arrow Keys to move the character";
        let text_width = measure_text(text, None, font_size as u16, 1.0).width;

        draw_text(
            text,
            screen_width() - text_width - 20.0,
            screen_height() - 20.0,
            font_size,
            WHITE,
        );

        let message = "You Collected All the coins!";
        let font_size = 50.0;

        let text_size = measure_text(message, None, font_size as u16, 1.0);

        if 10-coin_positions.len() == 10{
            draw_text(
                message,
                (screen_width() - text_size.width) / 2.0,
                (screen_height() + text_size.height) / 2.0,
                font_size,
                GREEN,
            );
        }
        
        let mut coin_to_remove = None;
        for (index, position) in coin_positions.iter().enumerate() {
            let cube_pos = cube.cube_position;
            let coin_pos = *position;
            let distance = cube_pos.distance(coin_pos);
            if distance <= 1.0 && objects.len() > index+2{
                coin_to_remove = Some(index);
                break;
            }
        }
        if let Some(index) =  coin_to_remove{
            objects.remove(index + 2); // Remove the coin from the objects vector
            coin_positions.remove(index); // Remove the coin position from the coin_positions vector
        // Check collision with THIS coin
        }


        if is_key_down(KeyCode::Escape){
            break;
        }
        
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) && cube.cube_position.z > -10.0{
            cube.cube_position.z -= 0.1; //foward
        }
        if is_key_down(KeyCode::S)||is_key_down(KeyCode::Down) && cube.cube_position.z < 10.0{
            cube.cube_position.z += 0.1; //backward
        }
        if is_key_down(KeyCode::A)||is_key_down(KeyCode::Left)&& cube.cube_position.x > -10.0{
            cube.cube_position.x -= 0.1; //left
        }
        if is_key_down(KeyCode::D)||is_key_down(KeyCode::Right)&& cube.cube_position.x < 10.0{
            cube.cube_position.x += 0.1; //right
        }
        next_frame().await;
    }
}