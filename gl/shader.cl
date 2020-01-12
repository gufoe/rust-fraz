// char sign(float x) {
//   return x > 0 ? 1 : -1;
// }

// float DE(float3 z) {
//   // return distance(z, 0.0) - 1;
//
//   // if (cos(view.x * view.y) < 0.9) return 0;
//   float Scale = 100;
//   float Offset = 0;
//   int Iterations = 5;
// 	float3 a1 = (float3){1,1,1};
// 	float3 a2 = (float3){-1,-1,1};
// 	float3 a3 = (float3){1,-1,-1};
// 	float3 a4 = (float3){-1,1,-1};
// 	float3 c;
// 	int n = 0;
// 	float dist, d;
// 	while (n < Iterations) {
// 		 c = a1; dist = length(z-a1);
// 	        d = length(z-a2); if (d < dist) { c = a2; dist=d; }
// 		 d = length(z-a3); if (d < dist) { c = a3; dist=d; }
// 		 d = length(z-a4); if (d < dist) { c = a4; dist=d; }
// 		z = Scale*z-c*(Scale-1.0f);
// 		n++;
// 	}
//
// 	float x =  length(z) * pow(Scale, (float)(-n)) - 0.8;
//   return x > 0 ? x : 0;
// }

inline float maxf(float a, float b) {
  return a > b ? a : b;
}
inline float DE(float3 z) {
  // return maxf(0, length(z)-1);

  float Scale = 2;
  float Offset = 4;
  int Iterations = 4;
  float r;
  int n = 0;
  while (n < Iterations) {
     if(z.x+z.y<0) z.xy = -z.yx; // fold 1
     if(z.x+z.z<0) z.xz = -z.zx; // fold 2
     if(z.y+z.z<0) z.zy = -z.yz; // fold 3
     z = z*Scale - Offset*(Scale-1.0f);
     n++;
  }
  float x = (length(z) ) * pow(Scale, -(float)(n)) - 10;
  return maxf(x, 0);
}


float3 calcNormal(float3 p, float dx) {
	const float3 k = (float3){1,-1,0};
	return normalize(k.xyy*DE(p + k.xyy*dx) +
					 k.yyx*DE(p + k.yyx*dx) +
					 k.yxy*DE(p + k.yxy*dx) +
					 k.xxx*DE(p + k.xxx*dx));
}


float3 mod_f3(float3 v, float u) {
  // v.x = v.x*v.y;
  // v.y = cos(v.y)* 10;
  // v.x = cos(v.x*0.5f)*0.1;

  // Spheric mod
  // float n = fabs(v[0]);
  // n = max(n, fabs(v[1]));
  // n = max(n, fabs(v[2]));
  //   // v*= s * (int)(n/s);
  //
  //   v.x-= s * (int) (n/s);
  //   v.y-= s * (int) (n/s);
    // v.z-= s * (int) (n/s);
  // u *= (u*2) * (int) (length(v) + u) / (u*2);
  // u = pow(length(v)*0.1f, pow(1.1f+fabs(v.x*v.y), 2));
  u = pow(length(v) / (10+0.1f*length(v)), 2);
  // u*= pow(pow(length(v), 2)*10 / (100+0.1f*length(v)), 1.5);
  // u = pow(pow(length(v), 2)*1 / (10+0.1f*length(v)), 0.5);
  // u = 4+10+sqrt(fabs(v.x*v.y)*10.0f)*log(fabs(v.y)*10.0f) + v.z*0.1;

   // + pow(cos(v.y/20), 4) + pow(v.x/20, 2) / (10+fabs(v.y));

   float u2;
  //
  // u = 6;
  u2 = u*2;
  v.x-= sign(v.x) * u2 * floor((fabs(v.x) + u) / u2);
  v.y-= sign(v.y) * u2 * floor((fabs(v.y) + u) / u2);
  v.z-= sign(v.z) * u2 * floor((fabs(v.z) + u) / u2);

    //
    // u = pow(1.99, length(v)*1.1f);

  return v;
}

float3 add_color(float3 a, float3 b, float p) {
  float q = 1.0f-p;
  return (a * q + b * p);
}

__kernel void march(
  __private int const w,
  __private int const h,
  __private float3 eye,
  __private float2 rot,
  __global float3 *ret_hits,
  __global uchar4 *ret_tex
) {


  const PIX_PER_THREAD = 1;
  int THREADS = w*h;
  const _gid = get_global_id(0);
  for (int gid = _gid*PIX_PER_THREAD; gid < (_gid+1)*PIX_PER_THREAD && gid < THREADS; gid++) {
    float MIN_DIST = 0.0001;
    float MAX_DIST = 500.0;
    int MAX_STEPS = 100;


    const x = gid % w;
    const y = gid / w;
    float3 view = eye;
    float2 look = rot;
    float ratio = w/(float)h;
    float ax = (x / (float) w - 0.5) * 3.14 * 0.4 * ratio;
    float ay = (y / (float) h - 0.5) * 3.14 * 0.4;
    look.x+= ax;
    look.y+= ay;
    float3 dir = ((float3){
      - cos(look.y) * sin(look.x),
      - sin(look.y),
      cos(look.y) * cos(look.x)
    });


    float tot_d = 0.0;
    float min_d = 0.0;
    float3 col = 0.0;(float3){0.05, 0.1, 0.1};
    float3 mat = (float3){0.2, 1.0, 0.0};
    float3 refl = (float3){ 0.0, 0.9, 0.4 };
    bool hit = false;
    int step = 0;
    for (; step < MAX_STEPS; step++) {
      MIN_DIST*= 1.05;
      float3 p = mod_f3(view, 3.0);
      float d = DE(p);
      tot_d+= d;
      if (step == 0 || d < min_d) min_d = d;

      if (d < MIN_DIST) {
        hit = true;
        // ret_hits[gid] = view;
        break;
      }
      if (tot_d > MAX_DIST) {
        break;
      }
      // ret_hits[gid] = view;
      view+= dir*d;
    }

    // col[0] = 100 * (1-fmin(min_d, 1));
    // col[1] = 100 * (1-fmin(min_d, 1));
    // col[2] = 100 * (1-fmin(min_d, 1));
    // if (!hit) {
    //   col*= (float3){ 1.0, 1.0, 0.0 };
    // }
      // col.y*= 1 / (1 + min_d * 1000.0);
    // }
    // col = add_color(col, (float3){ 1.0, 1.0, 1.0 }, 2, 0.5/(1.3+min_d*min_d));
    // col*= 30.0f / (30.0f+step + min_d/MIN_DIST);
    // if (step < MAX_STEPS) {
      // col = add_color(col, (float3){0.0,0.0,0.0}, min_d/MIN_DIST, 1);
    // } else {
    // }
    if (min_d > MIN_DIST)  {
      col = add_color(col, refl, 0.1f / (0.1f+pow(min_d, 2)*0.01f));
    } else {
      col = add_color(refl, 0, 30.0f / (30.0f+(step + min_d/MIN_DIST)));
      col = add_color(refl, col, 30.0f / (30.0f+(step + min_d/MIN_DIST)));
    }
    // col = add_color(col, (float3){ 1.3, 0.8, 1.0 }, 6, 0.1/(0.1+min_d*0.2f));
    // float lum = 1.1;
    // float fade = 0.5;
    // float p = tot_d/MAX_DIST;
    // col*= lum;
    // if (p > fade) {
    //   col*= 1 - (p - fade);
    // }
    // col = normalize(col);
    col = clamp(col, 0, 1);
    // if (length(col) > 1) {
    // }
    ret_tex[gid] = (uchar4) {
      col[0] * 255,
      col[1] * 255,
      col[2] * 255,
      255,
    };
  }
}
