
use macroquad::prelude::*;
use gltf;

fn window_conf() -> Conf{
    Conf{
        window_title: "Race Car".to_owned(),
        sample_count:20,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main(){

    let (gtlf, buffers, images) = gltf::import("assets/car.glb").unwrap();
    let mut car : Vec<Mesh> = Vec::new();
    for mesh in gtlf.meshes(){
        for primitive in mesh.primitives(){
            let reader = primitive.reader(|buffer|{
                Some(&buffers[buffer.index()])
            });
            let mut position_data : Vec<Vertex> = Vec::new();
            let mut normal_data = Vec::new();
            let geo = reader.read_positions();
            let normals = reader.read_normals();
            if let Some(norms) = normals{
                for normal in norms{
                    normal_data.push(normal)
                }
            }
            if let Some(positions) = geo{
                for (i,position) in positions.enumerate(){
                    //println!("{:?}",position);
                    position_data.push(Vertex{
                        position: vec3(position[0],-position[2],position[1]),
                        uv: vec2(0.0,0.0),
                        color:WHITE.into(),
                        normal: vec4(
                            normal_data[i][0],
                            normal_data[i][1],
                            normal_data[i][2],
                            0.0
                        )
                    });
                }
            }
            let index = reader.read_indices();
            let index_data: Vec<u16>;
            match index{
                Some(indices) => {
                    index_data = indices.into_u32().map(|i| i as u16).collect();
                }
                None => {
                    index_data = Vec::new();
                }
            }
            //if let Some(indices) = index{
                //let individual_index_data = indices.into_u32().collect();
              //  index_data.push(individual_index_data);
            //}
            let mesh = Mesh{
                vertices: position_data,
                indices : index_data,
                texture:None
            };
            car.push(mesh);
        }
    }
    let mut min = vec3(f32::MAX, f32::MAX, f32::MAX);
    let mut max = vec3(f32::MIN, f32::MIN, f32::MIN);

    for mesh in &car {
        for vertex in &mesh.vertices {
            min = min.min(vertex.position);
            max = max.max(vertex.position);
        }
    }
    
    let start_position = min+max/2.0;

    let mut car_position = start_position;


    loop{

        clear_background(SKYBLUE);

        

        let camera = Camera3D{
            position: vec3(
                car_position.x-9.0,
                car_position.y+9.0,
                car_position.z - 5.0,
            ),
            target: car_position,
            up: vec3(0.0,1.0,0.0),
            ..Default::default()
        };

        set_camera(&camera);

        let offset = car_position-start_position;
        for mesh in &car{
            let moved_mesh = Mesh {
                vertices: mesh.vertices.iter().map(|v| {
                    let mut vertex = *v;
                    vertex.position += offset;
                    vertex
                }).collect(),
                indices: mesh.indices.clone(),
                texture: mesh.texture.clone(),
            };
            draw_mesh(&moved_mesh)
        }

        if is_key_down(KeyCode::W){
            car_position.x += 1.0;
        }
        if is_key_down(KeyCode::S){
            car_position.x -= 1.0;
        }
        
        draw_grid(20,1.0,YELLOW,DARKGRAY);

        next_frame().await;
    }
}