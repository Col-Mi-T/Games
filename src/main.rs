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
        
        /*draw_cylinder(
            vec3(2.0,0.5,0.0),
            0.5, //radius top
            0.5, // radius bottom
            0.05, //height
            None,
            GOLD
        );*/

       
        draw_grid(20,1.0,GRAY, DARKGRAY);
                
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
        if is_key_pressed(KeyCode::Space) && objects.len()>5{
            objects.remove(5); // I can delete a coin from the list of objects, but I need to make sure that the index is valid. The first 5 objects are the camera and the cube, so I can only remove coins starting from index 5.
        }
        if is_key_down(KeyCode::Escape){
            break;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up){
            cube.cube_position.z -= 0.1;
        }
        if is_key_down(KeyCode::S)||is_key_down(KeyCode::Down){
            cube.cube_position.z += 0.1;
        }
        if is_key_down(KeyCode::A)||is_key_down(KeyCode::Left){
            cube.cube_position.x -= 0.1;
        }
        if is_key_down(KeyCode::D)||is_key_down(KeyCode::Right){
            cube.cube_position.x += 0.1;
        }
        next_frame().await;
    }
}