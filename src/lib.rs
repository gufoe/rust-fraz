pub mod util;

// use nalgebra::base::Vector3;
use std::collections::HashMap;
use ocl::*;
use nalgebra::base::Vector3;
pub type Vec3 = Vector3<f32>;
use ocl::prm::*;
use crate::util::Timer;
use ocl::ProQue;


pub trait SceneNode {
    fn dist_to(&self, point: &Vec3) -> f32;
}

pub struct Sphere {
    rad: f32,
    c: Vec3,
}

impl Sphere {
    pub fn new(rad: f32, c: Vec3) -> Self {
        Self { rad, c }
    }
}

impl SceneNode for Sphere {
    fn dist_to(&self, point: &Vec3) ->f32 {
        point.metric_distance(&self.c) - self.rad
    }
}

pub enum Shape {
    Sphere(Sphere),
}
impl SceneNode for Shape {
    fn dist_to(&self, point: &Vec3) ->f32 {
        match self {
            Shape::Sphere(x) => x.dist_to(point),
        }
    }
}

#[derive(Debug)]
pub struct MarchHit {
    pub node_id: usize,
    pub point: Vec3,
}
#[derive(Debug)]
pub struct MarchResult {
    pub hit: Option<MarchHit>,
}



pub struct Scene {
    objects: HashMap<usize, Shape>,
    next_id: usize,

    ocl_pq: ProQue,
    // ret_hits_buf: Buffer<f32>,
    // cl_kernel: Kernel,
}

impl Scene {
    pub fn new() -> Self {

        let src = std::fs::read_to_string("/home/jake/.tmp/fraz/gl/shader.cl").unwrap();
        Self {
            objects: HashMap::new(),
            next_id: 1,
            ocl_pq: ProQue::builder()
                .src(src)
                .build()
                .expect("ocl err"),
            // self.ocl_pq,
            // ret_hits_buf,
            // cl_kernel,
        }
    }

    pub fn multi_march(&mut self,
        w: u32, h: u32,
        eye: &Vec3, rot: &(f32, f32),
        tex: &mut [u8],
    ) -> HashMap<(u32, u32), MarchResult> {
        // use ocl::traits::WorkDims;
        const PIX_PER_THREAD: usize = 1;
        let pixels = (w*h) as usize;
        let threads = pixels / PIX_PER_THREAD;
        let mut ticker = Timer::new("start");
        self.ocl_pq.set_dims(threads);
        ticker.tick("set_dims");

        // let mut spheres = vec![];
        // self.objects.values().for_each(|x| match x {
        //     Shape::Sphere(x) => {
        //         spheres.push(Float4::new(x.c.x, x.c.y, x.c.z, x.rad));
        //     },
        // });
        //
        // ticker.tick("created spheres");


        // let spheres_buf = Buffer::builder()
        //     .queue(self.ocl_pq.queue().clone())
        //     .flags(MemFlags::READ_ONLY)
        //     .len(spheres.len())
        //     .copy_host_slice(&spheres)
        //     .build()
        //     .unwrap();
        // ticker.tick("created spheres buf");

        let ret_tex_buf = Buffer::builder()
            .queue(self.ocl_pq.queue().clone())
            .flags(MemFlags::new().host_read_only())
            .len(pixels * 4)
            .build()
            .unwrap();
        ticker.tick("created tex buf");

        let ret_hits_buf: Buffer<Float3> = Buffer::builder()
            .queue(self.ocl_pq.queue().clone())
            .flags(MemFlags::new().read_write())
            .len(pixels)
            .build()
            .unwrap();
        ticker.tick("created hits buf");

        let cl_kernel = self.ocl_pq.kernel_builder("march")
            .arg_named("w", w)
            .arg_named("h", h)
            .arg_named("eye", Float3::new(eye.x, eye.y, eye.z))
            .arg_named("rot", Float2::new(rot.0, rot.1))
            // .arg_named("spheres", &spheres_buf)
            .arg_named("ret_hits", &ret_hits_buf)
            .arg_named("ret_tex", &ret_tex_buf)
            .build()
            .expect("ocl err");
        ticker.tick("created cl_kernel");

        unsafe { cl_kernel.enq().expect("ocl cannot enq"); }
        ticker.tick("enqueued cl_kernel");
        self.ocl_pq.finish().unwrap();
        ticker.tick("finish cl_kernel");

        ret_tex_buf.read(tex).enq().expect("cannot enq read");
        ticker.tick("read tex");
        // let mut hits = vec![Float3::new(0.0, 0.0, 0.0); threads];
        // ticker.tick("alloc hits");
        // ret_hits_buf.read(&mut hits).enq().expect("cannot enq read");
        // ticker.tick("read hits");
        ticker.show();

        let ret = HashMap::with_capacity(1);
        // for gid in 0..threads {
        //     let x = (gid as u32) % w;
        //     let y = (gid as u32) / w;
        //     let v = Vec3::new(vec[gid][0], vec[gid][1], vec[gid][2]);
        //     ret.insert((x, y), MarchResult {
        //         hit: if v.magnitude() > 0.0 {
        //             Some(MarchHit {
        //                 node_id: 0,
        //                 point: v,
        //             })
        //         } else {
        //             None
        //         }
        //     });
        // }
        ret
    }

}

impl Scene {
    pub fn add_node(&mut self, n: Shape) -> usize {
        let id = self.next_id;
        self.next_id+= 1;
        self.objects.insert(id, n);
        id
    }
    pub fn march(&self, from: &Vec3, mut dir: Vec3, max_iter: usize, min_d: f32) -> MarchResult {
        let mut p = from.clone();
        dir = dir.normalize();

        for _ in 0..max_iter {
            let min = self.objects.iter()
            .map(|(node_id, n)| (node_id, n.dist_to(&p)))
                .fold(None, |min, x| match min {
                    None => Some(x),
                    Some(a) => Some(if a.1 < x.1 { a } else { x }),
                });

            match min {
                Some((node_id, dist)) => {
                    if dist < min_d {
                        return MarchResult {
                            hit: Some(MarchHit {
                                node_id: *node_id,
                                point: p,
                            }),
                        };
                    } else {
                        p+= dir * dist;
                    }
                },
                None => panic!("no objects in scene"),
            }

        }

        MarchResult {
            hit: None,
        }
    }
}







#[test]
fn main() {
    let eye = (
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, -1.0),
    );

    let mut scene = Scene::new();
    scene.add_node(Shape::Sphere(Sphere::new(5.0, Vec3::zeros())));

    // CPU marching
    let res = scene.march(&eye.0, eye.1, 20, 1.0);
    let hit = res.hit.expect("CPU no hit");
    assert!((hit.point - Vec3::new(0.0, 0.0, 5.0)).magnitude() < 1.0);


    // GPU marching
    let w = 5000 as u32;
    let h = 5000 as u32;
    let mut img = vec![0; (w*h*4) as usize];
    let res = scene.multi_march(w, h, &eye.0, &(0.0, 3.14), &mut img);
    let res = res.get(&(w/2, h/2)).expect("GPU no hit"); // central point
    let hit = res.hit.as_ref().expect("GPU no hit");
    println!("{:#?}", hit);
    assert!((hit.point - Vec3::new(0.0, 0.0, 5.0)).magnitude() < 1.0);

    println!("Tests passed");
}
