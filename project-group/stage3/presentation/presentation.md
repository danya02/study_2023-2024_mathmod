---
## Front matter
lang: ru-RU
title: Моделирование образования планетной системы 
subtitle: Этап 3
author:
  - Матюхин Г. В.
  - Генералов Д. М.
institute:
  - Российский университет дружбы народов, Москва, Россия
date: 9 марта 2024

## i18n babel
babel-lang: russian
babel-otherlangs: english

## Formatting pdf
toc: false
toc-title: Содержание
slide_level: 2
aspectratio: 169
section-titles: true
theme: metropolis
header-includes:
 - \metroset{progressbar=frametitle,sectionpage=progressbar,numbering=fraction}
 - \usepackage{fvextra}
 - \DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,commandchars=\\\{\}}
 - '\makeatletter'
 - '\beamer@ignorenonframefalse'
 - '\makeatother'
---

# Цель работы

Провести моделирование одного из этапов эволюции Вселенной -- образование некой "солнечной" системы из межзвездного газа.

# Выполнение

## Репозиторий

https://github.com/danya02/study_2023-2024_mathmod/tree/master/project-group/stage3

# Julia

## Частица

```julia
mutable struct Particle
    position
    velocity
    mass::Float64
    radius::Float64
end
```

## Основной цикл

```julia
particles = [get_random_particle(10, 1) for _ in 1:1000] #1
for i in 1:100
    apply_particle_forces(particles, dt) #2
end
```

## Генерация частиц

\footnotesize
```julia
function get_random_particle(radius_max, angular_speed)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    Particle([x,y], [vx,vy], Random.rand() * max_mass, Random.rand() * max_radius)
end
```

## Расчет сил

\footnotesize
```julia
for src_part in particles
    force = [0.0, 0.0]
    for dst_part in particles
        if src_part === dst_part
            continue
        end
        extra_force = normalize(src_part.position - dst_part.position) * universal_gravitation_const * (src_part.mass * dst_part.mass) / magnitude(src_part.position - dst_part.position)
        force += extra_force
    end
end
```

## Движение под воздействием сил

```julia
src_part.velocity += force * dt
src_part.position += src_part.velocity * dt
```

## Результаты


| Количесто частиц | Количество шагов | Время  |
|------------------|------------------|--------|
| 1000             | 100              | 80с    |
| 5000             | 50               | 14м47с |

# Rust && Julia

## Частица

```julia
struct Vec2
    x::Float64
    y::Float64
end

struct Particle
    id::UInt64
    position::Vec2
    velocity::Vec2
    mass::Float64
    radius::Float64
end
```

## Генерация частиц

\footnotesize
```julia
function get_random_particle(radius_max, angular_speed, i)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    mass = Random.rand() * max_part_mass
    density = Random.rand() * (max_density - min_density) + min_density
    volume = mass / density
    radius = cbrt((3/4pi) * volume)
    Particle(i, Vec2(x,y), Vec2(vx,vy), mass, radius)
end
```

## Основной цикл

\footnotesize
```julia
function perform_timesteps(particles::Vector{Particle}, steps)
    # skipped: create and initialize the array with default values

    sym = Libdl.dlsym(LIB, :perform_timesteps)
    @ccall $sym(step_array::Ptr{Ptr{Particle}}, particle_count::Csize_t, step_count::Csize_t)::Cvoid

    # # save simulation results
    # f = open(io -> write(io, JSON.json(step_array)), "output.json", write=true)
    step_array
end
```

## Симуляция

\footnotesize
```rust
pub extern "C" fn perform_timesteps(data: *mut *mut Particle, particle_count: usize, step_count: usize) {
    let dt = 0.001; let mut living_particles = particle_count;
    for src_count in 0..step_count - 1 {
        let [src, dst] = &mut particle_slice_list[src_count..=src_count + 1];
        dst.copy_from_slice(src);

        dst[..living_particles].chunks_mut(10).for_each(|dst| {
            apply_particle_forces(&src[..living_particles], dst, dt);
        });

        let glued_particles = run_glue(dst);
        if glued_particles > 0 {
            sort_zeroed(dst);
            living_particles -= glued_particles;
        }
    }
}
```

## Расчет сил и движение частиц

\footnotesize
```rust
pub fn apply_particle_forces( all_src_particles: &[Particle], my_particles: &mut [Particle], dt: f64) {
    for src in my_particles.iter_mut() {
        let mut force = Vec2::new(0.0, 0.0);
        for dst in all_src_particles.iter() {
            let extra_force = (dst.position - src.position).normalize()
                * UNIVERSAL_GRAVITATION
                * ((src.mass * dst.mass) / (src.position - dst.position).magnitude());
            force += extra_force;
        }
        src.velocity += force * dt;
        src.position += src.velocity * dt;
    }
}
```

## Слипание частиц

\footnotesize
```rust
pub fn glue_to_other(&mut self, other: &mut Self) {
    self.position = ((self.position * self.mass) + (other.position * other.mass))
        / (self.mass + other.mass);
    self.velocity = ((self.velocity * self.mass) + (other.velocity * other.mass))
        / (self.mass + other.mass);
    self.mass += other.mass;
    self.radius = f64::cbrt(self.radius.powi(3) + other.radius.powi(3));
    other.velocity = Vec2::new(0.0, 0.0);
    other.radius = 0.0;
    other.mass = 0.0;
}
```

# Сравнение с Julia

| Количесто частиц | Количество шагов | Julia  | Julia+Rust  |
|------------------|------------------|--------|-------|
| 1000             | 100              | 80с    | 5.6c  |
| 5000             | 50               | 14м47с | 17.6c |

# Визуализация

\footnotesize
```rust
fn rdp(points: &[TimeSeriesPoint], epsilon: f64, result: &mut Vec<TimeSeriesPoint>) {
    let n = points.len(); let mut max_dist = 0.0; let mut index = 0;
    for i in 1..n - 1 {
        let dist = perpendicular_distance(&points[i], &points[0], &points[n - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        rdp(&points[0..=index], epsilon, result);
        rdp(&points[index..n], epsilon, result);
    } else {
        result.push(points[n - 1]);
    }
}
```

# Выводы

На этом этапе проекта мы смогли сделать программу, которая считает динамику системы частиц достаточно быстро, чтобы можно было попробовать различные варианты начальных конфигураций и сделать анализ свойств солнечной системы. Это потребует отслеживания метрик симуляции (вроде общей энергии системы) и визуализаций с помощью Blender.